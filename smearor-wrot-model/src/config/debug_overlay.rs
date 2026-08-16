/// Configuration for debug overlay rendering
#[derive(Default)]
pub struct DebugOverlayConfig {
    /// Enable visual debugging of pointer
    pub debug_pointer: bool,
    /// Enable visual debugging of touch points
    pub debug_touch: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_overlay_config_default() {
        let config = DebugOverlayConfig::default();
        assert!(!config.debug_pointer);
        assert!(!config.debug_touch);
    }
}
