use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwks_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbConfig {
    pub url: String,
}

// --- FIX: Add pub ---
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub db: DbConfig,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }
}

//use serde::Deserialize;
//use std::fs;
//use anyhow::{Context, Result};
//
//#[derive(Debug, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct AppConfig {
//    pub log_level: String,
//    pub db: DbConfig,
//    pub auth: AuthConfig,
//}
//
//#[derive(Debug, Deserialize, Clone)]
//pub struct DbConfig {
//    pub host: String,
//    pub port: u16,
//    pub user: String,
//    pub password: String,
//    pub name: String,
//}
//
//#[derive(Debug, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct AuthConfig {
//    pub jwks_url: String,      // e.g., "http://localhost:8081/.well-known/jwks.json"
//    pub audience: String,      // e.g., "pastello-game-room"
//    pub issuer: String,        // e.g., "pastello-lobby-api"
//}
//
//pub fn load_config(path: &str) -> Result<AppConfig> {
//    let data = fs::read_to_string(path)
//        .with_context(|| format!("Failed to read config file at: {}", path))?;
//    
//    let cfg: AppConfig = serde_yaml::from_str(&data)
//        .with_context(|| "Failed to unmarshal config YAML")?;
//    
//    Ok(cfg)
//}