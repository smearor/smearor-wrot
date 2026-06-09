use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_debug_overlay::DebugOverlayConfig;
use smearor_wrot_debug_overlay::DebugOverlayHandler;
use smearor_wrot_model_geometry::Position;

impl DebugOverlayHandler for CompositorWidget {
    fn update_debug_overlay_config(&self, config: DebugOverlayConfig) {
        self.imp().update_debug_overlay_config(config);
    }

    fn debug_overlay_config(&self) -> DebugOverlayConfig {
        self.imp().debug_overlay_config()
    }

    fn show_touch_overlay(&self) {
        self.imp().show_touch_overlay();
    }

    fn hide_touch_overlay(&self) {
        self.imp().hide_touch_overlay();
    }

    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.imp().update_touch_point(sequence, gtk_position, app_position);
    }

    fn remove_touch_point(&self, sequence: usize) {
        self.imp().remove_touch_point(sequence);
    }

    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>) {
        self.imp().update_pointer_point(gtk_position, app_position);
    }

    fn clear_pointer_point(&self) {
        self.imp().clear_pointer_point();
    }
}
