use crate::widget::compositor::error::CompositorError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum TouchInputEventError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
}
