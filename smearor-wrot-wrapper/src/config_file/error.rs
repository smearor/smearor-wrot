use thiserror::Error;

/// Configuration error type
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(String),

    #[error("Failed to parse config file: {0}")]
    ParseError(String),
}
