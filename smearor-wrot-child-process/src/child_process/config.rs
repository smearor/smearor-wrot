use crate::ChildProcess;
use crate::ChildProcessStartError;
use crate::ChildProcessStartType;
use crate::ChildProcessStdio;
use crate::DBUS_SESSION_BUS_ADDRESS;
use crate::GDK_BACKEND;
use crate::GDK_BACKEND_WAYLAND;
use crate::GSK_RENDERER;
use crate::GSK_RENDERER_GL;
use crate::GSK_RENDERER_NGL;
use crate::Socket;
use crate::WAYLAND_DEBUG;
use crate::WAYLAND_DISPLAY;
use crate::XDG_RUNTIME_DIR;
use dashmap::DashMap;
use smithay::reexports::rustix::path::Arg;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::process::Command;
use std::process::Stdio;
use tracing::debug;
use which::which;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChildProcessConfig {
    /// The wayland socket to connect to
    socket: Socket,

    /// The arguments to pass to the child process. The first argument is the command to launch.
    command_arguments: Vec<OsString>,

    /// The environment variables to set for the child process
    env_variables: Vec<(OsString, OsString)>,

    /// The start type of the child process
    start_type: ChildProcessStartType,

    /// The stdio stream type for stdin
    stdin: ChildProcessStdio,

    /// The stdio stream type for stdout
    stdout: ChildProcessStdio,

    /// The stdio stream type for stderr
    stderr: ChildProcessStdio,
}

impl ChildProcessConfig {
    pub fn new(socket: Socket) -> Self {
        let mut env_variables = Vec::new();
        env_variables.push((OsString::from(WAYLAND_DISPLAY), OsString::from(&socket)));
        Self {
            socket,
            command_arguments: Vec::new(),
            env_variables: Vec::new(),
            start_type: ChildProcessStartType::Direct,
            stdin: ChildProcessStdio::Inherit,
            stdout: ChildProcessStdio::Piped,
            stderr: ChildProcessStdio::Piped,
        }
    }

    pub fn builder(socket: Socket) -> ChildProcessConfigBuilder {
        ChildProcessConfigBuilder::new(socket)
    }

    pub fn socket(&self) -> Socket {
        self.socket.clone()
    }

    pub fn executable_name(&self) -> OsString {
        self.command_arguments[0].clone()
    }

    pub fn arguments(&self) -> Vec<OsString> {
        self.command_arguments.clone()
    }

    pub fn env_variables(&self) -> Vec<(OsString, OsString)> {
        self.env_variables.clone()
    }

    pub fn start_type(&self) -> ChildProcessStartType {
        self.start_type
    }

    pub fn resolved_path(&self) -> Result<OsString, ChildProcessStartError> {
        let executable_name = self.executable_name();
        which(&executable_name)
            .map(|path| {
                debug!("Resolved executable '{}' to: {}", executable_name.to_string_lossy().to_string(), path.display());
                path.as_os_str().to_os_string()
            })
            .map_err(|_| ChildProcessStartError::ExecutableNotFoundInPath(executable_name.to_string_lossy().to_string()))
    }

    pub fn command(&self) -> Result<Command, ChildProcessStartError> {
        let mut command = match self.start_type {
            ChildProcessStartType::Direct => {
                let resolved_path = self.resolved_path()?;
                let mut cmd = Command::new(&resolved_path);
                if self.command_arguments.len() > 1 {
                    cmd.args(&self.command_arguments[1..]);
                }
                cmd
            }
            ChildProcessStartType::Shell => {
                let mut cmd = Command::new("sh");
                cmd.arg("-c");
                cmd.arg(self.command_arguments.join(OsStr::new(" ")).to_string_lossy().to_string());
                cmd
            }
            ChildProcessStartType::Terminal => {
                let mut cmd = Command::new("alacritty");
                cmd.arg("--command");
                cmd.arg(self.command_arguments.join(OsStr::new(" ")).to_string_lossy().to_string());
                cmd
            }
        };

        // Sets the environment variables
        for (env_var, value) in &self.env_variables {
            command.env(env_var, value);
        }

        // Sets the stdin, stdout and stderr
        command.stdin(Stdio::from(&self.stdin));
        command.stdout(Stdio::from(&self.stdout));
        command.stderr(Stdio::from(&self.stderr));
        Ok(command)
    }

    /// Launches the child process
    pub fn launch_child_process(&self) -> Result<ChildProcess, ChildProcessStartError> {
        let mut command = self.command()?;
        let child = command.spawn().map_err(|e| ChildProcessStartError::SpawnChildProcessError(e.to_string()))?;
        let mut child_process = ChildProcess::new(child, self.clone());
        debug!("Child process launched with PID {} on wayland socket {}", child_process.pid(), child_process.socket());
        child_process.follow_stdout();
        child_process.follow_stderr();
        Ok(child_process)
    }
}

pub struct ChildProcessConfigBuilder {
    env_variables: DashMap<OsString, OsString>,
    child_process_start_config: ChildProcessConfig,
}

impl ChildProcessConfigBuilder {
    pub fn new(socket: Socket) -> Self {
        Self {
            env_variables: DashMap::new(),
            child_process_start_config: ChildProcessConfig::new(socket.clone()),
        }
        .wayland_display(socket)
    }

    pub fn build(mut self) -> ChildProcessConfig {
        self.child_process_start_config.env_variables = self.env_variables.into_iter().collect();
        self.child_process_start_config
    }

    pub fn command_arguments(mut self, command_arguments: Vec<OsString>) -> Self {
        self.child_process_start_config.command_arguments = command_arguments;
        self
    }

    /// Adds all environment variables of the compositor process to the child process
    pub fn proxy_env_variables(mut self) -> Self {
        for (env_var, value) in std::env::vars() {
            self.env_variables.insert(env_var.into(), value.into());
        }
        self
    }

    /// Adds the given environment variables to the child process
    pub fn add_env_variables(mut self, env_variables: Vec<(OsString, OsString)>) -> Self {
        for (env_var, value) in env_variables {
            self.env_variables.insert(env_var, value);
        }
        self
    }

    /// Sets the start type (direct, shell, terminal)
    pub fn start_type(mut self, start_type: ChildProcessStartType) -> Self {
        self.child_process_start_config.start_type = start_type;
        self
    }

    /// Starts the child process directly (default, for wayland applications)
    pub fn direct(mut self) -> Self {
        self.child_process_start_config.start_type = ChildProcessStartType::Direct;
        self
    }

    /// Starts the child process in a shell (for shell scripts)
    pub fn shell(mut self) -> Self {
        self.child_process_start_config.start_type = ChildProcessStartType::Shell;
        self
    }

    /// Starts the child process in a terminal emulator (for terminal applications)
    pub fn terminal(mut self) -> Self {
        self.child_process_start_config.start_type = ChildProcessStartType::Terminal;
        self
    }

    /// Sets the stdio stream type for stdin
    pub fn stdin(mut self, stdio: ChildProcessStdio) -> Self {
        self.child_process_start_config.stdin = stdio;
        self
    }

    /// Sets the stdio stream type for stdout
    pub fn stdout(mut self, stdio: ChildProcessStdio) -> Self {
        self.child_process_start_config.stdout = stdio;
        self
    }

    /// Sets the stdio stream type for stderr
    pub fn stderr(mut self, stdio: ChildProcessStdio) -> Self {
        self.child_process_start_config.stderr = stdio;
        self
    }

    /// Sets the wayland display environment variable
    pub fn wayland_display(mut self, socket: Socket) -> Self {
        self.child_process_start_config.socket = socket.clone();
        self.env_variables.insert(WAYLAND_DISPLAY.into(), socket.as_os_str().to_os_string());
        self
    }

    pub fn xdg_runtime_dir(self) -> Self {
        if let Ok(xdg_runtime_dir) = std::env::var(XDG_RUNTIME_DIR) {
            self.env_variables.insert(XDG_RUNTIME_DIR.into(), xdg_runtime_dir.into());
        }
        self
    }

    pub fn dbus_session_bus_address(self) -> Self {
        if let Ok(dbus_session_bus_address) = std::env::var(DBUS_SESSION_BUS_ADDRESS) {
            self.env_variables.insert(DBUS_SESSION_BUS_ADDRESS.into(), dbus_session_bus_address.into());
        }
        self
    }

    pub fn gdk_backend(self) -> Self {
        self.env_variables.insert(GDK_BACKEND.into(), GDK_BACKEND_WAYLAND.into());
        self
    }

    pub fn wayland_debug(self) -> Self {
        self.env_variables.insert(WAYLAND_DEBUG.into(), "1".into());
        self
    }

    pub fn gsk_renderer_ngl(self) -> Self {
        self.env_variables.insert(GSK_RENDERER.into(), GSK_RENDERER_NGL.into());
        self
    }

    pub fn gsk_renderer_gl(self) -> Self {
        self.env_variables.insert(GSK_RENDERER.into(), GSK_RENDERER_GL.into());
        self
    }
}
