use crate::config_file::compositor::CompositorConfig;
use crate::config_file::error::ConfigError;
use crate::config_file::window::WindowConfig;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuration file structure (TOML format)
#[derive(Debug, Deserialize, Default)]
pub struct SmearorWindowRotationConfig {
    /// Window configuration
    #[serde(default)]
    pub window: WindowConfig,

    /// Compositor configuration
    #[serde(default)]
    pub compositor: CompositorConfig,
}

impl SmearorWindowRotationConfig {
    /// Load configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// * `Result<Config, ConfigError>` - The loaded configuration or an error
    pub fn load_config(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(|e| ConfigError::ReadError(e.to_string()))?;
        let config: Self = toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = SmearorWindowRotationConfig::default();
        assert!(config.window.title.is_none());
        assert!(config.window.width.is_none());
    }

    #[test]
    fn test_load_config_valid_toml() {
        let toml_content = r#"
[window]
title = "Test Window"
width = 800
height = 600
"#;
        let config: SmearorWindowRotationConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.window.title, Some("Test Window".to_string()));
        assert_eq!(config.window.width, Some(800));
        assert_eq!(config.window.height, Some(600));
    }

    #[test]
    fn test_load_config_empty() {
        let toml_content = "";
        let config: SmearorWindowRotationConfig = toml::from_str(toml_content).unwrap();
        assert!(config.window.title.is_none());
    }
}
