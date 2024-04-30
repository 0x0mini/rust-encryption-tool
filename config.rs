use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub key_path: PathBuf,
    pub config_path: PathBuf,
    pub encryption_standard: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default").required(false))?;
        let environment = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", environment)).required(false))?;
        s.merge(Environment::with_prefix("APP").separator("__"))?;
        s.try_into()
    }
}
