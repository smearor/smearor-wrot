use crate::CompositorWidget;
use crate::widget::buffer::error::SaveBufferError;
use glib::subclass::prelude::ObjectSubclassIsExt;
use std::path::PathBuf;

pub trait BufferHandler {
    fn save_buffer_to_png<P: Into<PathBuf>>(&self, path: P) -> Result<PathBuf, SaveBufferError>;
}

impl BufferHandler for CompositorWidget {
    fn save_buffer_to_png<P: Into<PathBuf>>(&self, path: P) -> Result<PathBuf, SaveBufferError> {
        self.imp().save_buffer_to_png(path)
    }
}
