use crate::SmearorCompositor;
use std::sync::atomic::Ordering;
use tracing::debug;

pub trait FrameLimiter {
    /// Set maximum frames per second
    fn set_max_fps(&self, max_fps: i64);

    /// Get maximum frames per second
    fn get_max_fps(&self) -> Option<i64>;

    /// Get frame rate limit in milliseconds
    fn frame_rate_limit(&self) -> Option<i64>;
}

impl FrameLimiter for SmearorCompositor {
    fn set_max_fps(&self, max_fps: i64) {
        let frame_rate_limit_ms = if max_fps <= 0 { -1 } else { 1000 / max_fps };
        self.frame_rate_limit_ms.store(frame_rate_limit_ms, Ordering::Relaxed);
        debug!("Max FPS set to {max_fps} (frame_rate_limit_ms: {frame_rate_limit_ms})");
    }

    fn get_max_fps(&self) -> Option<i64> {
        let frame_rate_limit_ms = self.frame_rate_limit_ms.load(Ordering::Relaxed);
        if frame_rate_limit_ms <= 0 { None } else { Some(1000 / frame_rate_limit_ms) }
    }

    fn frame_rate_limit(&self) -> Option<i64> {
        let frame_rate_limit_ms = self.frame_rate_limit_ms.load(Ordering::Relaxed);
        if frame_rate_limit_ms <= 0 { None } else { Some(frame_rate_limit_ms) }
    }
}
