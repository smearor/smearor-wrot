use thiserror::Error;

#[derive(Debug, Error)]
pub enum SocketInitializationError {
    #[error("Failed to lock compositor")]
    FailedToLockCompositor,
}
