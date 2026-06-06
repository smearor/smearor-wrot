use crate::BufferLifecycle;
use crate::SmearorCompositor;
use crate::surface::cleanup::DeadSurfaceCleanup;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::wayland::buffer::BufferHandler;

impl BufferHandler for SmearorCompositor {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {
        BufferLifecycle::buffer_destroyed(self, buffer);
        // Remove surfaces that have no buffers attached
        self.cleanup_dead_surfaces();
    }
}
