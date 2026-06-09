use crate::DebugOverlayManager;
use crate::PointerPosition;
use dashmap::DashMap;
use smearor_wrot_model_geometry::Position;
use smearor_wrot_state_debug_overlay::DebugOverlayStateManager;
use smearor_wrot_state_debug_overlay::accessor::DebugOverlayStateAccessor;
use std::sync::Arc;
use std::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct DebugOverlayManagerImpl {
    /// The debug overlay state manager
    pub(crate) state_manager: Arc<DebugOverlayStateManager>,

    /// The pointer point is always rendered in snapshot when available
    pub(crate) pointer_point: RwLock<Option<PointerPosition<f32>>>,

    /// Touch points are always rendered in snapshot when available
    pub(crate) touch_points: DashMap<usize, PointerPosition<f32>>,
}

impl DebugOverlayManagerImpl {
    pub fn new(state_manager: Arc<DebugOverlayStateManager>) -> Self {
        Self {
            state_manager,
            touch_points: DashMap::new(),
            pointer_point: RwLock::new(None),
        }
    }
}
impl DebugOverlayManager for DebugOverlayManagerImpl {
    fn is_debug_pointer_enabled(&self) -> bool {
        self.state_manager.is_debug_pointer_enabled()
    }

    fn show_debug_pointer(&self) {
        self.state_manager.enable_debug_pointer()
    }

    fn hide_debug_pointer(&self) {
        self.state_manager.disable_debug_pointer();
        self.clear_pointer_point();
    }

    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>) {
        if let Ok(mut pointer_point) = self.pointer_point.write() {
            pointer_point.replace(PointerPosition::new_pointer(gtk_position, app_position));
        }
    }

    fn clear_pointer_point(&self) {
        if let Ok(mut pointer_point) = self.pointer_point.write() {
            pointer_point.take();
        }
    }

    fn is_debug_touch_enabled(&self) -> bool {
        self.state_manager.is_debug_touch_enabled()
    }

    fn show_debug_touch_overlay(&self) {
        self.state_manager.enable_debug_touch_overlay()
    }

    fn hide_debug_touch_overlay(&self) {
        self.state_manager.disable_debug_touch_overlay();
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
