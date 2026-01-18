use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::{ProwlError, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub provider_key: Option<String>,
    #[serde(default)]
    pub application: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| ProwlError::Config(format!("Failed to read config file: {e}")))?;

        toml::from_str(&contents)
            .map_err(|e| ProwlError::Config(format!("Failed to parse config file: {e}")))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| ProwlError::Config(format!("Failed to serialize config: {e}")))?;

        std::fs::write(&config_path, contents)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        ProjectDirs::from("", "", "prowl")
            .map(|dirs| dirs.config_dir().join("config.toml"))
            .ok_or_else(|| ProwlError::Config("Could not determine config directory".to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub api_key: Option<String>,
    pub provider_key: Option<String>,
    pub application: String,
}

impl ResolvedConfig {
    pub fn resolve(
        cli_api_key: Option<&str>,
        cli_provider_key: Option<&str>,
        cli_application: Option<&str>,
    ) -> Result<Self> {
        let file_config = Config::load().unwrap_or_default();

        let api_key = cli_api_key
            .map(String::from)
            .or_else(|| std::env::var("PROWL_API_KEY").ok())
            .or(file_config.api_key);

        let provider_key = cli_provider_key
            .map(String::from)
            .or_else(|| std::env::var("PROWL_PROVIDER_KEY").ok())
            .or(file_config.provider_key);

        let application = cli_application
            .map(String::from)
            .or_else(|| std::env::var("PROWL_APPLICATION").ok())
            .or(file_config.application)
            .unwrap_or_else(|| "prowl-cli".to_string());

        Ok(ResolvedConfig {
            api_key,
            provider_key,
            application,
        })
    }

    pub fn require_api_key(&self) -> Result<&str> {
        self.api_key.as_deref().ok_or(ProwlError::MissingApiKey)
    }

    pub fn require_provider_key(&self) -> Result<&str> {
        self.provider_key
            .as_deref()
            .ok_or(ProwlError::MissingProviderKey)
    }
}
