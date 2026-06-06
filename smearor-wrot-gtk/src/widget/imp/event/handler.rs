use crate::widget::event::handler::InputEventHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use std::sync::atomic::Ordering;

impl InputEventHandler for CompositorWidgetImpl {
    fn block_input(&self) {
        self.input_blocked.store(true, Ordering::Relaxed);
    }

    fn unblock_input(&self) {
        self.input_blocked.store(false, Ordering::Relaxed);
    }

    fn is_input_blocked(&self) -> bool {
        self.input_blocked.load(Ordering::Relaxed)
    }
}
