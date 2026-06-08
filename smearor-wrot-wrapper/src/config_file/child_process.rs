use crate::cli::child_process::ChildProcessArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;
use smearor_wrot_application::ChildProcessStartType;
use smearor_wrot_application::ChildProcessStdio;
use std::collections::HashMap;
use std::ffi::OsString;

#[derive(Debug, Deserialize, Default)]
pub struct ChildProcessConfigFile {
    /// Environment variables to set on the child process.
    #[serde(default)]
    pub env: HashMap<OsString, OsString>,

    /// Enable GSK_RENDERER=gl for child process.
    #[serde(default)]
    pub gsk_renderer_gl: Option<bool>,

    /// Path to the Wayland Unix socket to be created (relative name in XDG_RUNTIME_DIR).
    #[serde(default)]
    pub socket: Option<String>,

    /// Runs the command in a shell
    #[serde(default)]
    pub start_type: Option<ChildProcessStartType>,

    /// The stdio stream type for stderr
    #[serde(default)]
    pub stderr: Option<ChildProcessStdio>,

    /// The stdio stream type for stdin
    #[serde(default)]
    pub stdin: Option<ChildProcessStdio>,

    /// The stdio stream type for stdout
    #[serde(default)]
    pub stdout: Option<ChildProcessStdio>,

    /// Enable WAYLAND_DEBUG=1 for the child process.
    #[serde(default)]
    pub wayland_debug: Option<bool>,
}

impl MergeWithConfigFile<ChildProcessConfigFile> for ChildProcessArguments {
    fn merge_with_config_file(mut self, config: &ChildProcessConfigFile) -> Self {
        // TODO: merge env variables
        if !self.gsk_renderer_gl
            && let Some(gsk_renderer_gl) = config.gsk_renderer_gl
        {
            self.gsk_renderer_gl = gsk_renderer_gl;
        }
        if self.socket.is_none() && config.socket.is_some() {
            self.socket = config.socket.clone();
        }
        if self.start_type.is_none() && config.start_type.is_some() {
            self.start_type = config.start_type.clone();
        }
        if self.stderr != ChildProcessStdio::Piped
            && let Some(stderr) = config.stderr
        {
            self.stderr = stderr;
        }
        if self.stdin != ChildProcessStdio::Inherit
            && let Some(stdin) = config.stdin
        {
            self.stdin = stdin;
        }
        if self.stdout != ChildProcessStdio::Piped
            && let Some(stdout) = config.stdout
        {
            self.stdout = stdout;
        }
        if !self.wayland_debug
            && let Some(wayland_debug) = config.wayland_debug
        {
            self.wayland_debug = wayland_debug;
        }
        self
    }
}
