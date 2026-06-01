//! Frame rate limiting for compositor rendering
//!
//! This module provides traits and implementations for limiting the frame rate
//! of compositor rendering to improve performance and reduce resource usage.

use crate::compositor::SmearorCompositor;
use std::time::Duration;
use std::time::Instant;

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
        let Ok(frame_rate_limit) = self.frame_rate_limit.lock() else {
            return true;
        };
        let Some(limit) = *frame_rate_limit else {
            return true;
        };
        let Ok(last_frame_time) = self.last_frame_time.lock() else {
            return true;
        };
        last_frame_time.elapsed() > limit
    }

    fn update_frame_time(&self) {
        if let Ok(mut last_frame_time) = self.last_frame_time.lock() {
            *last_frame_time = Instant::now();
        }
    }

    fn set_frame_rate_limit(&self, limit: Option<Duration>) {
        if let Ok(mut frame_rate_limit) = self.frame_rate_limit.lock() {
            *frame_rate_limit = limit;
        }
    }
}
