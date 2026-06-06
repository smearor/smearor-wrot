use crate::widget::compositor::error::CompositorError;
use smearor_wrot_compositor::texture::pixel_data::PixelDataSaveError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveBufferError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
    #[error("No top level surface")]
    NoTopLevelSurface,
    #[error("Failed to render buffer from holding area")]
    RenderBufferFromHoldingAreaError,
    #[error(transparent)]
    PixelDataSaveError(#[from] PixelDataSaveError),
}
