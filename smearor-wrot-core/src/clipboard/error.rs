use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CompositorSelectionError {
    #[error("Could not lock display")]
    DisplayLock,
    #[error("Could not lock clipboard pipe read end")]
    ClipboardPipeReadEndLock,
    #[error("Could not lock clipboard content")]
    ClipboardContentLock,
    #[error("Could not read from pipe")]
    ReadPipeError,
}
