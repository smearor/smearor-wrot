use crate::event_handler::error::MouseInputEventError;
use crate::widget::CompositorWidget;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use smearor_wrot_core::input::mouse::processing::MouseInputProcessing;

/// Trait for handling GTK mouse input events
pub trait MouseInputEventHandler {
    /// Handle mouse press event
    fn handle_mouse_press(&self, button: u32) -> Result<(), MouseInputEventError>;

    /// Handle mouse release event
    fn handle_mouse_release(&self, button: u32) -> Result<(), MouseInputEventError>;

    /// Handle mouse motion event
    fn handle_mouse_motion(&self, x: f64, y: f64) -> Result<(), MouseInputEventError>;

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

    fn handle_mouse_motion(&self, x: f64, y: f64) -> Result<(), MouseInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        let (transformed_x, transformed_y) = self.apply_pointer_transform(x, y);
        compositor.process_gtk_mouse_motion(x, y);
        self.update_pointer_point(x, y, transformed_x, transformed_y);
        Ok(())
    }

    fn handle_mouse_wheel(&self, dx: f64, dy: f64) -> Result<(), MouseInputEventError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_mouse_wheel(dx, dy);
        Ok(())
    }
}
