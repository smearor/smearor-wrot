use smearor_wrot_compositor::CoreError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CompositorError {
    #[error("Compositor not found")]
    CompositorNotFound,
    #[error("Failed to lock compositor")]
    CompositorLockError,
}

#[derive(Debug, Error)]
pub enum CompositorInitializationError {
    #[error("The compositor socket is not initialized")]
    CompositorSocketNotInitialized,
    #[error("Failed to create event loop")]
    FailedToCreateEventLoop,
    #[error("Failed to lock compositor")]
    FailedToLockCompositor,
    #[error("Failed to get listening socket")]
    FailedToGetListeningSocket,
    #[error("Failed to create display")]
    FailedToCreateDisplay,
    #[error("Failed to lock display")]
    FailedToLockDisplay,
    #[error(transparent)]
    FailedToInitializeCompositor(#[from] CoreError),
}
