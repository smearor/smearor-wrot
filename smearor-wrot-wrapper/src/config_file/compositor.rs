use crate::cli::compositor::CompositorArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

/// Compositor configuration section
#[derive(Debug, Deserialize, Default)]
pub struct CompositorConfigFile {
    /// Dialog margin in pixels for dialog positioning (default: 0)
    #[serde(default)]
    pub dialog_margin: Option<u32>,

    /// Enable or disable double buffering
    #[serde(default)]
    pub double_buffer: Option<bool>,

    /// Enable or disable DMA-BUF hardware acceleration
    #[serde(default)]
    pub dma_buf: Option<bool>,

    /// Enable or disable client-side decorations for Wayland clients
    #[serde(default)]
    pub client_decorations: Option<bool>,

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
}

impl MergeWithConfigFile<CompositorConfigFile> for CompositorArguments {
    fn merge_with_config_file(mut self, config: &CompositorConfigFile) -> Self {
        if self.dialog_margin == 0
            && let Some(dialog_margin) = config.dialog_margin
        {
            self.dialog_margin = dialog_margin;
        }
        if !self.disable_double_buffer
            && let Some(double_buffer) = config.double_buffer
        {
            self.disable_double_buffer = !double_buffer;
        }
        if !self.disable_dma_buf
            && let Some(dma_buf) = config.dma_buf
        {
            self.disable_dma_buf = !dma_buf;
        }
        if !self.disable_client_decorations
            && let Some(client_decorations) = config.client_decorations
        {
            self.disable_client_decorations = !client_decorations;
        }
        if self.margin_left == 0
            && let Some(margin_left) = config.margin_left
        {
            self.margin_left = margin_left;
        }
        if self.margin_right == 0
            && let Some(margin_right) = config.margin_right
        {
            self.margin_right = margin_right;
        }
        if self.margin_top == 0
            && let Some(margin_top) = config.margin_top
        {
            self.margin_top = margin_top;
        }
        if self.margin_bottom == 0
            && let Some(margin_bottom) = config.margin_bottom
        {
            self.margin_bottom = margin_bottom;
        }
        if self.opacity == 1.0
            && let Some(opacity) = config.opacity
        {
            self.opacity = opacity;
        }

        self
    }
}
