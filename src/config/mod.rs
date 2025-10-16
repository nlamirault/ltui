use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub refresh_interval: u64,
    pub default_team_id: Option<String>,
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            refresh_interval: 30,
            default_team_id: None,
            theme: ThemeConfig::default(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            primary_color: "blue".to_string(),
            secondary_color: "cyan".to_string(),
            background_color: "black".to_string(),
            text_color: "white".to_string(),
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("ltui");

        std::fs::create_dir_all(&config_dir).context("Could not create config directory")?;

        let config_file = if let Some(path) = config_path {
            Path::new(path).to_path_buf()
        } else {
            config_dir.join("config.toml")
        };

        if config_file.exists() {
            let config_str =
                std::fs::read_to_string(&config_file).context("Could not read config file")?;

            let mut config: Config =
                toml::from_str(&config_str).context("Could not parse config file")?;

            // Override with environment variable if present
            if let Ok(token) = std::env::var("LINEAR_API_KEY") {
                config.api_key = Some(token);
            }

            Ok(config)
        } else {
            let config = Config::default();
            let config_str =
                toml::to_string_pretty(&config).context("Could not serialize default config")?;

            std::fs::write(&config_file, config_str)
                .context("Could not write default config file")?;

            Ok(config)
        }
    }
}
