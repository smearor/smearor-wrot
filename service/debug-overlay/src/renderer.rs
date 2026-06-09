use crate::DebugOverlayManager;
use crate::manager::DebugOverlayManagerImpl;
use gtk4::Snapshot;
use std::ops::Deref;

pub trait DebugOverlayRenderer {
    fn snapshot(&self, snapshot: &Snapshot);
    fn render_debug_touch(&self, snapshot: &Snapshot);
    fn render_debug_pointer(&self, snapshot: &Snapshot);
}

impl DebugOverlayRenderer for DebugOverlayManagerImpl {
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
        if let Ok(pointer_point) = self.pointer_point.read() {
            if let Some(pointer_point) = pointer_point.deref() {
                pointer_point.render_snapshot(snapshot);
            }
        }
    }
}
