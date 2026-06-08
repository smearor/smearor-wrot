use crate::cli::compositor::CompositorArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

/// Compositor configuration section
#[derive(Debug, Deserialize, Default)]
pub struct CompositorConfigFile {
    /// Enable or disable double buffering
    #[serde(default)]
    pub double_buffer: Option<bool>,

    /// Enable or disable DMA-BUF hardware acceleration
    #[serde(default)]
    pub dma_buf: Option<bool>,

    /// Enable or disable client-side decorations for Wayland clients
    #[serde(default)]
    pub client_decorations: Option<bool>,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque)
    #[serde(default)]
    pub opacity: Option<f32>,
}

impl MergeWithConfigFile<CompositorConfigFile> for CompositorArguments {
    fn merge_with_config_file(mut self, config: &CompositorConfigFile) -> Self {
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
        if self.opacity == 1.0
            && let Some(opacity) = config.opacity
        {
            self.opacity = opacity;
        }
        self
    }
}
