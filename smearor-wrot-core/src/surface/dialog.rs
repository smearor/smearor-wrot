use crate::SmearorCompositor;
use smithay::backend::renderer::buffer_dimensions;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Buffer;
use smithay::utils::Size;
use smithay::wayland::compositor::BufferAssignment;
use smithay::wayland::compositor::SurfaceAttributes;
use smithay::wayland::compositor::with_states;
use smithay::wayland::seat::WaylandFocus;

pub trait DialogSizeQuery {
    // Get dialog size from surface attributes
    fn dialog_size_for_surface(&self, dialog_surface: &WlSurface) -> Option<Size<i32, Buffer>>;

    // Set dialog size from configure event
    fn set_dialog_size(&self, dialog_surface: &WlSurface, width: i32, height: i32);

    // Close a dialog
    fn close_dialog(&mut self, dialog_surface: &WlSurface);
}

impl DialogSizeQuery for SmearorCompositor {
    fn dialog_size_for_surface(&self, dialog_surface: &WlSurface) -> Option<Size<i32, Buffer>> {
        // First try to get size from the stored configure sizes
        if let Some((width, height)) = self.dialog_configure_sizes.get(dialog_surface).as_deref() {
            return Some(Size::from((*width, *height)));
        }

        // Fall back to buffer size
        with_states(dialog_surface, |surface_data| {
            let mut guard = surface_data.cached_state.get::<SurfaceAttributes>();
            let attrs = guard.current();
            // Try to get size from buffer first
            attrs.buffer.as_ref().and_then(|buffer| match buffer {
                BufferAssignment::NewBuffer(new_buffer) => Some(buffer_dimensions(new_buffer)),
                BufferAssignment::Removed => None,
            })
        })
        .flatten()
    }

    fn set_dialog_size(&self, dialog_surface: &WlSurface, width: i32, height: i32) {
        self.dialog_configure_sizes.insert(dialog_surface.clone(), (width, height));
    }

    fn close_dialog(&mut self, dialog_surface: &WlSurface) {
        if let Some(window_to_remove) = self
            .space
            .elements()
            .find(|window| window.wl_surface().as_deref() == Some(dialog_surface))
            .cloned() {
            // Send close event to the dialog's toplevel surface
            if let Some(toplevel) = window_to_remove.toplevel() {
                toplevel.send_close();
            }
        }
    }
}
