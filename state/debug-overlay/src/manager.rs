use crate::DebugOverlayState;
use crate::accessor::DebugOverlayStateAccessor;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct DebugOverlayStateManager {
    pub state: Arc<DebugOverlayState>,
}

impl DebugOverlayStateManager {
    pub fn new(state: Arc<DebugOverlayState>) -> Self {
        Self { state }
    }
}
impl DebugOverlayStateAccessor for DebugOverlayStateManager {
    fn is_debug_pointer_enabled(&self) -> bool {
        self.state.debug_pointer.load(Ordering::Relaxed)
    }

    fn enable_debug_pointer(&self) {
        self.state.debug_pointer.store(true, Ordering::Relaxed);
    }

    fn disable_debug_pointer(&self) {
        self.state.debug_pointer.store(false, Ordering::Relaxed);
    }

    fn is_debug_touch_enabled(&self) -> bool {
        self.state.debug_touch.load(Ordering::Relaxed)
    }

    fn enable_debug_touch_overlay(&self) {
        self.state.debug_touch.store(true, Ordering::Relaxed);
    }

    fn disable_debug_touch_overlay(&self) {
        self.state.debug_touch.store(false, Ordering::Relaxed);
    }
}
