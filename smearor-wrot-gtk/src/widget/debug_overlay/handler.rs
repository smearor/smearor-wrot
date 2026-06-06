use crate::CompositorWidget;
use crate::widget::debug_overlay::config::DebugOverlayConfig;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_model::Position;

pub trait DebugOverlayHandler {
    /// Sets the debug overlay configuration.
    fn set_debug_overlay_config(&self, config: DebugOverlayConfig);

    /// Returns the debug overlay configuration.
    fn debug_overlay_config(&self) -> DebugOverlayConfig;

    // Shows the touch overlay.
    fn show_touch_overlay(&self);

    /// Hides the touch overlay.
    fn hide_touch_overlay(&self);
    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>);
    fn remove_touch_point(&self, sequence: usize);
    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>);
    fn clear_pointer_point(&self);
}

impl DebugOverlayHandler for CompositorWidget {
    fn set_debug_overlay_config(&self, config: DebugOverlayConfig) {
        self.imp().set_debug_overlay_config(config);
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
