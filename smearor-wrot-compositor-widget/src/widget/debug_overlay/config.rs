pub struct DebugOverlayConfig {
    /// Enable visual debugging of pointer
    pub debug_pointer: bool,
    /// Enable visual debugging of touch points
    pub debug_touch: bool,
}

impl Default for DebugOverlayConfig {
    fn default() -> Self {
        Self {
            debug_pointer: false,
            debug_touch: false,
        }
    }
}
