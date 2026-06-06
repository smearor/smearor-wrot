use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::dmabuf::handler::DmabufHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use smearor_wrot_compositor::dma::count::DmaBufRenderCount;

impl DmabufHandler for CompositorWidgetImpl {
    fn get_dma_buf_render_count(&self) -> u32 {
        let Ok(compositor) = self.compositor() else {
            return 0;
        };
        let Ok(guard) = compositor.lock() else {
            return 0;
        };
        guard.get_dma_buf_render_count()
    }
}
