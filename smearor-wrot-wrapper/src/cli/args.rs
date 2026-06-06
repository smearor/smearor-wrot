use crate::config_file::config::SmearorWindowRotationConfig;
use clap::Parser;
use smearor_wrot_application::CompositorApplicationConfig;
use smearor_wrot_application::DEFAULT_WINDOW_HEIGHT;
use smearor_wrot_application::DEFAULT_WINDOW_WIDTH;
use smearor_wrot_application::SmearorLayer;
use std::error::Error;
use std::ffi::OsString;
use tracing::debug;

/// Smearor Window Rotation Compositor
///
/// A Wayland window rotation system designed for multi-user collaborative smart desks, enabling
/// individual window rotation without rotating the entire screen.
///
/// ## Overview
///
/// **smearor-wrot** solves the orientation problem on large touchscreen smart desks where users
/// sit at different sides of the table. When users sit opposite each other, one person sees the
/// content upside down. smearor-wrot allows individual window rotation so multiple users can
/// interact with applications oriented toward their position.
///
/// ### Key Features
///
/// - **Individual Window Rotation**: Rotate any Wayland application window by any angle
/// - **Input Transformation**: Mouse and touch input coordinates are automatically transformed according to window rotation
/// - **Cross-Desktop Compatibility**: Works with Hyprland, Sway, GNOME, and other Wayland compositors
/// - **High Performance**: Maintains 60 FPS rendering with hardware acceleration support
/// - **Touch Support**: Full touch input support for smart desk surfaces
/// - **Multi-Window**: Support for multiple rotated windows simultaneously
///
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct SmearorWindowRotationArguments {
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

impl SmearorWindowRotationArguments {
    pub(crate) fn load_and_merge_config(self) -> Result<Self, Box<dyn Error>> {
        let Some(config_path) = self.config.clone() else {
            return Ok(self);
        };
        match SmearorWindowRotationConfig::load_config(&config_path) {
            Ok(config) => {
                debug!("Loaded configuration from: {config_path:?}");
                Ok(self.merge_with_config(&config))
            }
            Err(e) => {
                eprintln!("Failed to load configuration file: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub(crate) fn merge_with_config(&self, config: &SmearorWindowRotationConfig) -> Self {
        let mut args = (*self).clone();

        // Window configuration
        if self.title.is_none() && config.window.title.is_some() {
            args.title = config.window.title.clone();
        }
        if self.width == DEFAULT_WINDOW_WIDTH
            && let Some(width) = config.window.width
        {
            args.width = width;
        }
        if self.height == DEFAULT_WINDOW_HEIGHT
            && let Some(height) = config.window.height
        {
            args.height = height;
        }
        if self.decorated
            && let Some(decorated) = config.window.decorated
        {
            args.decorated = decorated;
        }
        if self.resizable
            && let Some(resizable) = config.window.resizable
        {
            args.resizable = resizable;
        }
        if self.position_x.is_none() && config.window.position_x.is_some() {
            args.position_x = config.window.position_x;
        }
        if self.position_y.is_none() && config.window.position_y.is_some() {
            args.position_y = config.window.position_y;
        }
        if self.min_width.is_none() && config.window.min_width.is_some() {
            args.min_width = config.window.min_width;
        }
        if self.min_height.is_none() && config.window.min_height.is_some() {
            args.min_height = config.window.min_height;
        }
        if self.max_width.is_none() && config.window.max_width.is_some() {
            args.max_width = config.window.max_width;
        }
        if self.max_height.is_none() && config.window.max_height.is_some() {
            args.max_height = config.window.max_height;
        }
        if self.aspect_ratio.is_none() && config.window.aspect_ratio.is_some() {
            args.aspect_ratio = config.window.aspect_ratio;
        }
        if !self.fullscreen
            && let Some(fullscreen) = config.window.fullscreen
        {
            args.fullscreen = fullscreen;
        }
        if !self.maximized
            && let Some(maximized) = config.window.maximized
        {
            args.maximized = maximized;
        }

        // Compositor configuration
        if self.double_buffer
            && let Some(double_buffer) = config.compositor.double_buffer
        {
            args.double_buffer = double_buffer;
        }
        if !self.disable_rotation
            && let Some(disable_rotation) = config.compositor.disable_rotation
        {
            args.disable_rotation = disable_rotation;
        }
        if self.rotation == 0.0
            && let Some(rotation) = config.compositor.rotation
        {
            args.rotation = rotation;
        }
        if self.socket.as_deref() == Some("/tmp/io.smearor.wrot.sock") && config.compositor.socket.is_some() {
            args.socket = config.compositor.socket.clone();
        }
        if self.layer.is_none() && config.compositor.layer.is_some() {
            if let Some(layer_str) = config.compositor.layer.as_ref() {
                args.layer = Some(SmearorLayer::from(layer_str.as_str()));
            }
        }
        if self.namespace.is_none() && config.compositor.namespace.is_some() {
            args.namespace = config.compositor.namespace.clone();
        }
        if !self.shell
            && let Some(shell) = config.compositor.shell
        {
            args.shell = shell;
        }
        if !self.disable_dma_buf
            && let Some(disable_dma_buf) = config.compositor.disable_dma_buf
        {
            args.disable_dma_buf = disable_dma_buf;
        }

        if !self.disable_client_decorations
            && let Some(disable_client_decorations) = config.compositor.disable_client_decorations
        {
            args.disable_client_decorations = disable_client_decorations;
        }
        if self.margin_left == 0
            && let Some(margin_left) = config.compositor.margin_left
        {
            args.margin_left = margin_left;
        }
        if self.margin_right == 0
            && let Some(margin_right) = config.compositor.margin_right
        {
            args.margin_right = margin_right;
        }
        if self.margin_top == 0
            && let Some(margin_top) = config.compositor.margin_top
        {
            args.margin_top = margin_top;
        }
        if self.margin_bottom == 0
            && let Some(margin_bottom) = config.compositor.margin_bottom
        {
            args.margin_bottom = margin_bottom;
        }
        if self.dialog_margin == 0
            && let Some(dialog_margin) = config.compositor.dialog_margin
        {
            args.dialog_margin = dialog_margin;
        }
        if self.opacity == 1.0
            && let Some(opacity) = config.compositor.opacity
        {
            args.opacity = opacity;
        }
        if self.background_color.is_none() && config.compositor.background_color.is_some() {
            args.background_color = config.compositor.background_color.clone();
        }
        if self.window_opacity == 1.0
            && let Some(window_opacity) = config.compositor.window_opacity
        {
            args.window_opacity = window_opacity;
        }
        if self.max_fps == 60
            && let Some(max_fps) = config.compositor.max_fps
        {
            args.max_fps = max_fps;
        }
        if !self.color_mask_shader
            && let Some(color_mask_shader) = config.compositor.color_mask_shader
        {
            args.color_mask_shader = color_mask_shader;
        }
        if !self.disable_animations
            && let Some(disable_animations) = config.compositor.disable_animations
        {
            args.disable_animations = disable_animations;
        }
        args
    }
}

impl From<SmearorWindowRotationArguments> for CompositorApplicationConfig {
    fn from(command_line_arguments: SmearorWindowRotationArguments) -> Self {
        CompositorApplicationConfig::builder()
            .disable_rotation(command_line_arguments.disable_rotation)
            .rotation(command_line_arguments.rotation)
            .width(command_line_arguments.width)
            .height(command_line_arguments.height)
            .decorated(command_line_arguments.decorated)
            .resizable(command_line_arguments.resizable)
            .position_x(command_line_arguments.position_x)
            .position_y(command_line_arguments.position_y)
            .min_width(command_line_arguments.min_width)
            .min_height(command_line_arguments.min_height)
            .max_width(command_line_arguments.max_width)
            .max_height(command_line_arguments.max_height)
            .aspect_ratio(command_line_arguments.aspect_ratio)
            .fullscreen(command_line_arguments.fullscreen)
            .maximized(command_line_arguments.maximized)
            .double_buffer(command_line_arguments.double_buffer)
            .disable_dma_buf(command_line_arguments.disable_dma_buf)
            .id(command_line_arguments.id.clone())
            .title(command_line_arguments.title.clone())
            .layer(command_line_arguments.layer)
            .namespace(command_line_arguments.namespace.clone())
            .shell(command_line_arguments.shell)
            .socket(command_line_arguments.socket.clone())
            .config_path(command_line_arguments.config.clone())
            .wayland_debug(command_line_arguments.wayland_debug)
            .gsk_renderer_gl(command_line_arguments.gsk_renderer_gl)
            .disable_client_decorations(command_line_arguments.disable_client_decorations)
            .margin(command_line_arguments.margin)
            .margin_left(command_line_arguments.margin_left)
            .margin_right(command_line_arguments.margin_right)
            .margin_top(command_line_arguments.margin_top)
            .margin_bottom(command_line_arguments.margin_bottom)
            .opacity(command_line_arguments.opacity)
            .background_color(command_line_arguments.background_color.clone())
            .subsurface_background_color(command_line_arguments.subsurface_background_color.clone())
            .color_mask(command_line_arguments.color_mask.clone())
            .auto_color_mask(command_line_arguments.auto_color_mask)
            .subsurface_color_mask(command_line_arguments.subsurface_color_mask.clone())
            .auto_subsurface_color_mask(command_line_arguments.auto_subsurface_color_mask)
            .color_mask_tolerance(command_line_arguments.color_mask_tolerance)
            .color_mask_shader(command_line_arguments.color_mask_shader)
            .window_opacity(command_line_arguments.window_opacity)
            .max_fps(command_line_arguments.max_fps)
            .dialog_margin(command_line_arguments.dialog_margin)
            .animation_speed(command_line_arguments.animation_speed)
            .animation_overshoot(command_line_arguments.animation_overshoot)
            .disable_animations(command_line_arguments.disable_animations)
            .debug_touch(command_line_arguments.debug_touch)
            .debug_pointer(command_line_arguments.debug_pointer)
            .override_wayland_display(command_line_arguments.override_wayland_display.clone())
            .keyboard_layout(command_line_arguments.keyboard_layout.clone())
            .keyboard_variant(command_line_arguments.keyboard_variant.clone())
            .command_arguments(command_line_arguments.command_arguments.clone())
            .build()
    }
}
