use crate::DebugOverlayConfig;
use crate::DebugOverlayHandler;
use crate::PointerPosition;
use dashmap::DashMap;
use smearor_wrot_geometry::Position;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct DebugOverlayManager {
    pub config: Arc<DebugOverlayConfig>,
    /// The pointer point is always rendered in snapshot when available
    pub(crate) pointer_point: RefCell<Option<PointerPosition<f32>>>,
    /// Touch points are always rendered in snapshot when available
    pub(crate) touch_points: DashMap<usize, PointerPosition<f32>>,
}

impl DebugOverlayManager {
    pub fn new(config: Arc<DebugOverlayConfig>) -> Self {
        Self {
            config,
            touch_points: DashMap::new(),
            pointer_point: RefCell::new(None),
        }
    }
}
impl DebugOverlayHandler for DebugOverlayManager {
    fn debug_overlay_config(&self) -> Arc<DebugOverlayConfig> {
        self.config.clone()
    }

    fn is_debug_pointer_enabled(&self) -> bool {
        self.config.debug_pointer.load(Ordering::Relaxed)
    }

    fn show_debug_pointer(&self) {
        self.config.debug_pointer.store(true, Ordering::Release);
    }

    fn hide_debug_pointer(&self) {
        self.config.debug_pointer.store(false, Ordering::Release);
        self.clear_pointer_point();
    }

    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.pointer_point
            .borrow_mut()
            .replace(PointerPosition::new_pointer(gtk_position, app_position));
    }

    fn clear_pointer_point(&self) {
        self.pointer_point.borrow_mut().take();
    }

    fn is_debug_touch_enabled(&self) -> bool {
        self.config.debug_touch.load(Ordering::Relaxed)
    }

    fn show_debug_touch_overlay(&self) {
        self.config.debug_touch.store(true, Ordering::Release);
    }

    fn hide_debug_touch_overlay(&self) {
        self.config.debug_touch.store(false, Ordering::Release);
        self.clear_touch_points();
    }
    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.touch_points.insert(sequence, PointerPosition::new_touch(gtk_position, app_position));
    }

    fn remove_touch_point(&self, sequence: usize) {
        self.touch_points.remove(&sequence);
    }

    fn clear_touch_points(&self) {
        self.touch_points.clear();
    }
}
