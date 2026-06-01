use crate::CompositorWidget;
use crate::event_handler::error::TouchInputEventError;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use smearor_wrot_core::input::touch::processing::TouchInputProcessing;
use tracing::debug;

/// Trait for handling GTK touch input events
pub trait TouchInputEventHandler {
    /// Handle touch down event
    fn handle_touch_down(&self, sequence: usize, x: f64, y: f64) -> Result<(), TouchInputEventError>;
    /// Handle touch up event
    fn handle_touch_up(&self, sequence: usize) -> Result<(), TouchInputEventError>;
    /// Handle touch motion event
    fn handle_touch_motion(&self, sequence: usize, x: f64, y: f64) -> Result<(), TouchInputEventError>;
}

impl TouchInputEventHandler for CompositorWidget {
    fn handle_touch_down(&self, sequence: usize, x: f64, y: f64) -> Result<(), TouchInputEventError> {
        debug!("Touch down event received: sequence {}, x={}, y={}", sequence, x, y);

        // Apply transform callback if set
        let (transformed_x, transformed_y) = self.apply_touch_transform(sequence, x, y);
        debug!(
            "Transformed touch down: sequence {}, original x={}, y={}, transformed x={}, y={}",
            sequence, x, y, transformed_x, transformed_y
        );

        // Update touch point for visual debugging
        self.update_touch_point(sequence, x, y, transformed_x, transformed_y);

        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_touch_down(sequence, x, y);
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

    fn handle_touch_motion(&self, sequence: usize, x: f64, y: f64) -> Result<(), TouchInputEventError> {
        debug!("Touch motion event received: sequence {}, x={}, y={}", sequence, x, y);

        // Apply transform callback if set
        let (transformed_x, transformed_y) = self.apply_touch_transform(sequence, x, y);
        debug!(
            "Transformed touch motion: sequence {}, original x={}, y={}, transformed x={}, y={}",
            sequence, x, y, transformed_x, transformed_y
        );

        // Update touch point for visual debugging
        self.update_touch_point(sequence, x, y, transformed_x, transformed_y);

        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.process_gtk_touch_motion(sequence, x, y);
        debug!("Converted and forwarded touch motion sequence {} to compositor", sequence);
        Ok(())
    }
}
