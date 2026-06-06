use crate::widget::debug_overlay::config::DebugOverlayConfig;
use crate::widget::debug_overlay::handler::DebugOverlayHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use smearor_wrot_model::Position;

impl DebugOverlayHandler for CompositorWidgetImpl {
    fn set_debug_overlay_config(&self, config: DebugOverlayConfig) {
        self.debug_overlay.set_debug_overlay_config(config);
    }

    fn debug_overlay_config(&self) -> DebugOverlayConfig {
        self.debug_overlay.debug_overlay_config()
    }

    fn show_touch_overlay(&self) {
        self.debug_overlay.show_touch_overlay();
    }

    fn hide_touch_overlay(&self) {
        self.debug_overlay.hide_touch_overlay()
    }

    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.debug_overlay.update_touch_point(sequence, gtk_position, app_position);
    }

    fn remove_touch_point(&self, sequence: usize) {
        self.debug_overlay.remove_touch_point(sequence);
    }

    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.debug_overlay.update_pointer_point(gtk_position, app_position);
    }

    fn clear_pointer_point(&self) {
        self.debug_overlay.clear_pointer_point();
    }
}
