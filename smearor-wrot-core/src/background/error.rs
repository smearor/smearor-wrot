use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum BackgroundColorError {
    #[error("Failed to set background color")]
    FailedToSetBackgroundColor,
}
