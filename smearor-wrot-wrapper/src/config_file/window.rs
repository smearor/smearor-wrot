use crate::cli::window::WindowArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

/// Window configuration section
#[derive(Debug, Deserialize, Default)]
pub struct WindowConfigFile {
    /// Aspect ratio
    #[serde(default)]
    pub aspect_ratio: Option<f32>,

    /// Start in fullscreen mode
    #[serde(default)]
    pub fullscreen: Option<bool>,

    /// Initial height
    #[serde(default)]
    pub height: Option<i32>,

    /// Minimum height
    #[serde(default)]
    pub min_height: Option<i32>,

    /// Minimum width
    #[serde(default)]
    pub min_width: Option<i32>,

    /// Maximum height
    #[serde(default)]
    pub max_height: Option<i32>,

    /// Maximum width
    #[serde(default)]
    pub max_width: Option<i32>,

    /// Start in maximized mode
    #[serde(default)]
    pub maximized: Option<bool>,

    /// Whether the window should be resizable
    #[serde(default)]
    pub resizable: Option<bool>,

    /// Whether the window should have decorations
    #[serde(default)]
    pub show_decorations: Option<bool>,

    /// Title of the application window.
    #[serde(default)]
    pub title: Option<String>,

    /// Initial width
    #[serde(default)]
    pub width: Option<i32>,

    /// Window opacity for the compositor window (0.0 = fully transparent, 1.0 = fully opaque).
    #[serde(default)]
    pub window_opacity: Option<f32>,

    /// Initial x position
    #[serde(default)]
    pub x: Option<i32>,

    /// Initial y position
    #[serde(default)]
    pub y: Option<i32>,
}

impl MergeWithConfigFile<WindowConfigFile> for WindowArguments {
    fn merge_with_config_file(mut self, config: &WindowConfigFile) -> Self {
        if self.aspect_ratio.is_none() && config.aspect_ratio.is_some() {
            self.aspect_ratio = config.aspect_ratio.clone();
        }
        if !self.fullscreen
            && let Some(fullscreen) = config.fullscreen
        {
            self.fullscreen = fullscreen;
        }
        if self.height.is_none() && config.height.is_some() {
            self.height = config.height.clone();
        }
        if self.min_height.is_none() && config.min_height.is_some() {
            self.min_height = config.min_height.clone();
        }
        if self.min_width.is_none() && config.min_width.is_some() {
            self.min_width = config.min_width.clone();
        }
        if self.max_height.is_none() && config.max_height.is_some() {
            self.max_height = config.max_height.clone();
        }
        if self.max_width.is_none() && config.max_width.is_some() {
            self.max_width = config.max_width.clone();
        }
        if !self.maximized
            && let Some(maximized) = config.maximized
        {
            self.maximized = maximized;
        }
        if !self.disable_resizable
            && let Some(resizable) = config.resizable
        {
            self.disable_resizable = !resizable;
        }
        if !self.show_decorations
            && let Some(show_decorations) = config.show_decorations
        {
            self.show_decorations = show_decorations;
        }
        if self.title.is_none() && config.title.is_some() {
            self.title = config.title.clone();
        }
        if self.width.is_none() && config.width.is_some() {
            self.width = config.width.clone();
        }
        if self.window_opacity == 1.0
            && let Some(window_opacity) = config.window_opacity
        {
            self.window_opacity = window_opacity;
        }
        if self.x.is_none() && config.x.is_some() {
            self.x = config.xclone();
        }
        if self.y.is_none() && config.y.is_some() {
            self.y = config.y.clone();
        }
        self
    }
}
