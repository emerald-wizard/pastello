use crate::config::AuthConfig;
use jsonwebtoken::{
    decode, decode_header, DecodingKey, Validation,
    Algorithm, jwk::{Jwk, JwkSet},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use anyhow::{Result, anyhow};

/// This is the "Game Ticket" JWT claims structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameTicketClaims {
    pub sub: String, // Subject (user_id)
    pub sid: String, // Session ID (game_session_id)
    pub aud: String, // Audience
    pub iss: String, // Issuer
    pub exp: usize,  // Expiry
}

/// A thread-safe, caching JWT validator.
#[derive(Debug, Clone)]
pub struct TokenValidator {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    config: AuthConfig,
    keys: RwLock<HashMap<String, DecodingKey>>,
}

impl TokenValidator {
    /// Creates a new validator *without* fetching keys.
    /// Used for dev/testing.
    pub fn new_without_refresh(config: AuthConfig) -> Self {
        let inner = Arc::new(Inner {
            config,
            keys: RwLock::new(HashMap::new()),
        });
        Self { inner }
    }

    pub async fn new(config: AuthConfig) -> Result<Self> {
        let inner = Arc::new(Inner {
            config,
            keys: RwLock::new(HashMap::new()),
        });
        
        let validator = Self { inner };
        validator.refresh_keys().await?;
        Ok(validator)
    }

    pub async fn refresh_keys(&self) -> Result<()> {
        info!(url = %self.inner.config.jwks_url, "Fetching JWKS");
        let jwks: JwkSet = reqwest::get(&self.inner.config.jwks_url)
            .await?
            .json()
            .await?;

        let mut keys = self.inner.keys.write().await;
        keys.clear();

        for jwk in jwks.keys {
            if let Some(kid) = jwk.common.key_id.clone() {
                match jwk.algorithm {
                    jsonwebtoken::jwk::Algorithm::RS256 => {
                        let key = DecodingKey::from_jwk(&jwk)?;
                        keys.insert(kid, key);
                    }
                    _ => {
                        warn!(kid = %kid, "Skipping JWK with unsupported algorithm");
                    }
                }
            }
        }
        info!("JWKS cache refreshed. {} keys loaded.", keys.len());
        Ok(())
    }

    pub async fn validate_token(&self, token_str: &str) -> Result<GameTicketClaims> {
        let header = decode_header(token_str)?;
        let kid = header.kid.ok_or_else(|| anyhow!("token has no 'kid' (Key ID)"))?;

        let key = {
            let keys = self.inner.keys.read().await;
            keys.get(&kid).cloned()
        };

        let decoding_key = match key {
            Some(k) => k,
            None => {
                warn!(kid = %kid, "Unknown 'kid'. Refreshing JWKS cache...");
                self.refresh_keys().await?;
                let keys = self.inner.keys.read().await;
                keys.get(&kid)
                    .cloned()
                    .ok_or_else(|| anyhow!("'kid' not found even after JWKS refresh"))?
            }
        };
        
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.inner.config.audience.clone()]);
        validation.set_issuer(&[self.inner.config.issuer.clone()]);
        validation.leeway = 5;
        validation.required_spec_claims = HashSet::from(["exp", "sub", "sid", "aud", "iss"]);

        let token_data = decode::<GameTicketClaims>(token_str, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}