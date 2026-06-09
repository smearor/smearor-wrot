use crate::CompositorWidget;
use crate::widget::imp::resize::handler::CompositorWidgetResize;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_model_geometry::Size;

pub trait ResizeHandler {
    fn handle_resize(&self, requested_size: Size<i32>);
}

impl ResizeHandler for CompositorWidget {
    fn handle_resize(&self, requested_size: Size<i32>) {
        self.imp().handle_resize_debounced(requested_size);
    }
}
