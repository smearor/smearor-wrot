use crate::clipboard::error::CompositorClipboardError;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::CompositorWidgetImpl;
use smearor_wrot_core::clipboard::handler::ClipboardSelectionHandler;

pub trait ClipboardHandler {
    fn set_clipboard_content(&self, content: Option<String>) -> Result<(), CompositorClipboardError>;

    fn get_clipboard_content(&self) -> Result<Option<String>, CompositorClipboardError>;

    fn set_selection_from_host(&self, mime_types: Vec<String>) -> Result<(), CompositorClipboardError>;
}

impl ClipboardHandler for CompositorWidgetImpl {
    fn set_clipboard_content(&self, content: Option<String>) -> Result<(), CompositorClipboardError> {
        let compositor = self.compositor()?;
        let compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        let mut clipboard_content = compositor
            .clipboard_content
            .lock()
            .map_err(|_| CompositorClipboardError::ClipboardContentLockError)?;
        *clipboard_content = content;
        Ok(())
    }

    fn get_clipboard_content(&self) -> Result<Option<String>, CompositorClipboardError> {
        let compositor = self.compositor()?;
        let compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        let clipboard_content = compositor
            .clipboard_content
            .lock()
            .map_err(|_| CompositorClipboardError::ClipboardContentLockError)?;
        Ok(clipboard_content.clone())
    }

    fn set_selection_from_host(&self, mime_types: Vec<String>) -> Result<(), CompositorClipboardError> {
        let compositor = self.compositor()?;
        let compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor
            .set_selection_from_host(mime_types)
            .map_err(CompositorClipboardError::CompositorSelectionError)
    }
}
