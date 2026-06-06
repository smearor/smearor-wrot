use crate::event_handler::error::MouseInputEventError;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::debug_overlay::handler::DebugOverlayHandler;
use crate::widget::widget::CompositorWidget;
use smearor_wrot_core::input::mouse::processing::MouseInputProcessing;
use smearor_wrot_model::Position;

/// Trait for handling GTK mouse input events
pub trait MouseInputEventHandler {
    /// Handle mouse press event
    fn handle_mouse_press(&self, button: u32) -> Result<(), MouseInputEventError>;

    /// Handle mouse release event
    fn handle_mouse_release(&self, button: u32) -> Result<(), MouseInputEventError>;

    /// Handle mouse motion event
    fn handle_mouse_motion(&self, position: Position<f64>) -> Result<(), MouseInputEventError>;

    /// Handle mouse wheel scroll event
    fn handle_mouse_wheel(&self, dx: f64, dy: f64) -> Result<(), MouseInputEventError>;
}

impl MouseInputEventHandler for CompositorWidget {
    fn handle_mouse_press(&self, button: u32) -> Result<(), MouseInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_mouse_press(button);
        Ok(())
    }

    fn handle_mouse_release(&self, button: u32) -> Result<(), MouseInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_mouse_release(button);
        self.clear_pointer_point();
        Ok(())
    }

    fn handle_mouse_motion(&self, position: Position<f64>) -> Result<(), MouseInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        let transformed_position = self.apply_pointer_transform(position);
        compositor.process_gtk_mouse_motion(position);
        self.update_pointer_point(position.into(), transformed_position.into());
        Ok(())
    }

    fn handle_mouse_wheel(&self, dx: f64, dy: f64) -> Result<(), MouseInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_mouse_wheel(dx, dy);
        Ok(())
    }
}
