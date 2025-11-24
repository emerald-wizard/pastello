use crate::domain::game::Session;
use anyhow::Result;
use async_trait::async_trait;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;

// --- DEFINITIONS ---

// FIX: Define AuthError at the top of the module to resolve E0432, E0412, E0433
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("No token provided")]
    NoToken,
    #[error("Auth timed out")]
    Timeout,
    #[error("Network error: {0}")]
    Network(String),
    #[error("JWK error: {0}")]
    Jwk(String),
    #[error("No matching key in JWKS")]
    NoMatchingKey,
    #[error("Unsupported JWT algorithm")]
    UnsupportedAlgorithm,
    #[error("Token KID not found")]
    NoTokenKid,
    #[error("JWT error: {0}")]
    Jwt(String),
}

// FIX: AuthError is now in scope for the trait
#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<Session, AuthError>;
}

pub struct FirebaseAuthenticator {
    jwks_url: String,
    project_id: String,
    keys: RwLock<HashMap<String, DecodingKey>>,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirebaseClaims {
    aud: String,
    iss: String,
    sub: String, // This is the user_id
    exp: usize,
    iat: usize,
    auth_time: usize,
    // ... other fields like email, etc., if needed
}

// --- IMPLEMENTATIONS ---

impl FirebaseAuthenticator {
    pub fn new(project_id: &str) -> Self {
        Self {
            jwks_url: "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com".to_string(),
            project_id: project_id.to_string(),
            keys: RwLock::new(HashMap::new()),
            client: Client::new(),
        }
    }

    async fn get_decoding_key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        // Check cache first
        if let Some(key) = self.keys.read().await.get(kid) {
            return Ok(key.clone());
        }

        // Fetch JWKS
        info!("Fetching JWKS from {}", self.jwks_url);
        let jwks = self
            .client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| AuthError::Network(e.to_string()))?
            .json::<JwkSet>()
            .await
            .map_err(|e| AuthError::Jwk(e.to_string()))?;

        // Find and process the key
        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.common.key_id.as_deref() == Some(kid))
            .ok_or(AuthError::NoMatchingKey)?;

        // FIX: Use DecodingKey::from_jwk which correctly handles RS256 components (E0599 fix)
        if jwk.common.key_algorithm.as_ref().map(|a| a.to_string()) == Some("RS256".to_string()) {
            let decoding_key = DecodingKey::from_jwk(jwk)
                .map_err(|e| AuthError::Jwk(format!("JWK conversion error: {}", e.to_string())))?;

            // Cache the key
            self.keys
                .write()
                .await
                .insert(kid.to_string(), decoding_key.clone());
            Ok(decoding_key)
        } else {
            Err(AuthError::UnsupportedAlgorithm)
        }
    }

    fn create_validation(&self) -> Validation {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation.iss = Some(HashSet::from([format!(
            "https://securetoken.google.com/{}",
            self.project_id
        )]));
        validation.aud = Some(HashSet::from([self.project_id.clone()]));
        validation
    }
}

#[async_trait]
impl Authenticator for FirebaseAuthenticator {
    async fn authenticate(&self, token: &str) -> Result<Session, AuthError> {
        let header = decode_header(token).map_err(|e| AuthError::Jwk(e.to_string()))?;
        let kid = header.kid.ok_or(AuthError::NoTokenKid)?;

        let decoding_key = self.get_decoding_key(&kid).await?;
        let validation = self.create_validation();

        let token_data = decode::<FirebaseClaims>(token, &decoding_key, &validation)
            .map_err(|e| AuthError::Jwt(e.to_string()))?;

        // Construct Session with authenticated user_id as host_id
        Ok(Session {
            host_id: token_data.claims.sub,
            id: "temp-session".to_string(),
            game_type: crate::domain::game::GameType::Puzzle, // Placeholder
            players: Vec::new(), // Placeholder
        })
    }
}

// ... NoOpAuthenticator for testing ...
pub struct NoOpAuthenticator;

#[async_trait]
impl Authenticator for NoOpAuthenticator {
    async fn authenticate(&self, token: &str) -> Result<Session, AuthError> {
        info!("Using NoOpAuthenticator, bypassing validation for token: {}", token);
        // Create a dummy session based on the token string for testing
        Ok(Session {
            host_id: format!("user_for_token_{}", token),
            id: "no-op-session".to_string(),
            game_type: crate::domain::game::GameType::Puzzle,
            players: Vec::new(),
        })
    }
}
