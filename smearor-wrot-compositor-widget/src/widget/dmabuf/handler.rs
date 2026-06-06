use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait DmabufHandler {
    fn get_dma_buf_render_count(&self) -> u32;
}

impl DmabufHandler for CompositorWidget {
    fn get_dma_buf_render_count(&self) -> u32 {
        self.imp().get_dma_buf_render_count()
    }
}
