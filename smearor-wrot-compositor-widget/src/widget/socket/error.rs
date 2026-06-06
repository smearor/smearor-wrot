use smearor_wrot_compositor::CoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SocketInitializationError {
    #[error("Failed to create event loop")]
    FailedToCreateEventLoop,
    #[error("Failed to create display")]
    FailedToCreateDisplay,
    #[error(transparent)]
    FailedToInitializeCompositor(#[from] CoreError),
    #[error("Failed to lock compositor")]
    FailedToLockCompositor,
    #[error("Failed to get listening socket")]
    FailedToGetListeningSocket,
    #[error("Failed to lock display")]
    FailedToLockDisplay,
}
