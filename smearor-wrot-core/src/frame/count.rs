use crate::SmearorCompositor;
use std::sync::atomic::Ordering;

pub trait FrameCounter {
    /// Get the current frame count
    fn get_frame_count(&self) -> Option<u32>;

    /// Increment the frame count
    fn increment_frame_count(&self);

    /// Reset the frame count
    fn reset_frame_count(&self);
}

impl FrameCounter for SmearorCompositor {
    fn get_frame_count(&self) -> Option<u32> {
        Some(self.frame_count.load(Ordering::Relaxed))
    }

    fn increment_frame_count(&self) {
        self.frame_count.fetch_add(1, Ordering::Relaxed);
    }

    fn reset_frame_count(&self) {
        self.frame_count.store(0, Ordering::Relaxed);
    }
}
