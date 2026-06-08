use clap::Parser;
use smearor_wrot_application::EnvironmentVariablesConfig;
use std::sync::atomic::AtomicBool;

/// Environment variables for the parent process.
#[derive(Parser, Debug, Clone)]
pub struct EnvironmentVariablesArguments {
    /// Enable WAYLAND_DEBUG=1 for the parent process.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) env_override_wayland_debug: bool,

    /// Override the WAYLAND_DISPLAY environment variable for the parent process.
    #[arg(long)]
    pub(crate) env_override_wayland_display: Option<String>,
}

impl From<EnvironmentVariablesArguments> for EnvironmentVariablesConfig {
    fn from(args: EnvironmentVariablesArguments) -> Self {
        Self {
            env_override_wayland_debug: AtomicBool::new(args.env_override_wayland_debug),
            env_override_wayland_display: args.env_override_wayland_display,
        }
    }
}
