use serde::Deserialize;

/// Window configuration section
#[derive(Debug, Deserialize, Default)]
pub struct WindowConfig {
    /// Window title
    #[serde(default)]
    pub title: Option<String>,

    /// Initial width
    #[serde(default)]
    pub width: Option<i32>,

    /// Initial height
    #[serde(default)]
    pub height: Option<i32>,

    /// Whether the window should have decorations
    #[serde(default)]
    pub decorated: Option<bool>,

    /// Whether the window should be resizable
    #[serde(default)]
    pub resizable: Option<bool>,

    /// Initial x position
    #[serde(default)]
    pub position_x: Option<i32>,

    /// Initial y position
    #[serde(default)]
    pub position_y: Option<i32>,

    /// Minimum width
    #[serde(default)]
    pub min_width: Option<i32>,

    /// Minimum height
    #[serde(default)]
    pub min_height: Option<i32>,

    /// Maximum width
    #[serde(default)]
    pub max_width: Option<i32>,

    /// Maximum height
    #[serde(default)]
    pub max_height: Option<i32>,

    /// Aspect ratio
    #[serde(default)]
    pub aspect_ratio: Option<f32>,

    /// Start in fullscreen mode
    #[serde(default)]
    pub fullscreen: Option<bool>,

    /// Start in maximized mode
    #[serde(default)]
    pub maximized: Option<bool>,
}
