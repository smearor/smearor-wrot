use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_model_geometry::Size;

pub trait WidgetSizeHandler {
    fn widget_size(&self) -> Size<i32>;
}

impl WidgetSizeHandler for CompositorWidget {
    fn widget_size(&self) -> Size<i32> {
        self.imp().widget_size()
    }
}
