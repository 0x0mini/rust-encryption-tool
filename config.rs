use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use std::{
    path::PathBuf,
    fmt,
    error::Error,
};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub key_path: PathBuf,
    pub config_path: PathBuf,
    pub encryption_standard: String,
}

#[derive(Debug)]
enum SettingsError {
    Config(ConfigError),
    EnvVar(std::env::VarError),
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsError::Config(e) => write!(f, "Configuration error: {}", e),
            SettingsError::EnvVar(e) => write!(f, "Environment variable error: {}", e),
        }
    }
}

impl Error for SettingsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SettingsError::Config(e) => Some(e),
            SettingsError::EnvVar(e) => Some(e),
        }
    }
}

impl From<ConfigError> for SettingsError {
    fn from(e: ConfigError) -> Self {
        SettingsError::Config(e)
    }
}

impl From<std::env::VarError> for SettingsError {
    fn from(e: std::env::VarError) -> Self {
        SettingsError::EnvVar(e)
    }
}

impl Settings {
    pub fn new() -> Result<Self, SettingsError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default").required(false))?;
        let environment = std::env::var("RUN_MODE").map_err(SettingsError::from)?;
        s.merge(File::with_name(&format!("config/{}", environment)).required(false))?;
        s.merge(Environment::with_prefix("APP").separator("__"))?;
        s.try_into().map_err(SettingsError::from)
    }
}