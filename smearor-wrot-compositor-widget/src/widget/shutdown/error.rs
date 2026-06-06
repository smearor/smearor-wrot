use crate::widget::compositor::error::CompositorError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ShutdownCheckError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
}
