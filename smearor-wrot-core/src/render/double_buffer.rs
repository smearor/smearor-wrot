//! Double buffering for compositor rendering
//!
//! This module provides traits and implementations for double buffering
//! to prevent flickering and tearing during rendering.

use crate::compositor::SmearorCompositor;

/// Trait for double buffering during rendering
///
/// Double buffering uses two buffers (front and back) to prevent flickering
/// and tearing. Rendering happens on the back buffer while the front buffer
/// is displayed, then they are swapped.
pub trait DoubleBuffer {
    /// Check if double buffering is enabled
    fn is_double_buffer_enabled(&self) -> bool;

    /// Enable or disable double buffering
    fn set_double_buffer_enabled(&self, enabled: bool);

    /// Swap front and back buffers
    fn swap_buffers(&self);
}

impl DoubleBuffer for SmearorCompositor {
    fn is_double_buffer_enabled(&self) -> bool {
        self.double_buffer_enabled.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn set_double_buffer_enabled(&self, enabled: bool) {
        self.double_buffer_enabled.store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    fn swap_buffers(&self) {
        if let Ok(ref mut front_buffer) = self.front_buffer.lock() {
            if let Ok(ref mut back_buffer) = self.back_buffer.lock() {
                std::mem::swap(front_buffer, back_buffer);
            }
        }
    }
}
