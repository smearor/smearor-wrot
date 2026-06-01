//! Configuration for compositor widgets

/// Configuration for the CompositorWidget
///
/// This struct defines the layout and spacing parameters for the compositor widget.
/// It can be used to customize the widget's appearance and spacing behavior.
#[derive(Debug, Clone)]
pub struct CompositorWidgetConfig {
    /// Spacing between child widgets in pixels
    pub spacing: i32,
    /// Top margin in pixels
    pub margin_top: i32,
    /// Bottom margin in pixels
    pub margin_bottom: i32,
    /// Start margin (left in LTR, right in RTL) in pixels
    pub margin_start: i32,
    /// End margin (right in LTR, left in RTL) in pixels
    pub margin_end: i32,
    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque)
    pub opacity: f32,
    /// Color mask for background replacement (None = use original color)
    pub color_mask: Option<ColorMask>,
    /// Enable window decorations (title bar, borders, etc.)
    pub show_decorations: bool,
    /// Initial window position (x, y) in pixels
    pub initial_position: Option<(i32, i32)>,
    /// Minimum window width in pixels
    pub min_width: i32,
    /// Minimum window height in pixels
    pub min_height: i32,
    /// Maximum window width in pixels (None = no limit)
    pub max_width: Option<i32>,
    /// Maximum window height in pixels (None = no limit)
    pub max_height: Option<i32>,
    /// Aspect ratio as width/height (None = no constraint)
    pub aspect_ratio: Option<f32>,
    /// Start in fullscreen mode
    pub fullscreen: bool,
    /// Initial window width in pixels
    pub initial_width: i32,
    /// Initial window height in pixels
    pub initial_height: i32,
    /// Title for the header bar (None = sync with application window title)
    pub title: Option<String>,
    /// Enable DMA-BUF hardware acceleration
    pub dma_buf_enabled: bool,
    /// Enable visual debugging of touch points
    pub debug_touch: bool,
    /// Enable visual debugging of pointer
    pub debug_pointer: bool,
    /// Enable automatic background color detection for color mask
    pub auto_color_mask: bool,
    /// Enable automatic background color detection for subsurface color mask
    pub auto_subsurface_color_mask: bool,
    /// Color mask tolerance for color matching (0.0-1.0)
    pub color_mask_tolerance: f32,
    /// Whether the window should be resizable
    pub resizable: bool,
    /// Disable client-side decorations for Wayland clients in the compositor
    pub disable_client_decorations: bool,
    /// Enable shader-based color masking for better performance
    pub color_mask_shader: bool,
    /// Enable animations for visual effects
    pub animations: bool,
}

/// Color mask configuration for background replacement
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorMask {
    /// Red component (0.0-1.0)
    pub red: f32,
    /// Green component (0.0-1.0)
    pub green: f32,
    /// Blue component (0.0-1.0)
    pub blue: f32,
}

/// Default configuration for CompositorWidget
///
/// Returns a configuration with sensible defaults:
/// - spacing: 6 pixels
/// - margins: 12 pixels on all sides
/// - opacity: 1.0 (fully opaque)
/// - color_mask: None (no color masking)
/// - show_decorations: false (decorations disabled by default)
/// - initial_position: None (use default window positioning)
/// - min_width: 100 pixels (minimum window width)
/// - min_height: 100 pixels (minimum window height)
/// - max_width: None (no maximum width constraint)
/// - max_height: None (no maximum height constraint)
/// - aspect_ratio: None (no aspect ratio constraint)
/// - fullscreen: false (start in normal mode)
/// - title: None (sync with application window title)
impl Default for CompositorWidgetConfig {
    fn default() -> Self {
        Self {
            spacing: 0,
            margin_top: 0,
            margin_bottom: 0,
            margin_start: 0,
            margin_end: 0,
            opacity: 1.0,
            color_mask: None,
            show_decorations: false,
            initial_position: None,
            min_width: 100,
            min_height: 100,
            max_width: None,
            max_height: None,
            aspect_ratio: None,
            fullscreen: false,
            initial_width: 1920,
            initial_height: 1080,
            title: None,
            dma_buf_enabled: true,
            debug_touch: false,
            debug_pointer: false,
            auto_color_mask: false,
            auto_subsurface_color_mask: false,
            color_mask_tolerance: 0.1,
            resizable: true,
            disable_client_decorations: false,
            color_mask_shader: false,
            animations: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_widget_config_default() {
        let config = CompositorWidgetConfig::default();
        assert_eq!(config.spacing, 6);
        assert_eq!(config.margin_top, 12);
        assert_eq!(config.margin_bottom, 12);
        assert_eq!(config.margin_start, 12);
        assert_eq!(config.margin_end, 12);
        assert_eq!(config.opacity, 1.0);
        assert_eq!(config.color_mask, None);
        assert_eq!(config.show_decorations, false);
        assert_eq!(config.initial_position, None);
        assert_eq!(config.min_width, 100);
        assert_eq!(config.min_height, 100);
        assert_eq!(config.max_width, None);
        assert_eq!(config.max_height, None);
        assert_eq!(config.aspect_ratio, None);
        assert_eq!(config.fullscreen, false);
    }

    #[test]
    fn test_compositor_widget_config_decorations_toggle() {
        let mut config = CompositorWidgetConfig::default();
        assert_eq!(config.show_decorations, false);

        config.show_decorations = true;
        assert_eq!(config.show_decorations, true);
    }

    #[test]
    fn test_compositor_widget_config_position() {
        let config = CompositorWidgetConfig::default();
        assert_eq!(config.initial_position, None);

        let config_with_position = CompositorWidgetConfig {
            initial_position: Some((100, 200)),
            ..Default::default()
        };
        assert_eq!(config_with_position.initial_position, Some((100, 200)));
    }

    #[test]
    fn test_compositor_widget_config_min_size() {
        let config = CompositorWidgetConfig::default();
        assert_eq!(config.min_width, 100);
        assert_eq!(config.min_height, 100);

        let config_with_min_size = CompositorWidgetConfig {
            min_width: 300,
            min_height: 250,
            ..Default::default()
        };
        assert_eq!(config_with_min_size.min_width, 300);
        assert_eq!(config_with_min_size.min_height, 250);
    }

    #[test]
    fn test_compositor_widget_config_max_size() {
        let config = CompositorWidgetConfig::default();
        assert_eq!(config.max_width, None);
        assert_eq!(config.max_height, None);

        let config_with_max_size = CompositorWidgetConfig {
            max_width: Some(1920),
            max_height: Some(1080),
            ..Default::default()
        };
        assert_eq!(config_with_max_size.max_width, Some(1920));
        assert_eq!(config_with_max_size.max_height, Some(1080));
    }

    #[test]
    fn test_compositor_widget_config_aspect_ratio() {
        let config = CompositorWidgetConfig::default();
        assert_eq!(config.aspect_ratio, None);

        let config_with_aspect = CompositorWidgetConfig {
            aspect_ratio: Some(16.0 / 9.0),
            ..Default::default()
        };
        assert_eq!(config_with_aspect.aspect_ratio, Some(16.0 / 9.0));
    }
}
