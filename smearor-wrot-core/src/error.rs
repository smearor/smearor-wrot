//! Core error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Compositor error: {0}")]
    Compositor(String),
    
    #[error("Rendering error: {0}")]
    Rendering(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
