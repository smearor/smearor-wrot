use crate::widget::compositor::error::CompositorError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ConfigError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
    #[error("Failed to lock config")]
    ConfigLockError,
}
