use crate::SmearorCompositor;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Logical;
use smithay::utils::Point;
use smithay::wayland::compositor::SubsurfaceCachedState;
use smithay::wayland::compositor::with_states;
use tracing::debug;
use tracing::error;

pub trait SubsurfaceHandler {
    /// Returns all subsurfaces for rendering
    /// This is needed to render subsurface-based popups (e.g., GTK4 native popups)
    fn get_all_subsurfaces(&self) -> Vec<(WlSurface, Point<i32, Logical>)>;

    /// Cleanup all subsurfaces for a toplevel surface
    fn cleanup_surfaces_for_toplevel(&mut self, surface: &WlSurface);

    /// Returns the count of active subsurfaces
    fn active_subsurface_count(&self) -> usize;
}

impl SubsurfaceHandler for SmearorCompositor {
    /// TODO: Phase 5 - Subsurface Rendering - Get all subsurfaces
    /// Returns all subsurfaces for rendering
    /// This is needed to render subsurface-based popups (e.g., GTK4 native popups)
    fn get_all_subsurfaces(&self) -> Vec<(WlSurface, Point<i32, Logical>)> {
        // Get all tracked subsurfaces
        let Ok(subsurfaces) = self.subsurfaces.lock() else {
            error!("Failed to lock subsurfaces registry");
            return Vec::new();
        };

        let mut all_subsurfaces = Vec::new();

        for subsurface in subsurfaces.iter() {
            // Get the position from SubsurfaceCachedState
            with_states(&subsurface.subsurface, |states| {
                let mut cached_state = states.cached_state.get::<SubsurfaceCachedState>();
                let position = cached_state.current().location;
                all_subsurfaces.push((subsurface.subsurface.clone(), position));
            });
        }

        debug!("get_all_subsurfaces found {} subsurfaces", all_subsurfaces.len());
        all_subsurfaces
    }

    fn cleanup_surfaces_for_toplevel(&mut self, surface: &WlSurface) {
        if let Ok(mut subsurfaces) = self.subsurfaces.lock() {
            subsurfaces.retain(|subsurface_data| subsurface_data.parent.id() != surface.id());
        }
    }

    fn active_subsurface_count(&self) -> usize {
        let Ok(subsurfaces) = self.subsurfaces.lock() else {
            return 0;
        };
        subsurfaces.iter().filter(|subsurface_data| subsurface_data.subsurface.is_alive()).count()
    }
}
