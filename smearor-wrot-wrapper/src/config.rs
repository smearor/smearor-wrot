//! Configuration file support for smearor-wrot-wrapper
//!
//! This module provides configuration file loading and merging with CLI arguments.

pub use smearor_wrot_model::config::CompositorConfig;
pub use smearor_wrot_model::config::Config;
pub use smearor_wrot_model::config::ConfigError;
pub use smearor_wrot_model::config::WindowConfig;

use std::fs;
use std::path::Path;

/// Load configuration from a TOML file
///
/// # Arguments
///
/// * `path` - Path to the configuration file
///
/// # Returns
///
/// * `Result<Config, ConfigError>` - The loaded configuration or an error
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path).map_err(|e| ConfigError::ReadError(e.to_string()))?;

    let config: Config = toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
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
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.window.title, Some("Test Window".to_string()));
        assert_eq!(config.window.width, Some(800));
        assert_eq!(config.window.height, Some(600));
    }

    #[test]
    fn test_load_config_empty() {
        let toml_content = "";
        let config: Config = toml::from_str(toml_content).unwrap();
        assert!(config.window.title.is_none());
    }
}
