//! Frame rate limiting for compositor rendering
//!
//! This module provides traits and implementations for limiting the frame rate
//! of compositor rendering to improve performance and reduce resource usage.

use crate::compositor::SmearorCompositor;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Trait for limiting frame rate during rendering
///
/// Frame rate limiting allows the compositor to control how often rendering
/// occurs, reducing CPU/GPU usage and improving system performance.
pub trait FrameLimiter {
    /// Check if rendering should proceed based on frame rate limiting
    ///
    /// # Returns
    ///
    /// true if rendering should proceed, false if it should be skipped
    fn should_render(&self) -> bool;

    /// Update the last frame time after rendering
    fn update_frame_time(&self);

    /// Set frame rate limit (None = unlimited)
    ///
    /// # Arguments
    ///
    /// * `limit` - The minimum duration between frames, or None for unlimited
    fn set_frame_rate_limit(&self, limit: Option<Duration>);
}

impl FrameLimiter for SmearorCompositor {
    fn should_render(&self) -> bool {
        let frame_rate_limit_ms = self.frame_rate_limit_ms.load(Ordering::Relaxed);
        if frame_rate_limit_ms < 0 {
            return true;
        }
        let last_frame_time = self.last_frame_time.load(Ordering::Relaxed);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
        (now - last_frame_time) >= frame_rate_limit_ms
    }

    fn update_frame_time(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
        self.last_frame_time.store(now, Ordering::Relaxed);
    }

    fn set_frame_rate_limit(&self, frame_rate_limit_ms: Option<Duration>) {
        self.frame_rate_limit_ms
            .store(frame_rate_limit_ms.map(|d| d.as_millis() as i64).unwrap_or(-1), Ordering::Relaxed);
    }
}
