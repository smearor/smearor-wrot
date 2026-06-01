use crate::SmearorCompositor;
use std::time::Duration;
use tracing::debug;

pub trait FrameLimiter {
    /// Set maximum frames per second
    fn set_max_fps(&self, max_fps: u32);

    /// Get maximum frames per second
    fn get_max_fps(&self) -> Option<u32>;
}

impl FrameLimiter for SmearorCompositor {
    fn set_max_fps(&self, max_fps: u32) {
        let frame_duration = if max_fps == 0 {
            None
        } else {
            Some(Duration::from_millis((1000 / max_fps) as u64))
        };
        *self.frame_rate_limit.lock().unwrap() = frame_duration;
        debug!("Max FPS set to {} (frame duration: {:?})", max_fps, frame_duration);
    }

    fn get_max_fps(&self) -> Option<u32> {
        self.frame_rate_limit
            .lock()
            .ok()
            .and_then(|limit| limit.map(|duration| 1000 / duration.as_millis() as u32))
    }
}
