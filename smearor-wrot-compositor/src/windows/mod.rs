//! Window lifecycle management
//!
//! This module provides traits and implementations for window lifecycle management
//! including activation, focus, configuration, close handling, and state queries.

pub mod activation;
pub mod close;
pub mod configuration;
pub mod decoration;
pub mod focus;
pub mod state;
pub mod title;

pub use activation::WindowActivation;
pub use close::WindowClose;
pub use configuration::WindowConfiguration;
pub use focus::WindowFocus;
pub use state::WindowState;
pub use title::WindowTitle;
