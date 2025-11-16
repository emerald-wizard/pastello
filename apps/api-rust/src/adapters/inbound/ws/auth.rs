use anyhow::Result;
use async_trait::async_trait;
use jsonwebtoken::{
    // --- FIX: Import Algorithm from root, not private jwk module ---
    // --- FIX: Removed unused Jwk import ---
    decode, decode_header, jwk::{JwkSet}, Algorithm,
    DecodingKey, Header, Validation,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
// --- FIX: Removed unused Arc ---
// use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[async_trait]
pub trait TokenValidator: Send + Sync {
    async fn validate(&self, token: &str) -> Result<GameTicketClaims, AuthError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameTicketClaims {
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub sid: String, // Session ID
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("JWT error: {0}")]
    JwtError(String),
    #[error("JWKS fetch error: {0}")]
    JwksFetchError(String),
    #[error("JWKS key not found: {0}")]
    JwksKeyNotFound(String),
    #[error("Unsupported JWT algorithm")]
    UnsupportedAlgorithm,
}

// --- FIX: Remove #[derive(Debug)] ---
pub struct RemoteJwkValidator {
    jwks_url: String,
    keys: RwLock<HashMap<String, DecodingKey>>,
    client: Client,
}

// --- FIX: Implement Debug manually ---
impl std::fmt::Debug for RemoteJwkValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteJwkValidator")
            .field("jwks_url", &self.jwks_url)
            .field("client", &self.client)
            .field("keys", &"RwLock<HashMap<...>>") // Don't print the non-Debug keys
            .finish()
    }
}

impl RemoteJwkValidator {
    pub fn new(jwks_url: String) -> Self {
        Self {
            jwks_url,
            keys: RwLock::new(HashMap::new()),
            client: Client::new(),
        }
    }

    async fn find_decoding_key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        // 1. Check cache
        if let Some(key) = self.keys.read().await.get(kid) {
            return Ok(key.clone());
        }

        // 2. Fetch from URL if not in cache
        let jwkset = self.fetch_jwks().await?;
        let jwk = jwkset.find(kid);

        let jwk = jwk.ok_or_else(|| AuthError::JwksKeyNotFound(kid.to_string()))?;

        // --- FIX: Match on jwk.common.algorithm (string) not jwk.algorithm (enum) ---
        match jwk.common.algorithm.as_deref() {
            Some("RS256") => {
                // --- FIX: Use DecodingKey::from_jwk to convert ---
                let key = DecodingKey::from_jwk(jwk)
                    .map_err(|e| AuthError::JwtError(e.to_string()))?;
                
                // 3. Cache the key
                self.keys.write().await.insert(kid.to_string(), key.clone());
                Ok(key)
            }
            _ => {
                // Algorithm not supported
                Err(AuthError::UnsupportedAlgorithm)
            }
        }
    }

    async fn fetch_jwks(&self) -> Result<JwkSet, AuthError> {
        self.client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| AuthError::JwksFetchError(e.to_string()))?
            .json::<JwkSet>()
            .await
            .map_err(|e| AuthError::JwksFetchError(e.to_string()))
    }

    fn create_validation(&self) -> Validation {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.validate_nbf = false;
        
        // --- FIX: Convert &str to String for HashSet ---
        validation.required_spec_claims = ["exp", "sub", "sid", "aud", "iss"]
            .iter()
            .map(|s| s.to_string())
            .collect();
            
        validation.aud = Some(HashSet::from(["pastello-game".to_string()]));
        validation.iss = Some(HashSet::from(["pastello-auth".to_string()]));
        validation
    }
}

#[async_trait]
impl TokenValidator for RemoteJwkValidator {
    async fn validate(&self, token: &str) -> Result<GameTicketClaims, AuthError> {
        let header: Header =
            decode_header(token).map_err(|e| AuthError::JwtError(e.to_string()))?;

        let kid = header.kid.ok_or_else(|| AuthError::JwtError("Missing 'kid' in token header".to_string()))?;

        let decoding_key = self.find_decoding_key(&kid).await?;

        let validation = self.create_validation();

        let token_data = decode::<GameTicketClaims>(token, &decoding_key, &validation)
            .map_err(|e| AuthError::JwtError(e.to_string()))?;

        Ok(token_data.claims)
    }
}