use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::shm::handler::ShmHandler;
use smearor_wrot_compositor::render::count::ShmRenderCount;

impl ShmHandler for CompositorWidgetImpl {
    fn get_shm_render_count(&self) -> u32 {
        let Ok(compositor) = self.compositor() else {
            return 0;
        };
        let Ok(guard) = compositor.lock() else {
            return 0;
        };
        guard.get_shm_render_count()
    }
}
