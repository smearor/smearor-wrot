use crate::CompositorWidget;
use crate::event_handler::error::KeyboardInputEventError;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use gtk4::gdk;
use smearor_wrot_core::input::keyboard::processing::KeyboardInputProcessing;

/// Trait for handling GTK keyboard input events
pub trait KeyboardInputEventHandler {
    /// Handle key press event
    fn handle_key_press(&self, keyval: gdk::Key, keycode: u32) -> Result<(), KeyboardInputEventError>;

    /// Handle key release event
    fn handle_key_release(&self, keyval: gdk::Key, keycode: u32) -> Result<(), KeyboardInputEventError>;
}

impl KeyboardInputEventHandler for CompositorWidget {
    fn handle_key_press(&self, _keyval: gdk::Key, keycode: u32) -> Result<(), KeyboardInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_key_press(keycode);
        Ok(())
    }

    fn handle_key_release(&self, _keyval: gdk::Key, keycode: u32) -> Result<(), KeyboardInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_key_release(keycode);
        Ok(())
    }
}
