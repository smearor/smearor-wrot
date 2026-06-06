use crate::CompositorWidget;
use crate::event_handler::error::TouchInputEventError;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::debug_overlay::handler::DebugOverlayHandler;
use smearor_wrot_core::input::touch::processing::TouchInputProcessing;
use smearor_wrot_model::Position;
use tracing::debug;

/// Trait for handling GTK touch input events
pub trait TouchInputEventHandler {
    /// Handle touch down event
    fn handle_touch_down(&self, sequence: usize, position: Position<f64>) -> Result<(), TouchInputEventError>;
    /// Handle touch up event
    fn handle_touch_up(&self, sequence: usize) -> Result<(), TouchInputEventError>;
    /// Handle touch motion event
    fn handle_touch_motion(&self, sequence: usize, position: Position<f64>) -> Result<(), TouchInputEventError>;
}

impl TouchInputEventHandler for CompositorWidget {
    fn handle_touch_down(&self, sequence: usize, position: Position<f64>) -> Result<(), TouchInputEventError> {
        debug!("Touch down event received: sequence {sequence}, {position}");

        // Apply transform callback if set
        let transformed_position = self.apply_touch_transform(sequence, position);
        debug!("Transformed touch down: sequence={sequence}, original={position}, transformed={transformed_position}");

        // Update touch point for visual debugging
        self.update_touch_point(sequence, position.into(), transformed_position.into());

        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_touch_down(sequence, position);
        debug!("Converted and forwarded touch down sequence {} to compositor", sequence);
        Ok(())
    }

    fn handle_touch_up(&self, sequence: usize) -> Result<(), TouchInputEventError> {
        debug!("Touch up event received: sequence {}", sequence);

        // Remove touch point for visual debugging
        self.remove_touch_point(sequence);

        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_touch_up(sequence);
        debug!("Converted and forwarded touch up sequence {} to compositor", sequence);
        Ok(())
    }

    fn handle_touch_motion(&self, sequence: usize, position: Position<f64>) -> Result<(), TouchInputEventError> {
        debug!("Touch motion event received: sequence={sequence}, position={position}");

        // Apply transform callback if set
        let transformed_position = self.apply_touch_transform(sequence, position);
        debug!("Transformed touch motion: sequence={sequence}, original={position}, transformed={transformed_position}",);

        // Update touch point for visual debugging
        self.update_touch_point(sequence, position.into(), transformed_position.into());

        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_touch_motion(sequence, position);
        debug!("Converted and forwarded touch motion sequence {} to compositor", sequence);
        Ok(())
    }
}
