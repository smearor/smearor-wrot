use crate::SmearorCompositor;
use std::sync::atomic::Ordering;

pub trait DmaBufRenderCount {
    /// Increment DMA-BUF render count
    fn increment_dma_buf_render_count(&self);

    /// Get DMA-BUF render count
    fn get_dma_buf_render_count(&self) -> u32;
}

impl DmaBufRenderCount for SmearorCompositor {
    fn increment_dma_buf_render_count(&self) {
        self.dma_buf_render_count.fetch_add(1, Ordering::Relaxed);
    }

    fn get_dma_buf_render_count(&self) -> u32 {
        self.dma_buf_render_count.load(Ordering::Relaxed)
    }
}
