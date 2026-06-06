use crate::CompositorError;
use crate::CompositorWidget;
use crate::widget::compositor::error::CompositorInitializationError;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_compositor::SmearorCompositor;
use std::sync::Arc;
use std::sync::Mutex;

pub trait CompositorHandler {
    /// Set the compositor
    fn set_compositor(&self, compositor: Option<Arc<Mutex<SmearorCompositor>>>);

    /// Returns the compositor
    fn compositor(&self) -> Result<Arc<Mutex<SmearorCompositor>>, CompositorError>;

    /// Initializes the compositor
    fn initialize_compositor(&self) -> Result<(), CompositorInitializationError>;
}

impl CompositorHandler for CompositorWidget {
    fn set_compositor(&self, compositor: Option<Arc<Mutex<SmearorCompositor>>>) {
        self.imp().set_compositor(compositor)
    }

    fn compositor(&self) -> Result<Arc<Mutex<SmearorCompositor>>, CompositorError> {
        self.imp().compositor()
    }

    fn initialize_compositor(&self) -> Result<(), CompositorInitializationError> {
        self.imp().initialize_compositor()
    }
}
