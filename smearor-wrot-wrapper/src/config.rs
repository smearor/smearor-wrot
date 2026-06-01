//! Configuration file support for smearor-wrot-wrapper
//!
//! This module provides configuration file loading and merging with CLI arguments.

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuration file structure (TOML format)
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// Window configuration
    #[serde(default)]
    pub window: WindowConfig,

    /// Compositor configuration
    #[serde(default)]
    pub compositor: CompositorConfig,
}

/// Window configuration section
#[derive(Debug, Deserialize, Default)]
pub struct WindowConfig {
    /// Window title
    #[serde(default)]
    pub title: Option<String>,

    /// Initial width
    #[serde(default)]
    pub width: Option<i32>,

    /// Initial height
    #[serde(default)]
    pub height: Option<i32>,

    /// Whether the window should have decorations
    #[serde(default)]
    pub decorated: Option<bool>,

    /// Whether the window should be resizable
    #[serde(default)]
    pub resizable: Option<bool>,

    /// Initial x position
    #[serde(default)]
    pub position_x: Option<i32>,

    /// Initial y position
    #[serde(default)]
    pub position_y: Option<i32>,

    /// Minimum width
    #[serde(default)]
    pub min_width: Option<i32>,

    /// Minimum height
    #[serde(default)]
    pub min_height: Option<i32>,

    /// Maximum width
    #[serde(default)]
    pub max_width: Option<i32>,

    /// Maximum height
    #[serde(default)]
    pub max_height: Option<i32>,

    /// Aspect ratio
    #[serde(default)]
    pub aspect_ratio: Option<f32>,

    /// Start in fullscreen mode
    #[serde(default)]
    pub fullscreen: Option<bool>,

    /// Start in maximized mode
    #[serde(default)]
    pub maximized: Option<bool>,
}

/// Compositor configuration section
#[derive(Debug, Deserialize, Default)]
pub struct CompositorConfig {
    /// Enable double buffering
    #[serde(default)]
    pub double_buffer: Option<bool>,

    /// Disable rotation widget
    #[serde(default)]
    pub disable_rotation: Option<bool>,

    /// Rotation angle in degrees
    #[serde(default)]
    pub rotation: Option<f32>,

    /// Wayland socket path
    #[serde(default)]
    pub socket: Option<String>,

    /// Layer shell protocol layer
    #[serde(default)]
    pub layer: Option<String>,

    /// Layer shell namespace
    #[serde(default)]
    pub namespace: Option<String>,

    /// Run command in shell
    #[serde(default)]
    pub shell: Option<bool>,

    /// Disable DMA-BUF hardware acceleration
    #[serde(default)]
    pub disable_dma_buf: Option<bool>,

    /// Disable client-side decorations for Wayland clients
    #[serde(default)]
    pub disable_client_decorations: Option<bool>,

    /// Left margin in pixels for window rendering
    #[serde(default)]
    pub margin_left: Option<u32>,

    /// Right margin in pixels for window rendering
    #[serde(default)]
    pub margin_right: Option<u32>,

    /// Top margin in pixels for window rendering
    #[serde(default)]
    pub margin_top: Option<u32>,

    /// Bottom margin in pixels for window rendering
    #[serde(default)]
    pub margin_bottom: Option<u32>,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque)
    #[serde(default)]
    pub opacity: Option<f32>,

    /// Background color in hex format (e.g., "#FF0000" for red)
    #[serde(default)]
    pub background_color: Option<String>,

    /// Window opacity for the compositor window (0.0 = fully transparent, 1.0 = fully opaque)
    #[serde(default)]
    pub window_opacity: Option<f32>,

    /// Maximum frames per second (default: 60)
    #[serde(default)]
    pub max_fps: Option<u32>,

    /// Dialog margin in pixels for dialog positioning (default: 0)
    #[serde(default)]
    pub dialog_margin: Option<u32>,

    /// Enable shader-based color masking for better performance (default: false)
    #[serde(default)]
    pub color_mask_shader: Option<bool>,

    /// Disable all animations (default: false)
    #[serde(default)]
    pub disable_animations: Option<bool>,
}

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

/// Configuration error type
#[derive(Debug)]
pub enum ConfigError {
    /// Error reading configuration file
    ReadError(String),

    /// Error parsing configuration file
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ReadError(msg) => write!(f, "Failed to read config file: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse config file: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

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
