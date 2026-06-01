/// Messages sent from pie menu to main application
#[derive(Debug, Clone)]
pub enum PieMenuMessage {
    /// Rotate clockwise
    RotateCw,
    /// Rotate counter-clockwise
    RotateCcw,
    /// Rotate to
    Rotate(f32),
    /// Open settings dialog
    Settings,
    /// Take screenshot
    Screenshot,
    /// Exit application
    Exit,
    /// Toggle maximize
    ToggleMaximize,
    /// Minimize
    Minimize,
    /// Toggle fullscreen
    ToggleFullscreen,
    /// Custom event with name
    Custom(String),
}

impl From<&str> for PieMenuMessage {
    fn from(event: &str) -> Self {
        match event.to_lowercase().as_str() {
            "rotate-cw" => Self::RotateCw,
            "rotate-ccw" => Self::RotateCcw,
            "settings" => Self::Settings,
            "screenshot" => Self::Screenshot,
            "exit" => Self::Exit,
            "toggle-maximize" => Self::ToggleMaximize,
            "minimize" => Self::Minimize,
            "toggle-fullscreen" => Self::ToggleFullscreen,
            _ => Self::Custom(event.to_string()),
        }
    }
}
