use crate::widget::compositor::error::CompositorError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum KeyboardInputEventError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
}

#[derive(Debug, Clone, Error)]
pub enum MouseInputEventError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
}

#[derive(Debug, Clone, Error)]
pub enum TouchInputEventError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
}
