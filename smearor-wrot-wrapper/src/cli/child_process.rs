use clap::Parser;
use smearor_wrot_application::ChildProcessStartType;
use smearor_wrot_application::ChildProcessState;
use smearor_wrot_application::ChildProcessStdio;
use smearor_wrot_application::SocketBuilder;
use smearor_wrot_application::SocketBuilderError;
use std::error::Error;
use std::ffi::OsString;
use thiserror::Error;

#[derive(Parser, Debug, Clone)]
pub struct ChildProcessArguments {
    /// Arguments to be passed to the client application.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub(crate) command_arguments: Vec<OsString>,

    /// Environment variables to set on the child process.
    #[arg(long, value_parser = parse_key_val::<String, i32>)]
    pub(crate) env: Vec<(OsString, OsString)>,

    /// Enable GSK_RENDERER=gl for child process.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) gsk_renderer_gl: bool,

    /// Path to the Wayland Unix socket to be created (relative name in XDG_RUNTIME_DIR).
    #[arg(short = 'S', long)]
    pub(crate) socket: Option<String>,

    /// Runs the child process in a shell or in a terminal emulator.
    #[arg(long, default_value_t = ChildProcessStartType::Direct)]
    pub(crate) start_type: Option<ChildProcessStartType>,

    /// The stdio stream type for stderr
    #[arg(long, default_value_t = ChildProcessStdio::Piped)]
    pub(crate) stderr: ChildProcessStdio,

    /// The stdio stream type for stdin
    #[arg(long, default_value_t = ChildProcessStdio::Inherit)]
    pub(crate) stdin: ChildProcessStdio,

    /// The stdio stream type for stdout
    #[arg(long, default_value_t = ChildProcessStdio::Piped)]
    pub(crate) stdout: ChildProcessStdio,

    /// Enable WAYLAND_DEBUG=1 for the child process.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) wayland_debug: bool,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Debug, Error)]
pub enum ChildProcessConfigError {
    #[error(transparent)]
    SocketBuilderError(#[from] SocketBuilderError),
}

impl TryFrom<ChildProcessArguments> for ChildProcessState {
    type Error = ChildProcessConfigError;

    fn try_from(args: ChildProcessArguments) -> Result<Self, Self::Error> {
        let socket = SocketBuilder::build(&args.socket)?;
        let mut builder = ChildProcessState::builder(socket)
            .command_arguments(args.command_arguments)
            .proxy_env_variables()
            .start_type(args.start_type.unwrap_or_default())
            .stdin(args.stdin)
            .stdout(args.stdout)
            .stderr(args.stderr)
            .xdg_runtime_dir()
            .dbus_session_bus_address()
            .gdk_backend();
        if args.wayland_debug {
            builder = builder.wayland_debug();
        }
        if args.gsk_renderer_gl {
            builder = builder.gsk_renderer_gl();
        } else {
            builder = builder.gsk_renderer_ngl();
        }
        Ok(builder.build())
    }
}
