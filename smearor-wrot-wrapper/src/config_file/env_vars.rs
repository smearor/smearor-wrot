use crate::cli::env_vars::EnvironmentVariablesArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct EnvironmentVariablesConfigFile {
    /// Enable WAYLAND_DEBUG=1 for the parent process.
    #[serde(default)]
    pub(crate) env_override_wayland_debug: Option<bool>,

    /// Override the WAYLAND_DISPLAY environment variable for the parent process.
    #[serde(default)]
    pub(crate) env_override_wayland_display: Option<String>,
}

impl MergeWithConfigFile<EnvironmentVariablesConfigFile> for EnvironmentVariablesArguments {
    fn merge_with_config_file(mut self, config: &EnvironmentVariablesConfigFile) -> Self {
        if !self.env_override_wayland_debug
            && let Some(env_override_wayland_debug) = config.env_override_wayland_debug
        {
            self.env_override_wayland_debug = env_override_wayland_debug;
        }
        if !self.env_override_wayland_display.is_none() && config.env_override_wayland_display.is_some() {
            self.env_override_wayland_display = config.env_override_wayland_display.clone();
        }
        self
    }
}
