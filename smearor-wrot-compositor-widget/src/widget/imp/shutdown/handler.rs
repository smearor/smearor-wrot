use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::shutdown::error::ShutdownCheckError;
use smearor_wrot_compositor::lifecycle::shutdown::ShutdownHandler;

impl crate::widget::shutdown::handler::ShutdownHandler for CompositorWidgetImpl {
    fn check_and_request_shutdown(&self) -> Result<(), ShutdownCheckError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.check_and_request_shutdown();
        Ok(())
    }
}
