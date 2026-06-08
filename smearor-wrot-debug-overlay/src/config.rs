use std::sync::atomic::AtomicBool;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct DebugOverlayConfig {
    /// Enable visual debugging of pointer
    #[builder(default)]
    pub debug_pointer: AtomicBool,

    /// Enable visual debugging of touch points
    #[builder(default)]
    pub debug_touch: AtomicBool,
}
