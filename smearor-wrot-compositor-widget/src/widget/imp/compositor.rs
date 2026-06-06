use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use smearor_wrot_compositor::SmearorCompositor;
use std::sync::Arc;
use std::sync::Mutex;

impl CompositorHandler for CompositorWidgetImpl {
    fn set_compositor(&self, compositor: Option<Arc<Mutex<SmearorCompositor>>>) {
        *self.compositor.borrow_mut() = compositor;
    }

    fn compositor(&self) -> Result<Arc<Mutex<SmearorCompositor>>, CompositorError> {
        self.compositor.borrow().clone().ok_or(CompositorError::CompositorNotFound)
    }
}
