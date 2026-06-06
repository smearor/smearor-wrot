//! Buffer import and export

use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

use crate::buffer::lifecycle::BufferLifecycle;
use crate::compositor::SmearorCompositor;

/// Trait for buffer import and export operations
pub trait BufferImportExport {
    /// Import a buffer for use with a surface
    fn import_buffer(&mut self, surface: &WlSurface, buffer: &WlBuffer);

    /// Export a buffer from a surface
    fn export_buffer(&self, surface: &WlSurface) -> Option<WlBuffer>;

    /// Check if a buffer can be imported
    fn can_import_buffer(&self, buffer: &WlBuffer) -> bool;

    /// Check if a surface has a buffer attached
    fn has_buffer(&self, surface: &WlSurface) -> bool;
}

impl BufferImportExport for SmearorCompositor {
    fn import_buffer(&mut self, surface: &WlSurface, buffer: &WlBuffer) {
        let surface_id = surface.id();
        let buffer_id = buffer.id();
        self.surface_buffers.insert(surface_id, buffer_id);
    }

    fn export_buffer(&self, _surface: &WlSurface) -> Option<WlBuffer> {
        // Buffer export not directly supported in Smithay
        None
    }

    fn can_import_buffer(&self, buffer: &WlBuffer) -> bool {
        !self.is_buffer_in_use(buffer)
    }

    fn has_buffer(&self, surface: &WlSurface) -> bool {
        let surface_id = surface.id();
        self.surface_buffers.contains_key(&surface_id)
    }
}

#[cfg(test)]
mod tests {
    use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
    use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

    #[test]
    fn test_buffer_import_export_trait_exists() {
        use crate::buffer::import_export::BufferImportExport;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, s: &WlSurface, b: &WlBuffer| {
            c.import_buffer(s, b);
        };
    }

    #[test]
    fn test_export_buffer_trait_exists() {
        use crate::buffer::import_export::BufferImportExport;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &SmearorCompositor, s: &WlSurface| {
            c.export_buffer(s);
        };
    }

    #[test]
    fn test_can_import_buffer_trait_exists() {
        use crate::buffer::import_export::BufferImportExport;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &SmearorCompositor, b: &WlBuffer| {
            c.can_import_buffer(b);
        };
    }

    #[test]
    fn test_has_buffer_trait_exists() {
        use crate::buffer::import_export::BufferImportExport;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &SmearorCompositor, s: &WlSurface| {
            c.has_buffer(s);
        };
    }
}
