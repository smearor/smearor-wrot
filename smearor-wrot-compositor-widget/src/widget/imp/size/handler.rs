use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::size::handler::WidgetSizeHandler;
use glib::subclass::prelude::ObjectSubclassExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_geometry::Size;

impl WidgetSizeHandler for CompositorWidgetImpl {
    fn widget_size(&self) -> Size<i32> {
        Size::new(self.obj().width(), self.obj().height())
    }
}
