use crate::widget::config::handler::ConfigHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::subclass::prelude::ObjectSubclassExt;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_core::OutputGeometry;
use smearor_wrot_model::geometry::size::Size;
use std::time::Duration;
use tracing::debug;

pub trait CompositorWidgetResize {
    fn handle_resize_debounced(&self, requested_size: Size<i32>);
}

impl CompositorWidgetResize for CompositorWidgetImpl {
    fn handle_resize_debounced(&self, requested_size: Size<i32>) {
        // Clamp dimensions to prevent negative values
        let min_size = self.min_size();
        let clamped_size = requested_size.max(&min_size);
        debug!("handle resize debounced requested({}) min({}) clamped({})", requested_size, min_size, clamped_size);

        // Store pending resize dimensions
        *self.pending_resize.borrow_mut() = Some(clamped_size);

        // Cancel existing timeout if any
        if let Some(timeout_id) = self.resize_timeout.borrow_mut().take() {
            timeout_id.remove();
        }

        // Set new timeout for debounced resize (100ms)
        let widget_weak = self.obj().downgrade();
        let timeout_id = glib::timeout_add_local_once(Duration::ZERO, move || {
            let Some(widget) = widget_weak.upgrade() else {
                return;
            };
            let imp = widget.imp();
            *imp.resize_timeout.borrow_mut() = None;
            let Some(pending_size) = imp.pending_resize.borrow_mut().take() else {
                return;
            };
            // Update compositor output size (sends configure events to application)
            if let Some(compositor_mutex) = imp.compositor.borrow().clone() {
                if let Ok(mut compositor) = compositor_mutex.lock() {
                    compositor.update_output_size(pending_size);
                }
            }

            // Do NOT update application window size directly
            // Wait for the application to send a new buffer with the new size
            // The rendering surface will be resized when the new buffer arrives

            // Trigger redraw after resize
            widget.queue_resize();
            // widget.request_render();
        });

        *self.resize_timeout.borrow_mut() = Some(timeout_id);
    }
}
