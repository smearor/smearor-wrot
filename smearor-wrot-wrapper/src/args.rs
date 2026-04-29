use smearor_wrot_core::DEFAULT_WINDOW_HEIGHT;
use smearor_wrot_core::DEFAULT_WINDOW_WIDTH;
use smearor_wrot_rotation::layer::SmearorLayer;
use clap::Parser;
use std::ffi::OsString;

/// Command line arguments for the Adlisac wrapper application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Disable the rotation widget even if a rotation value is provided.
    #[arg(short = 'R', long, action)]
    pub(crate) disable_rotation: bool,

    /// Rotation angle in degrees.
    #[arg(short, long, default_value_t = 0.0)]
    pub(crate) rotation: f32,

    /// Initial width of the application window.
    #[arg(short = 'W', long, default_value_t = DEFAULT_WINDOW_WIDTH)]
    pub(crate) width: i32,

    /// Initial height of the application window.
    #[arg(short = 'H', long, default_value_t = DEFAULT_WINDOW_HEIGHT)]
    pub(crate) height: i32,

    /// Whether the window should have client-side decorations.
    #[arg(short = 'd', long, action)]
    pub(crate) decorated: bool,

    /// Title of the application window.
    #[arg(short = 't', long)]
    pub(crate) title: Option<String>,

    /// Specify the layer for the layer shell protocol (e.g., Background, Top).
    #[arg(long)]
    pub(crate) layer: Option<SmearorLayer>,

    /// Namespace for the layer shell, used by compositors for rules.
    #[arg(short = 'n', long)]
    pub(crate) namespace: Option<String>,

    /// Runs the command in a shell
    #[arg(short = 's', long, action)]
    pub(crate) shell: bool,

    /// Path to the Wayland Unix socket to be created.
    #[arg(short = 'S', long, default_value = "/tmp/io.smearor.casilda.simple.sock")]
    pub(crate) socket: String,

    /// Arguments to be passed to the client application.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub(crate) command_arguments: Vec<OsString>,
}
