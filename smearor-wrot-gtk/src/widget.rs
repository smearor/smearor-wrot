//! GTK4 compositor widget

use gtk4::glib;
use gtk4::subclass::prelude::*;

glib::wrapper! {
    pub struct CompositorWidget(ObjectSubclass<imp::CompositorWidget>)
        @extends gtk4::Widget, gtk4::Box,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct CompositorWidget;

    #[glib::object_subclass]
    impl ObjectSubclass for CompositorWidget {
        const NAME: &'static str = "CompositorWidget";
        type Type = super::CompositorWidget;
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for CompositorWidget {}
    impl WidgetImpl for CompositorWidget {}
    impl BoxImpl for CompositorWidget {}
}
