use crate::config::env_vars::EnvironmentVariablesState;
use crate::config::gtk_application::GtkApplicationState;
use crate::config::rotation::RotationState;
use crate::config::state::CompositorState;
use smearor_wrot_child_process::ChildProcessState;
use smearor_wrot_color_mask::ColorMaskState;
use smearor_wrot_debug_overlay::DebugOverlayState;
use smearor_wrot_keyboard::KeyboardState;
use smearor_wrot_layer::LayerShellState;
use smearor_wrot_state_margin::MarginState;
use smearor_wrot_window::WindowState;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct ApplicationState {
    /// Child process state
    pub child_process: Arc<ChildProcessState>,

    /// Color masking state
    pub color_mask: Arc<ColorMaskState>,

    /// Compositor state
    pub compositor: Arc<CompositorState>,

    /// Debug overlay state
    pub debug_overlay: Arc<DebugOverlayState>,

    /// State of the environment variables of the parent process
    pub env_vars: Arc<EnvironmentVariablesState>,

    /// GTK application state
    pub gtk_application: Arc<GtkApplicationState>,

    /// Keyboard state
    pub keyboard: Arc<KeyboardState>,

    /// Wayland layer shell state
    pub layer: Arc<LayerShellState>,

    /// Margins state
    pub margin: Arc<MarginState>,

    /// Rotation state
    pub rotation: Arc<RotationState>,

    /// State of the GTK application window
    pub window: Arc<WindowState>,
}
