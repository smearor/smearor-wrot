use crate::widget::imp::widget::CompositorWidgetImpl;
use smearor_wrot_debug_overlay::DebugOverlayConfig;
use smearor_wrot_debug_overlay::DebugOverlayHandler;
use smearor_wrot_geometry::Position;

impl DebugOverlayHandler for CompositorWidgetImpl {
    fn update_debug_overlay_config(&self, config: DebugOverlayConfig) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.update_debug_overlay_config(config);
    }

    fn debug_overlay_config(&self) -> DebugOverlayConfig {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return Default::default();
        };
        debug_overlay.debug_overlay_config()
    }

    fn show_touch_overlay(&self) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.show_touch_overlay();
    }

    fn hide_touch_overlay(&self) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.hide_touch_overlay()
    }

    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.update_touch_point(sequence, gtk_position, app_position);
    }

    fn remove_touch_point(&self, sequence: usize) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.remove_touch_point(sequence);
    }

    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.update_pointer_point(gtk_position, app_position);
    }

    fn clear_pointer_point(&self) {
        let Some(debug_overlay) = &*self.debug_overlay.borrow() else {
            return;
        };
        debug_overlay.clear_pointer_point();
    }
}
