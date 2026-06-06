use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::window_state::WindowStateHandler;
use crate::widget::window_state::error::ChangeWindowStateError;
use smearor_wrot_compositor::windows::WindowState;

impl WindowStateHandler for CompositorWidgetImpl {
    fn toggle_maximize_first_toplevel(&self) -> Result<(), ChangeWindowStateError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.toggle_maximize_first_toplevel();
        Ok(())
    }

    fn toggle_fullscreen_first_toplevel(&self) -> Result<(), ChangeWindowStateError> {
        let compositor = self.compositor()?;
        let mut compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.toggle_fullscreen_first_toplevel();
        Ok(())
    }
}
