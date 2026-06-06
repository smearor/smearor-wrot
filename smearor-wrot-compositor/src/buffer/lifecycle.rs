//! Buffer lifecycle management

use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;

use crate::compositor::SmearorCompositor;

/// Trait for buffer lifecycle management
pub trait BufferLifecycle {
    /// Called when a buffer is destroyed
    fn buffer_destroyed(&mut self, buffer: &WlBuffer);

    /// Check if a buffer is still in use
    fn is_buffer_in_use(&self, buffer: &WlBuffer) -> bool;

    /// Mark a buffer as in use
    fn mark_buffer_in_use(&mut self, buffer: &WlBuffer);

    /// Release a buffer when no longer in use
    fn release_buffer(&mut self, buffer: &WlBuffer);
}

impl BufferLifecycle for SmearorCompositor {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {
        let id = buffer.id();
        self.buffers_in_use.remove(&id);
    }

    fn is_buffer_in_use(&self, buffer: &WlBuffer) -> bool {
        let id = buffer.id();
        self.buffers_in_use.contains(&id)
    }

    fn mark_buffer_in_use(&mut self, buffer: &WlBuffer) {
        let id = buffer.id();
        self.buffers_in_use.insert(id);
    }

    fn release_buffer(&mut self, buffer: &WlBuffer) {
        let id = buffer.id();
        self.buffers_in_use.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;

    #[test]
    fn test_buffer_lifecycle_trait_exists() {
        use crate::buffer::lifecycle::BufferLifecycle;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, b: &WlBuffer| {
            c.buffer_destroyed(b);
        };
    }

    #[test]
    fn test_buffer_is_buffer_in_use_trait_exists() {
        use crate::buffer::lifecycle::BufferLifecycle;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &SmearorCompositor, b: &WlBuffer| {
            c.is_buffer_in_use(b);
        };
    }

    #[test]
    fn test_buffer_mark_buffer_in_use_trait_exists() {
        use crate::buffer::lifecycle::BufferLifecycle;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, b: &WlBuffer| {
            c.mark_buffer_in_use(b);
        };
    }

    #[test]
    fn test_buffer_release_buffer_trait_exists() {
        use crate::buffer::lifecycle::BufferLifecycle;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, b: &WlBuffer| {
            c.release_buffer(b);
        };
    }
}
