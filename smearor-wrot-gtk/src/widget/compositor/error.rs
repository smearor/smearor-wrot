use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CompositorError {
    #[error("Compositor not found")]
    CompositorNotFound,
    #[error("Failed to lock compositor")]
    CompositorLockError,
}
