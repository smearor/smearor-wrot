use crate::cli::application::ApplicationArguments;
use crate::config_file::child_process::ChildProcessConfigFile;
use crate::config_file::color_mask::ColorMaskConfigFile;
use crate::config_file::compositor::CompositorConfigFile;
use crate::config_file::debug_overlay::DebugOverlayConfigFile;
use crate::config_file::env_vars::EnvironmentVariablesConfigFile;
use crate::config_file::error::ConfigError;
use crate::config_file::gtk_application::GtkApplicationConfigFile;
use crate::config_file::keyboard::KeyboardConfigFile;
use crate::config_file::layer::LayerConfigFile;
use crate::config_file::merge::MergeWithConfigFile;
use crate::config_file::rotation::RotationConfigFile;
use crate::config_file::window::WindowConfigFile;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Configuration file structure (TOML format)
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ApplicationConfigFile {
    /// Configuration for child process
    #[serde(default)]
    pub child_process: Option<ChildProcessConfigFile>,

    /// Configuration for the color mask
    #[serde(default)]
    pub color_mask: Option<ColorMaskConfigFile>,

    /// Configuration for the compositor
    #[serde(default)]
    pub compositor: Option<CompositorConfigFile>,

    /// Configuration for the debug overlay
    #[serde(default)]
    pub debug_overlay: Option<DebugOverlayConfigFile>,

    /// Configuration for the environment variables
    #[serde(default)]
    pub env_vars: Option<EnvironmentVariablesConfigFile>,

    /// Configuration for the GTK4 application window
    #[serde(default)]
    pub gtk_application: Option<GtkApplicationConfigFile>,

    /// Configuration for the keyboard
    #[serde(default)]
    pub keyboard: Option<KeyboardConfigFile>,

    /// Configuration for the layer
    #[serde(default)]
    pub layer: Option<LayerConfigFile>,

    /// Configuration for the rotation
    #[serde(default)]
    pub rotation: Option<RotationConfigFile>,

    /// Configuration for the GTK4 application window
    #[serde(default)]
    pub window: Option<WindowConfigFile>,
}

impl ApplicationConfigFile {
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

impl MergeWithConfigFile<ApplicationConfigFile> for ApplicationArguments {
    fn merge_with_config_file(mut self, config: &ApplicationConfigFile) -> Self {
        let mut args = self;
        if let Some(child_process) = config.child_process {
            args.child_process = args.child_process.merge_with_config_file(&child_process);
        }
        if let Some(color_mask) = config.color_mask {
            args.color_mask = args.color_mask.merge_with_config_file(&color_mask);
        }
        if let Some(compositor) = config.compositor {
            args.compositor = args.compositor.merge_with_config_file(&compositor);
        }
        if let Some(debug_overlay) = config.debug_overlay {
            args.debug_overlay = args.debug_overlay.merge_with_config_file(&debug_overlay);
        }
        if let Some(env_vars) = config.env_vars {
            args.env_vars = args.env_vars.merge_with_config_file(&env_vars);
        }
        if let Some(gtk_application) = config.gtk_application {
            args.gtk_application = args.gtk_application.merge_with_config_file(&gtk_application);
        }
        if let Some(keyboard) = config.keyboard {
            args.keyboard = args.keyboard.merge_with_config_file(&keyboard);
        }
        if let Some(layer) = config.layer {
            args.layer = args.layer.merge_with_config_file(&layer);
        }
        if let Some(rotation) = config.rotation {
            args.rotation = args.rotation.merge_with_config_file(&rotation);
        }
        if let Some(window) = config.window {
            args.window = args.window.merge_with_config_file(&window);
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ApplicationConfigFile::default();
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
        let config: ApplicationConfigFile = toml::from_str(toml_content).unwrap();
        assert_eq!(config.window.title, Some("Test Window".to_string()));
        assert_eq!(config.window.width, Some(800));
        assert_eq!(config.window.height, Some(600));
    }

    #[test]
    fn test_load_config_empty() {
        let toml_content = "";
        let config: ApplicationConfigFile = toml::from_str(toml_content).unwrap();
        assert!(config.window.title.is_none());
    }
}
