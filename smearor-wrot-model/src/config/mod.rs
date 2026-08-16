//! Configuration file structures for TOML-based configuration.
//!
//! This module contains all configuration types used across the workspace:
//! - `DebugOverlayConfig`: debug overlay settings (always available)
//! - `Config`, `WindowConfig`, `CompositorConfig`: TOML config structs (serde feature)
//! - `ConfigError`: error type for config loading

pub mod debug_overlay;

pub use debug_overlay::DebugOverlayConfig;

#[cfg(feature = "serde")]
use serde::Deserialize;

/// Root configuration file structure (TOML format)
#[cfg(feature = "serde")]
#[derive(Debug, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct Config {
    /// Window configuration section
    pub window: WindowConfig,

    /// Compositor configuration section
    pub compositor: CompositorConfig,
}

/// Window configuration section
#[cfg(feature = "serde")]
#[derive(Debug, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct WindowConfig {
    /// Window title
    pub title: Option<String>,

    /// Initial width
    pub width: Option<i32>,

    /// Initial height
    pub height: Option<i32>,

    /// Whether the window should have decorations
    pub decorated: Option<bool>,

    /// Whether the window should be resizable
    pub resizable: Option<bool>,

    /// Initial x position
    pub position_x: Option<i32>,

    /// Initial y position
    pub position_y: Option<i32>,

    /// Minimum width
    pub min_width: Option<i32>,

    /// Minimum height
    pub min_height: Option<i32>,

    /// Maximum width
    pub max_width: Option<i32>,

    /// Maximum height
    pub max_height: Option<i32>,

    /// Aspect ratio
    pub aspect_ratio: Option<f32>,

    /// Start in fullscreen mode
    pub fullscreen: Option<bool>,

    /// Start in maximized mode
    pub maximized: Option<bool>,
}

/// Compositor configuration section
#[cfg(feature = "serde")]
#[derive(Debug, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct CompositorConfig {
    /// Enable double buffering
    pub double_buffer: Option<bool>,

    /// Disable rotation widget
    pub disable_rotation: Option<bool>,

    /// Rotation angle in degrees
    pub rotation: Option<f32>,

    /// Wayland socket path
    pub socket: Option<String>,

    /// Layer shell protocol layer
    pub layer: Option<String>,

    /// Layer shell namespace
    pub namespace: Option<String>,

    /// Run command in shell
    pub shell: Option<bool>,

    /// Disable DMA-BUF hardware acceleration
    pub disable_dma_buf: Option<bool>,

    /// Disable client-side decorations for Wayland clients
    pub disable_client_decorations: Option<bool>,

    /// Left margin in pixels for window rendering
    pub margin_left: Option<u32>,

    /// Right margin in pixels for window rendering
    pub margin_right: Option<u32>,

    /// Top margin in pixels for window rendering
    pub margin_top: Option<u32>,

    /// Bottom margin in pixels for window rendering
    pub margin_bottom: Option<u32>,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque)
    pub opacity: Option<f32>,

    /// Background color in hex format (e.g., "#FF0000" for red)
    pub background_color: Option<String>,

    /// Window opacity for the compositor window (0.0 = fully transparent, 1.0 = fully opaque)
    pub window_opacity: Option<f32>,

    /// Maximum frames per second (default: 60)
    pub max_fps: Option<i64>,

    /// Dialog margin in pixels for dialog positioning (default: 0)
    pub dialog_margin: Option<u32>,

    /// Enable shader-based color masking for better performance (default: false)
    pub color_mask_shader: Option<bool>,

    /// Disable all animations (default: false)
    pub disable_animations: Option<bool>,
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

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.window.title.is_none());
        assert!(config.window.width.is_none());
    }

    #[test]
    fn test_config_deserialize_valid_toml() {
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
    fn test_config_deserialize_empty() {
        let toml_content = "";
        let config: Config = toml::from_str(toml_content).unwrap();
        assert!(config.window.title.is_none());
    }

    #[test]
    fn test_config_deserialize_partial_compositor() {
        let toml_content = r#"
[compositor]
opacity = 0.5
max_fps = 30
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.compositor.opacity, Some(0.5));
        assert_eq!(config.compositor.max_fps, Some(30));
        assert!(config.compositor.disable_dma_buf.is_none());
    }
}
