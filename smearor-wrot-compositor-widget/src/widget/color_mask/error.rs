use crate::widget::compositor::error::CompositorError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ColorMaskError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
    #[error(transparent)]
    ColorMaskError(#[from] smearor_wrot_color_mask::ColorMaskError),
}
