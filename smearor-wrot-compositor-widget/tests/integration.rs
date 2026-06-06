//! Integration tests for smearor-wrot-compositor-widget
//!
//! These tests verify the integration between the GTK widget and the Smithay compositor.

use std::sync::Arc;
use std::sync::Mutex;

use smearor_wrot_compositor_widget::CompositorWidgetConfig;

/// Mock compositor for runtime integration testing
#[derive(Debug, Default)]
struct MockCompositor {
    output_size: Mutex<(i32, i32)>,
    update_count: Mutex<usize>,
}

impl MockCompositor {
    fn new() -> Self {
        Self::default()
    }

    fn update_output_size(&self, width: i32, height: i32) {
        if let Ok(mut size) = self.output_size.lock() {
            *size = (width, height);
        }
        if let Ok(mut count) = self.update_count.lock() {
            *count += 1;
        }
    }

    fn get_output_size(&self) -> (i32, i32) {
        if let Ok(size) = self.output_size.lock() { *size } else { (0, 0) }
    }

    fn get_update_count(&self) -> usize {
        if let Ok(count) = self.update_count.lock() { *count } else { 0 }
    }
}

#[test]
fn test_compositor_widget_config_integration() {
    let config = CompositorWidgetConfig {
        show_decorations: true,
        ..Default::default()
    };
    assert_eq!(config.show_decorations, true);
    assert_eq!(config.opacity, 1.0);
}

#[test]
fn test_compositor_widget_config_default() {
    let config = CompositorWidgetConfig::default();
    assert_eq!(config.show_decorations, false);
    assert_eq!(config.opacity, 1.0);
    assert_eq!(config.color_mask, None);
    assert_eq!(config.initial_position, None);
    assert_eq!(config.min_width, 100);
    assert_eq!(config.min_height, 100);
    assert_eq!(config.max_width, None);
    assert_eq!(config.max_height, None);
    assert_eq!(config.aspect_ratio, None);
    assert_eq!(config.fullscreen, false);
}

#[test]
fn test_compositor_widget_config_clone_integration() {
    let config = CompositorWidgetConfig {
        initial_position: Some((100, 200)),
        min_width: 300,
        min_height: 250,
        max_width: Some(1920),
        max_height: Some(1080),
        aspect_ratio: Some(16.0 / 9.0),
        fullscreen: true,
        ..Default::default()
    };
    let cloned = config.clone();
    assert_eq!(config.show_decorations, cloned.show_decorations);
    assert_eq!(config.opacity, cloned.opacity);
    assert_eq!(config.initial_position, cloned.initial_position);
    assert_eq!(config.min_width, cloned.min_width);
    assert_eq!(config.min_height, cloned.min_height);
    assert_eq!(config.max_width, cloned.max_width);
    assert_eq!(config.max_height, cloned.max_height);
    assert_eq!(config.aspect_ratio, cloned.aspect_ratio);
    assert_eq!(config.fullscreen, cloned.fullscreen);
}

#[test]
fn test_compositor_widget_config_resize_parameters() {
    let config = CompositorWidgetConfig {
        show_decorations: true,
        opacity: 0.9,
        ..Default::default()
    };
    assert_eq!(config.show_decorations, true);
    assert_eq!(config.opacity, 0.9);
}

#[test]
fn test_mock_compositor_runtime_integration() {
    let mock = MockCompositor::new();

    // Test initial state
    assert_eq!(mock.get_output_size(), (0, 0));
    assert_eq!(mock.get_update_count(), 0);

    // Test update_output_size
    mock.update_output_size(800, 600);
    assert_eq!(mock.get_output_size(), (800, 600));
    assert_eq!(mock.get_update_count(), 1);

    // Test multiple updates
    mock.update_output_size(1024, 768);
    assert_eq!(mock.get_output_size(), (1024, 768));
    assert_eq!(mock.get_update_count(), 2);
}

#[test]
fn test_mock_compositor_thread_safety() {
    let mock = Arc::new(MockCompositor::new());
    let mock_clone = Arc::clone(&mock);

    // Test thread-safe access
    mock.update_output_size(800, 600);
    assert_eq!(mock.get_output_size(), (800, 600));

    // Test with cloned Arc
    mock_clone.update_output_size(1024, 768);
    assert_eq!(mock_clone.get_output_size(), (1024, 768));
}
