use crate::widget::debug_overlay::config::DebugOverlayConfig;
use crate::widget::debug_overlay::handler::DebugOverlayHandler;
use dashmap::DashMap;
use smearor_wrot_model::PointerPosition;
use smearor_wrot_model::Position;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct DebugOverlayManager {
    /// If enabled, the pointer is rendered in snapshot when available
    pub debug_pointer: AtomicBool,
    /// The pointer point is always rendered in snapshot when available
    pub(crate) pointer_point: RefCell<Option<PointerPosition<f32>>>,
    /// If enabled, touch points are rendered in snapshot when available
    pub debug_touch: AtomicBool,
    /// Touch points are always rendered in snapshot when available
    pub(crate) touch_points: DashMap<usize, PointerPosition<f32>>,
}

impl Default for DebugOverlayManager {
    fn default() -> Self {
        Self {
            debug_pointer: AtomicBool::new(false),
            touch_points: DashMap::new(),
            debug_touch: AtomicBool::new(false),
            pointer_point: RefCell::new(None),
        }
    }
}

impl DebugOverlayHandler for DebugOverlayManager {
    fn set_debug_overlay_config(&self, config: DebugOverlayConfig) {
        self.debug_pointer.store(config.debug_pointer, Ordering::Relaxed);
        self.debug_touch.store(config.debug_touch, Ordering::Relaxed);
    }

    fn debug_overlay_config(&self) -> DebugOverlayConfig {
        DebugOverlayConfig {
            debug_pointer: self.debug_pointer.load(Ordering::Relaxed),
            debug_touch: self.debug_pointer.load(Ordering::Relaxed),
        }
    }

    fn show_touch_overlay(&self) {
        // Touch points are always rendered in snapshot when available
    }

    fn hide_touch_overlay(&self) {
        // Clear all touch points
        self.touch_points.clear();
    }
    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.touch_points.insert(sequence, PointerPosition::new_touch(gtk_position, app_position));
    }

    fn remove_touch_point(&self, sequence: usize) {
        self.touch_points.remove(&sequence);
    }

    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.pointer_point
            .borrow_mut()
            .replace(PointerPosition::new_pointer(gtk_position, app_position));
    }

    fn clear_pointer_point(&self) {
        self.pointer_point.borrow_mut().take();
    }
}
