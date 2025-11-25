use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use thiserror::Error;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseConfig {
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub firebase: FirebaseConfig,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

// FIX: Ensure load_config is public
pub fn load_config() -> Result<Config, ConfigError> {
    // Assuming standard config path, update if different
    let config_path = "config.yaml"; 
    let contents = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}