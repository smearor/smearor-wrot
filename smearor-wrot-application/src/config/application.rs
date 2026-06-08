use crate::config::compositor::CompositorConfig;
use crate::config::env_vars::EnvironmentVariablesConfig;
use crate::config::gtk_application::GtkApplicationConfig;
use crate::config::rotation::RotationConfig;
use smearor_wrot_child_process::ChildProcessConfig;
use smearor_wrot_color_mask::ColorMaskConfig;
use smearor_wrot_debug_overlay::DebugOverlayConfig;
use smearor_wrot_keyboard::KeyboardConfig;
use smearor_wrot_layer::LayerConfig;
use smearor_wrot_margin::MarginConfig;
use smearor_wrot_window::WindowConfig;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct ApplicationConfig {
    /// Configuration for child process
    pub child_process: Arc<ChildProcessConfig>,

    /// Configuration for the color masking
    pub color_mask: Arc<ColorMaskConfig>,

    /// Configuration for the compositor
    pub compositor: Arc<CompositorConfig>,

    /// Configuration for the debug overlay
    pub debug_overlay: Arc<DebugOverlayConfig>,

    /// Configuration for the application environment variables
    pub env_vars: Arc<EnvironmentVariablesConfig>,

    /// Configuration for the GTK application
    pub gtk_application: Arc<GtkApplicationConfig>,

    /// Configuration for the keyboard
    pub keyboard: Arc<KeyboardConfig>,

    /// Configuration for the wayland layer shell
    pub layer: Arc<LayerConfig>,

    /// Configuration for the margins
    pub margin: Arc<MarginConfig>,

    /// Configuration for the rotation
    pub rotation: Arc<RotationConfig>,

    /// Configuration for the GTK application window
    pub window: Arc<WindowConfig>,
}
