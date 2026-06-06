use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ColorMaskError {
    #[error("Failed to set color mask")]
    FailedToSetColorMask,

    #[error("Failed to clear color mask")]
    FailedToClearColorMask,
}
