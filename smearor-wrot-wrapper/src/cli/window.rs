use atomic_float::AtomicF32;
use clap::Parser;
use smearor_wrot_application::Position;
use smearor_wrot_application::Size;
use smearor_wrot_application::WindowState;
use std::sync::atomic::AtomicBool;

#[derive(Parser, Debug, Clone)]
pub struct WindowArguments {
    /// Aspect ratio as width/height (e.g., 1.777 for 16:9).
    #[arg(long)]
    pub(crate) aspect_ratio: Option<f32>,

    /// Start in fullscreen mode.
    #[arg(short = 'f', long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) fullscreen: bool,

    /// Initial height of the application window.
    #[arg(short = 'H', long)]
    pub(crate) height: Option<i32>,

    /// Minimum height of the window.
    #[arg(long)]
    pub(crate) min_height: Option<i32>,

    /// Minimum width of the window.
    #[arg(long)]
    pub(crate) min_width: Option<i32>,

    /// Maximum height of the window.
    #[arg(long, requires = "min-height")]
    pub(crate) max_height: Option<i32>,

    /// Maximum width of the window.
    #[arg(long, requires = "min-width")]
    pub(crate) max_width: Option<i32>,

    /// Start in maximized mode.
    #[arg(short = 'm', long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) maximized: bool,

    /// Whether the window should be resizable.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_resizable: bool,

    /// Whether the window should have decorations.
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) show_decorations: bool,

    /// Title of the application window.
    #[arg(short = 't', long)]
    pub(crate) title: Option<String>,

    /// Initial width of the application window.
    #[arg(short = 'W', long)]
    pub(crate) width: Option<i32>,

    /// Window opacity for the compositor window (0.0 = fully transparent, 1.0 = fully opaque).
    #[arg(long, default_value_t = 1.0, min_value = 0.0, max_value = 1.0)]
    pub(crate) window_opacity: f32,

    /// Initial x position of the window.
    #[arg(short = 'x', long)]
    pub(crate) x: Option<i32>,

    /// Initial y position of the window.
    #[arg(short = 'y', long)]
    pub(crate) y: Option<i32>,
}

impl From<WindowArguments> for WindowState {
    fn from(args: WindowArguments) -> Self {
        Self {
            aspect_ratio: args.aspect_ratio.map(|aspect_ratio| AtomicF32::new(aspect_ratio)),
            fullscreen: AtomicBool::new(args.fullscreen),
            initial_position: if let (Some(x), Some(y)) = (args.x, args.y) {
                Some(Position::new(x, y))
            } else {
                None
            },
            initial_size: if let (Some(width), Some(height)) = (args.width, args.height) {
                Some(Size::new(width, height))
            } else {
                None
            },
            max_size: if let (Some(max_width), Some(max_height)) = (args.max_width, args.max_height) {
                Some(Size::new(max_width, max_height))
            } else {
                None
            },
            maximized: AtomicBool::new(args.maximized),
            min_size: if let (Some(min_width), Some(min_height)) = (args.min_width, args.min_height) {
                Some(Size::new(min_width, min_height))
            } else {
                None
            },
            resizable: AtomicBool::new(args.resizable),
            show_decorations: AtomicBool::new(args.show_decorations),
            title: args.title,
            window_opacity: AtomicF32::new(args.window_opacity),
        }
    }
}
