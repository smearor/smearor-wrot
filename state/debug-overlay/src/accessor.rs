pub trait DebugOverlayStateAccessor {
    /// Returns true, if the debug pointer overlay is enabled
    fn is_debug_pointer_enabled(&self) -> bool;

    /// Enables the debug pointer overlay
    fn enable_debug_pointer(&self);

    /// Disable the debug pointer overlay
    fn disable_debug_pointer(&self);

    /// Returns true, if the debug touch overlay is enabled
    fn is_debug_touch_enabled(&self) -> bool;

    /// Enables the debug touch overlay
    fn enable_debug_touch_overlay(&self);

    /// Disables the debug touch overlay
    fn disable_debug_touch_overlay(&self);
}
