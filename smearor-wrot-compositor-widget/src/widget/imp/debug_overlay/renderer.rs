use crate::widget::imp::debug_overlay::manager::DebugOverlayManager;
use gtk4::Snapshot;
use std::sync::atomic::Ordering;

pub trait DebugOverlayRenderer {
    fn snapshot(&self, snapshot: &Snapshot);
    fn render_debug_touch(&self, snapshot: &Snapshot);
    fn render_debug_pointer(&self, snapshot: &Snapshot);
}

impl DebugOverlayRenderer for DebugOverlayManager {
    fn snapshot(&self, snapshot: &Snapshot) {
        if self.debug_pointer.load(Ordering::Relaxed) {
            self.render_debug_pointer(snapshot);
        }
        if self.debug_touch.load(Ordering::Relaxed) {
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
