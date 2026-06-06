use crate::CompositorWidget;
use crate::widget::shutdown::error::ShutdownCheckError;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait ShutdownHandler {
    fn check_and_request_shutdown(&self) -> Result<(), ShutdownCheckError>;
}

impl ShutdownHandler for CompositorWidget {
    fn check_and_request_shutdown(&self) -> Result<(), ShutdownCheckError> {
        self.imp().check_and_request_shutdown()
    }
}
