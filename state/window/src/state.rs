use atomic_float::AtomicF32;
use smearor_wrot_model_geometry::Position;
use smearor_wrot_model_geometry::Size;
use std::sync::atomic::AtomicBool;
use typed_builder::TypedBuilder;

/// Configuration for the (outer) application window
#[derive(Debug, TypedBuilder)]
pub struct WindowState {
    /// Aspect ratio as width/height (None = no constraint)
    pub aspect_ratio: Option<AtomicF32>,

    /// Start in fullscreen mode
    pub fullscreen: AtomicBool,

    /// Initial window position (None = center)
    pub initial_position: Option<Position<i32>>,

    /// Initial window width in pixels
    pub initial_size: Option<Size<i32>>,

    /// Maximum window size in pixels (None = no limit)
    pub max_size: Option<Size<i32>>,

    /// Start in maximized mode
    pub maximized: AtomicBool,

    /// Minimum window size in pixels
    pub min_size: Option<Size<i32>>,

    /// Whether the window should be resizable
    pub resizable: AtomicBool,

    /// Show window decorations
    pub show_decorations: AtomicBool,

    /// Title for the header bar (None = sync with application window title)
    pub title: Option<String>,

    pub window_opacity: AtomicF32,
}
