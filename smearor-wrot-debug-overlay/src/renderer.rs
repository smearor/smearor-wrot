use crate::DebugOverlayHandler;
use crate::DebugOverlayManager;
use gtk4::Snapshot;

pub trait DebugOverlayRenderer {
    fn snapshot(&self, snapshot: &Snapshot);
    fn render_debug_touch(&self, snapshot: &Snapshot);
    fn render_debug_pointer(&self, snapshot: &Snapshot);
}

impl DebugOverlayRenderer for DebugOverlayManager {
    fn snapshot(&self, snapshot: &Snapshot) {
        if self.is_debug_pointer_enabled() {
            self.render_debug_pointer(snapshot);
        }
        if self.is_debug_touch_enabled() {
            self.render_debug_touch(snapshot);
        }
    }

    fn render_debug_touch(&self, snapshot: &Snapshot) {
        for touch_point in self.touch_points.iter() {
            touch_point.render_snapshot(snapshot);
        }
    }

    fn render_debug_pointer(&self, snapshot: &Snapshot) {
        let pointer_point = self.pointer_point.borrow();
        if let Some(pointer_point) = pointer_point.as_ref() {
            pointer_point.render_snapshot(snapshot);
        }
    }
}
