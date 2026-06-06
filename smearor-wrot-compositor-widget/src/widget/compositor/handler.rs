use crate::CompositorError;
use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_core::SmearorCompositor;
use std::sync::Arc;
use std::sync::Mutex;

pub trait CompositorHandler {
    /// Set the compositor
    fn set_compositor(&self, compositor: Option<Arc<Mutex<SmearorCompositor>>>);

    fn compositor(&self) -> Result<Arc<Mutex<SmearorCompositor>>, CompositorError>;
}

impl CompositorHandler for CompositorWidget {
    fn set_compositor(&self, compositor: Option<Arc<Mutex<SmearorCompositor>>>) {
        self.imp().set_compositor(compositor)
    }

    fn compositor(&self) -> Result<Arc<Mutex<SmearorCompositor>>, CompositorError> {
        self.imp().compositor()
    }
}
