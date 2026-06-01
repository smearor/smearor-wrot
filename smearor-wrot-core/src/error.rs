//! Core error types
//!
//! This module defines error types used throughout the smearor-wrot-core crate.
//! It uses `thiserror` for error derivation and `miette` for user-friendly error reporting.

use miette::Diagnostic;
use thiserror::Error;

/// Core error type for smearor-wrot
///
/// This enum represents all possible errors that can occur in the core compositor functionality.
/// It implements `std::error::Error`, `Debug`, and `Diagnostic` for comprehensive error handling.
#[derive(Error, Debug, Diagnostic)]
pub enum CoreError {
    /// Error related to Wayland compositor operations
    #[error("Compositor error: {message}")]
    #[diagnostic(
        code(smearor_wrot::compositor),
        help("Check that the Wayland compositor is running and accessible")
    )]
    Compositor {
        /// Error message describing the compositor issue
        message: String,
    },

    /// Error related to rendering operations
    #[error("Rendering error: {message}")]
    #[diagnostic(
        code(smearor_wrot::rendering),
        help("Ensure GPU acceleration is available and drivers are up to date")
    )]
    Rendering {
        /// Error message describing the rendering issue
        message: String,
    },

    /// Error related to surface management
    #[error("Surface error: {message}")]
    #[diagnostic(
        code(smearor_wrot::surface),
        help("Verify the application supports Wayland and is properly configured")
    )]
    Surface {
        /// Error message describing the surface issue
        message: String,
    },

    /// Error related to buffer management
    #[error("Buffer error: {message}")]
    #[diagnostic(
        code(smearor_wrot::buffer),
        help("Check available memory and GPU resources")
    )]
    Buffer {
        /// Error message describing the buffer issue
        message: String,
    },

    /// Error related to input handling
    #[error("Input error: {message}")]
    #[diagnostic(
        code(smearor_wrot::input),
        help("Verify input devices are properly connected and configured")
    )]
    Input {
        /// Error message describing the input issue
        message: String,
    },

    /// Error related to window lifecycle
    #[error("Window error: {message}")]
    #[diagnostic(
        code(smearor_wrot::window),
        help("Check window manager configuration and permissions")
    )]
    Window {
        /// Error message describing the window issue
        message: String,
    },

    /// I/O error from standard library
    #[error("IO error: {0}")]
    #[diagnostic(code(smearor_wrot::io))]
    Io(#[from] std::io::Error),

    /// Error related to configuration
    #[error("Configuration error: {message}")]
    #[diagnostic(
        code(smearor_wrot::config),
        help("Review configuration file syntax and values")
    )]
    Configuration {
        /// Error message describing the configuration issue
        message: String,
    },

    /// Error related to Wayland protocol
    #[error("Wayland protocol error: {message}")]
    #[diagnostic(
        code(smearor_wrot::wayland),
        help("Ensure Wayland libraries are installed and compatible")
    )]
    Wayland {
        /// Error message describing the Wayland protocol issue
        message: String,
    },
}

impl CoreError {
    /// Create a new compositor error
    pub fn compositor(message: impl Into<String>) -> Self {
        Self::Compositor {
            message: message.into(),
        }
    }

    /// Create a new rendering error
    pub fn rendering(message: impl Into<String>) -> Self {
        Self::Rendering {
            message: message.into(),
        }
    }

    /// Create a new surface error
    pub fn surface(message: impl Into<String>) -> Self {
        Self::Surface {
            message: message.into(),
        }
    }

    /// Create a new buffer error
    pub fn buffer(message: impl Into<String>) -> Self {
        Self::Buffer {
            message: message.into(),
        }
    }

    /// Create a new input error
    pub fn input(message: impl Into<String>) -> Self {
        Self::Input {
            message: message.into(),
        }
    }

    /// Create a new window error
    pub fn window(message: impl Into<String>) -> Self {
        Self::Window {
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a new Wayland protocol error
    pub fn wayland(message: impl Into<String>) -> Self {
        Self::Wayland {
            message: message.into(),
        }
    }
}

/// Result type alias for CoreError
pub type Result<T> = std::result::Result<T, CoreError>;
