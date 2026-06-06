use crate::SmearorCompositor;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tracing::debug;

pub trait FrameLimiter {
    /// Set maximum frames per second
    fn set_max_fps(&self, max_fps: i64);

    /// Get maximum frames per second
    fn get_max_fps(&self) -> Option<i64>;

    /// Get frame rate limit in milliseconds
    fn frame_rate_limit(&self) -> Option<i64>;

    /// Get elapsed time since start
    fn elapsed_since_start(&self) -> Duration;

    /// Check if a frame should be sent
    fn should_send_frame(&self) -> bool;
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

    fn elapsed_since_start(&self) -> Duration {
        self.start_time.elapsed()
    }

    fn should_send_frame(&self) -> bool {
        let frame_rate_limit_ms = self.frame_rate_limit_ms.load(Ordering::Relaxed);
        if frame_rate_limit_ms <= 0 {
            return true;
        }
        let last_frame_time = self.last_frame_time.load(Ordering::Relaxed);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
        (now - last_frame_time) >= frame_rate_limit_ms
    }
}
