use clap::Parser;
use smearor_wrot_application::DEFAULT_WINDOW_HEIGHT;
use smearor_wrot_application::DEFAULT_WINDOW_WIDTH;
use smearor_wrot_application::SmearorLayer;
use std::ffi::OsString;

/// Smearor WROT
#[derive(Parser, Debug, Clone)]
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
    #[arg(short = 'd', long, action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub(crate) decorated: bool,

    /// Whether the window should be resizable.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = true)]
    pub(crate) resizable: bool,

    /// Initial x position of the window.
    #[arg(short = 'x', long)]
    pub(crate) position_x: Option<i32>,

    /// Initial y position of the window.
    #[arg(short = 'y', long)]
    pub(crate) position_y: Option<i32>,

    /// Minimum width of the window.
    #[arg(short = 'w', long)]
    pub(crate) min_width: Option<i32>,

    /// Minimum height of the window.
    #[arg(long)]
    pub(crate) min_height: Option<i32>,

    /// Maximum width of the window.
    #[arg(long)]
    pub(crate) max_width: Option<i32>,

    /// Maximum height of the window.
    #[arg(long)]
    pub(crate) max_height: Option<i32>,

    /// Aspect ratio as width/height (e.g., 1.777 for 16:9).
    #[arg(long)]
    pub(crate) aspect_ratio: Option<f32>,

    /// Start in fullscreen mode.
    #[arg(short = 'f', long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) fullscreen: bool,

    /// Start in maximized mode.
    #[arg(short = 'm', long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) maximized: bool,

    /// Enable double buffering (default: true).
    #[arg(short = 'b', long, action = clap::ArgAction::SetTrue, default_value_t = true)]
    pub(crate) double_buffer: bool,

    /// Disable DMA-BUF hardware acceleration (default: false).
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_dma_buf: bool,

    /// Application ID.
    #[arg(short = 'i', long)]
    pub(crate) id: Option<String>,

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

    /// Path to the Wayland Unix socket to be created (relative name in XDG_RUNTIME_DIR).
    #[arg(short = 'S', long)]
    pub(crate) socket: Option<String>,

    /// Path to the configuration file (TOML format).
    #[arg(short = 'c', long)]
    pub(crate) config: Option<std::path::PathBuf>,

    /// Enable WAYLAND_DEBUG=1 for child process.
    #[arg(long, action)]
    pub(crate) wayland_debug: bool,

    /// Enable GSK_RENDERER=gl for child process.
    #[arg(long, action)]
    pub(crate) gsk_renderer_gl: bool,

    /// Disable client-side decorations for Wayland clients in the compositor.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_client_decorations: bool,

    /// Margin in pixels for window rendering (shortcut for all margins).
    #[arg(long)]
    pub(crate) margin: Option<u32>,

    /// Left margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_left: u32,

    /// Right margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_right: u32,

    /// Top margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_top: u32,

    /// Bottom margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_bottom: u32,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque).
    #[arg(long, default_value_t = 1.0)]
    pub(crate) opacity: f32,

    /// Background color in hex format (e.g., #FF0000 for red).
    #[arg(long)]
    pub(crate) background_color: Option<String>,

    /// Subsurface background color in hex format (e.g., #FF0000 for red on subsurfaces).
    #[arg(long)]
    pub(crate) subsurface_background_color: Option<String>,

    /// Color mask in hex format for chroma-keying (e.g., #808080 to make gray transparent).
    #[arg(long)]
    pub(crate) color_mask: Option<String>,

    /// Enable automatic background color detection for color mask.
    #[arg(long, action)]
    pub(crate) auto_color_mask: bool,

    /// Subsurface color mask in hex format for chroma-keying (e.g., #FFFFFF to make white transparent on subsurfaces).
    #[arg(long)]
    pub(crate) subsurface_color_mask: Option<String>,

    /// Enable automatic background color detection for subsurface color mask.
    #[arg(long, action)]
    pub(crate) auto_subsurface_color_mask: bool,

    /// Tolerance for color mask (0.0-1.0, default: 0.1).
    #[arg(long, default_value_t = 0.1)]
    pub(crate) color_mask_tolerance: f32,

    /// Enable shader-based color masking for better performance (default: false).
    #[arg(long, action)]
    pub(crate) color_mask_shader: bool,

    /// Window opacity for the compositor window (0.0 = fully transparent, 1.0 = fully opaque).
    #[arg(long, default_value_t = 1.0)]
    pub(crate) window_opacity: f32,

    /// Maximum frames per second (default: 60).
    #[arg(long, default_value_t = 60)]
    pub(crate) max_fps: i64,

    /// Dialog margin in pixels for dialog positioning (default: 0).
    #[arg(long, default_value_t = 0)]
    pub(crate) dialog_margin: u32,

    /// Animation speed in milliseconds for rotation overshoot animation (default: 500).
    #[arg(long, default_value_t = 500)]
    pub(crate) animation_speed: u64,

    /// Animation overshoot amount for rotation gesture (default: 1.7).
    #[arg(long, default_value_t = 1.7)]
    pub(crate) animation_overshoot: f64,

    /// Disable all animations.
    #[arg(long, action)]
    pub(crate) disable_animations: bool,

    /// Enable visual debugging of touch points (red rectangle for GTK coordinates, green border for app coordinates).
    #[arg(long, action)]
    pub(crate) debug_touch: bool,

    /// Enable visual debugging of pointer (blue rectangle for GTK coordinates, magenta border for app coordinates).
    #[arg(long, action)]
    pub(crate) debug_pointer: bool,

    /// Override the WAYLAND_DISPLAY environment variable for the GTK4 application.
    #[arg(long)]
    pub(crate) override_wayland_display: Option<String>,

    /// Keyboard layout (e.g., "de", "us"). Overrides automatic detection.
    #[arg(long)]
    pub(crate) keyboard_layout: Option<String>,

    /// Keyboard variant (e.g., "nodeadkeys"). Overrides automatic detection.
    #[arg(long)]
    pub(crate) keyboard_variant: Option<String>,

    /// Arguments to be passed to the client application.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub(crate) command_arguments: Vec<OsString>,
}
