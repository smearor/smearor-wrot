use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait InputEventHandler {
    fn block_input(&self);
    fn unblock_input(&self);
    fn is_input_blocked(&self) -> bool;
}

impl InputEventHandler for CompositorWidget {
    fn block_input(&self) {
        self.imp().block_input();
    }

    fn unblock_input(&self) {
        self.imp().unblock_input();
    }

    fn is_input_blocked(&self) -> bool {
        self.imp().is_input_blocked()
    }
}
