use crate::cli::child_process::ChildProcessArguments;
use crate::cli::child_process::ChildProcessConfigError;
use crate::cli::color_mask::ColorMaskArguments;
use crate::cli::compositor::CompositorArguments;
use crate::cli::debug_overlay::DebugOverlayArguments;
use crate::cli::env_vars::EnvironmentVariablesArguments;
use crate::cli::gtk_application::GtkApplicationArguments;
use crate::cli::keyboard::KeyboardArguments;
use crate::cli::layer::LayerArguments;
use crate::cli::rotation::RotationArguments;
use crate::cli::window::WindowArguments;
use crate::config_file::application::ApplicationConfigFile;
use crate::config_file::merge::MergeWithConfigFile;
use clap::Parser;
use smearor_wrot_application::ApplicationConfig;
use smearor_wrot_application::ChildProcessConfig;
use smearor_wrot_application::ColorMaskConfig;
use smearor_wrot_application::CompositorConfig;
use smearor_wrot_application::DEFAULT_WINDOW_HEIGHT;
use smearor_wrot_application::DEFAULT_WINDOW_WIDTH;
use smearor_wrot_application::DebugOverlayConfig;
use smearor_wrot_application::EnvironmentVariablesConfig;
use smearor_wrot_application::GtkApplicationConfig;
use smearor_wrot_application::KeyboardConfig;
use smearor_wrot_application::LayerConfig;
use smearor_wrot_application::RotationConfig;
use smearor_wrot_application::SmearorLayer;
use smearor_wrot_application::SocketBuilderError;
use smearor_wrot_application::WindowConfig;
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tracing::debug;

/// Smearor Window Rotation Compositor
///
/// A Wayland window rotation system designed for multi-user collaborative smart desks, enabling
/// individual window rotation without rotating the entire screen.
///
/// ## Overview
///
/// **smearor-wrot** solves the orientation problem on large touchscreen smart desks where users
/// sit at different sides of the table. When users sit opposite each other, one person sees the
/// content upside down. smearor-wrot allows individual window rotation so multiple users can
/// interact with applications oriented toward their position.
///
/// ### Key Features
///
/// - **Individual Window Rotation**: Rotate any Wayland application window by any angle
/// - **Input Transformation**: Mouse and touch input coordinates are automatically transformed according to window rotation
/// - **Cross-Desktop Compatibility**: Works with Hyprland, Sway, GNOME, and other Wayland compositors
/// - **High Performance**: Maintains 60 FPS rendering with hardware acceleration support
/// - **Touch Support**: Full touch input support for smart desk surfaces
/// - **Multi-Window**: Support for multiple rotated windows simultaneously
///
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct ApplicationArguments {
    /// Path to the configuration file (TOML format).
    #[arg(short = 'c', long)]
    pub(crate) config: Option<PathBuf>,

    #[command(flatten)]
    pub(crate) child_process: ChildProcessArguments,

    #[command(flatten)]
    pub(crate) color_mask: ColorMaskArguments,

    #[command(flatten)]
    pub(crate) compositor: CompositorArguments,

    #[command(flatten)]
    pub(crate) debug_overlay: DebugOverlayArguments,

    #[command(flatten)]
    pub(crate) env_vars: EnvironmentVariablesArguments,

    #[command(flatten)]
    pub(crate) gtk_application: GtkApplicationArguments,

    #[command(flatten)]
    pub(crate) keyboard: KeyboardArguments,

    #[command(flatten)]
    pub(crate) layer: LayerArguments,

    #[command(flatten)]
    pub(crate) rotation: RotationArguments,

    #[command(flatten)]
    pub(crate) window: WindowArguments,
}

impl ApplicationArguments {
    pub(crate) fn load_and_merge_config(self) -> Result<Self, Box<dyn Error>> {
        let Some(config_path) = self.config.clone() else {
            return Ok(self);
        };
        match ApplicationConfigFile::load_config(&config_path) {
            Ok(config) => {
                debug!("Loaded configuration from: {config_path:?}");
                Ok(self.merge_with_config_file(&config))
            }
            Err(e) => {
                eprintln!("Failed to load configuration file: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ApplicationConfigError {
    #[error(transparent)]
    ChildProcessConfigError(#[from] ChildProcessConfigError),
}

impl TryFrom<ApplicationArguments> for ApplicationConfig {
    type Error = ApplicationConfigError;
    fn try_from(args: ApplicationArguments) -> Result<Self, Self::Error> {
        let child_process_config = ChildProcessConfig::try_from(args.child_process)?;
        Ok(ApplicationConfig::builder()
            .child_process(Arc::new(child_process_config))
            .color_mask(Arc::new(ColorMaskConfig::from(args.color_mask)))
            .compositor(Arc::new(CompositorConfig::from(args.compositor)))
            .debug_overlay(Arc::new(DebugOverlayConfig::from(args.debug_overlay)))
            .env_vars(Arc::new(EnvironmentVariablesConfig::from(args.env_vars)))
            .gtk_application(Arc::new(GtkApplicationConfig::from(args.gtk_application)))
            .keyboard(Arc::new(KeyboardConfig::from(args.keyboard)))
            .layer(Arc::new(LayerConfig::from(args.layer)))
            .rotation(Arc::new(RotationConfig::from(args.rotation)))
            .window(Arc::new(WindowConfig::from(args.window)))
            .build())
    }
}
