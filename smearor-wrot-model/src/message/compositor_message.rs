/// Messages sent from compositor core to GTK wrapper
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompositorMessage {
    /// Request to maximize the compositor window
    Maximize,
    /// Request to unmaximize the compositor window
    Unmaximize,
    /// Request to minimize the compositor window
    Minimize,
    /// Request to fullscreen the compositor window
    Fullscreen,
    /// Request to unfullscreen the compositor window
    Unfullscreen,
    /// Request to resize the compositor window
    Resize(i32, i32),
    /// Request to shutdown the compositor when all clients are closed
    Shutdown,
    /// Title of the active client window changed
    TitleChanged(String),
    /// Icon of the active client window changed
    AppIdChanged(String),
    /// A window was mapped (for rotation widget to update size)
    WindowMapped,
    /// First commit received from application (to show window)
    FirstCommit,
    /// Wayland selection changed, extract and sync to host clipboard
    WaylandSelectionChanged,
    /// Client requested to move window (drag by title bar)
    MoveRequest(u32),
    /// Client requested to resize window (drag by edge)
    ResizeRequest(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_message_clone() {
        let msg = CompositorMessage::TitleChanged("test".to_string());
        let cloned = msg.clone();
        match cloned {
            CompositorMessage::TitleChanged(title) => assert_eq!(title, "test"),
            _ => panic!("Expected TitleChanged variant"),
        }
    }

    #[test]
    fn test_compositor_message_debug() {
        let msg = CompositorMessage::Resize(800, 600);
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Resize"));
        assert!(debug_str.contains("800"));
        assert!(debug_str.contains("600"));
    }

    #[test]
    fn test_compositor_message_variant_maximize() {
        let msg = CompositorMessage::Maximize;
        let debug_str = format!("{:?}", msg);
        assert_eq!(debug_str, "Maximize");
    }

    #[test]
    fn test_compositor_message_variant_shutdown() {
        let msg = CompositorMessage::Shutdown;
        let debug_str = format!("{:?}", msg);
        assert_eq!(debug_str, "Shutdown");
    }

    #[test]
    fn test_compositor_message_variant_move_request() {
        let msg = CompositorMessage::MoveRequest(42);
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("42"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_compositor_message_serde_roundtrip() {
        let msg = CompositorMessage::TitleChanged("hello".to_string());
        let toml_str = toml::to_string(&msg).unwrap();
        let deserialized: CompositorMessage = toml::from_str(&toml_str).unwrap();
        match deserialized {
            CompositorMessage::TitleChanged(title) => assert_eq!(title, "hello"),
            _ => panic!("Expected TitleChanged variant"),
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_compositor_message_serde_resize_roundtrip() {
        let msg = CompositorMessage::Resize(800, 600);
        let toml_str = toml::to_string(&msg).unwrap();
        let deserialized: CompositorMessage = toml::from_str(&toml_str).unwrap();
        match deserialized {
            CompositorMessage::Resize(w, h) => {
                assert_eq!(w, 800);
                assert_eq!(h, 600);
            }
            _ => panic!("Expected Resize variant"),
        }
    }
}
