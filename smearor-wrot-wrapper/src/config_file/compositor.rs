use serde::Deserialize;

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
    pub max_fps: Option<i64>,

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
