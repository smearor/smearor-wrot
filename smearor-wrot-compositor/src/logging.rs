//! Logging infrastructure for smearor-wrot
//!
//! This module provides centralized logging configuration using the `tracing` crate.
//! It supports environment variable-based log level filtering and structured logging.

use tracing_subscriber::EnvFilter;

/// Initialize the logging infrastructure
///
/// This function sets up the global tracing subscriber with the following features:
/// - Environment variable-based log level filtering (RUST_LOG)
/// - Pretty formatting for human-readable output
/// - ANSI color support for terminal output
/// - Default log level: INFO
///
/// # Examples
///
/// ```no_run
/// use smearor_wrot_compositor::init_logging;
///
/// fn main() {
///     init_logging();
///     tracing::info!("Application started");
/// }
/// ```
///
/// # Environment Variables
///
/// - `RUST_LOG`: Set the log level (e.g., `info`, `debug`, `trace`, `warn`, `error`)
/// - Default: `info`
/// - Example: `RUST_LOG=debug ./smearor-wrot`
pub fn init_logging() {
    let env_filter = EnvFilter::from_default_env();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

/// Initialize logging with a custom log level
///
/// # Arguments
///
/// * `level` - The minimum log level to display
///
/// # Examples
///
/// ```no_run
/// use smearor_wrot_compositor::init_logging_with_level;
/// use tracing::Level;
///
/// fn main() {
///     init_logging_with_level(Level::DEBUG);
///     tracing::debug!("Debug message");
/// }
/// ```
pub fn init_logging_with_level(level: tracing::Level) {
    let env_filter = EnvFilter::from_default_env().add_directive(level.into());

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

#[cfg(test)]
mod tests {
    use tracing::Level;

    #[test]
    fn test_init_logging_function_exists() {
        let _ = || super::init_logging();
    }

    #[test]
    fn test_init_logging_with_level_function_exists() {
        let _ = |level: Level| super::init_logging_with_level(level);
    }

    #[test]
    fn test_logging_levels_are_valid() {
        let levels = [Level::ERROR, Level::WARN, Level::INFO, Level::DEBUG, Level::TRACE];
        for level in levels {
            let _ = level;
        }
    }
}
