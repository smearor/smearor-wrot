use crate::SmearorCompositor;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::wayland::compositor::with_states;
use smithay::wayland::shell::xdg::XdgToplevelSurfaceData;

pub trait TopLevelCommitHandler {
    /// Handle toplevel commits
    /// Should be called on `WlSurface::commit`
    fn handle_toplevel_commit(&self, surface: &WlSurface);
}

impl TopLevelCommitHandler for SmearorCompositor {
    fn handle_toplevel_commit(&self, surface: &WlSurface) {
        // Handle toplevel commits.
        let window = self
            .space
            .elements()
            .find(|w| w.toplevel().map(|t| t.wl_surface() == surface).unwrap_or(false))
            .cloned();
        if let Some(window) = window {
            let initial_configure_sent = with_states(surface, |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .and_then(|data| data.lock().ok())
                    .map(|data| data.initial_configure_sent)
                    .unwrap_or(false)
            });

            if !initial_configure_sent {
                if let Some(toplevel) = window.toplevel() {
                    toplevel.send_configure();
                }
            }

            // Handle geometry updates - no remapping needed, rendering handles offset
        }
    }
}
