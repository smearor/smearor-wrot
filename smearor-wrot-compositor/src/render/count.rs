use crate::SmearorCompositor;
use std::sync::atomic::Ordering;

pub trait ShmRenderCount {
    /// Increment SHM render count
    fn increment_shm_render_count(&self);

    /// Get SHM render count
    fn get_shm_render_count(&self) -> u32;
}

impl ShmRenderCount for SmearorCompositor {
    fn increment_shm_render_count(&self) {
        self.shm_render_count.fetch_add(1, Ordering::Relaxed);
    }

    fn get_shm_render_count(&self) -> u32 {
        self.shm_render_count.load(Ordering::Relaxed)
    }
}
