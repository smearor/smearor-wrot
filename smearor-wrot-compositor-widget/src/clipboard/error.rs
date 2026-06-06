use crate::clipboard::host_clipboard::HostClipboardError;
use crate::widget::compositor::error::CompositorError;
use smearor_wrot_compositor::clipboard::error::CompositorSelectionError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CompositorClipboardError {
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
    #[error("Could not lock clipboard content")]
    ClipboardContentLockError,
    #[error(transparent)]
    CompositorSelectionError(#[from] CompositorSelectionError),
}

#[derive(Debug, Clone, Error)]
pub enum ClipboardSyncError {
    #[error("Host Clipboard Error: {0}")]
    HostClipboardError(#[from] HostClipboardError),
    #[error("Compositor Clipboard Error: {0}")]
    CompositorClipboardError(#[from] CompositorClipboardError),
    #[error(transparent)]
    CompositorError(#[from] CompositorError),
    #[error("Could not lock clipboard pipe read end")]
    PipeReadLockError,
    #[error("Compositor Selection Error: {0}")]
    CompositorSelectionError(#[from] CompositorSelectionError),
}
