use crate::SmearorCompositor;
use crate::clipboard::error::CompositorSelectionError;
use smithay::wayland::selection::data_device::set_data_device_selection;
use tracing::debug;

pub trait ClipboardSelectionHandler {
    /// Sets the selection on the seat when host clipboard changes
    fn set_selection_from_host(&self, mime_types: Vec<String>) -> Result<(), CompositorSelectionError>;

    /// This method is called when a Wayland client sets the selection
    /// The extraction is now handled asynchronously in the GTK wrapper using GIO
    fn extract_selection_source_text(&self) -> Result<(), CompositorSelectionError>;
}

impl ClipboardSelectionHandler for SmearorCompositor {
    fn set_selection_from_host(&self, mime_types: Vec<String>) -> Result<(), CompositorSelectionError> {
        debug!("Setting selection from host with mime types: {:?}", mime_types);
        let Ok(display) = self.display.lock() else {
            return Err(CompositorSelectionError::DisplayLock);
        };
        let display_handle = display.handle();
        set_data_device_selection(
            &display_handle,
            &self.seat,
            mime_types,
            (), // TODO: Phase 1 - Clipboard Integration - Provide proper user_data
        );
        Ok(())
    }

    fn extract_selection_source_text(&self) -> Result<(), CompositorSelectionError> {
        debug!("Extracting text from SelectionSource - handled by GTK wrapper");
        // The actual extraction is now handled asynchronously in the GTK wrapper
        // using gio::UnixInputStream to avoid blocking
        Ok(())
    }
}
