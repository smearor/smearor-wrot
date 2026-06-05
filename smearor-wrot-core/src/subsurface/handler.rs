use crate::SmearorCompositor;
use crate::subsurface::model::SubsurfacePositionData;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::wayland::compositor::SubsurfaceCachedState;
use smithay::wayland::compositor::with_states;
use tracing::debug;
use tracing::error;

pub trait SubsurfaceHandler {
    /// Returns all subsurfaces for rendering
    /// This is needed to render subsurface-based popups (e.g., GTK4 native popups)
    fn get_all_subsurfaces(&self) -> Vec<SubsurfacePositionData>;

    /// Cleanup all subsurfaces for a toplevel surface
    fn cleanup_surfaces_for_toplevel(&mut self, surface: &WlSurface);

    /// Returns the count of active subsurfaces
    fn active_subsurface_count(&self) -> usize;
}

impl SubsurfaceHandler for SmearorCompositor {
    /// Returns all subsurfaces for rendering
    /// This is needed to render subsurface-based popups (e.g., GTK4 native popups)
    fn get_all_subsurfaces(&self) -> Vec<SubsurfacePositionData> {
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
                let subsurface_location_data = SubsurfacePositionData::new(&subsurface.parent, &subsurface.subsurface, &position);
                all_subsurfaces.push(subsurface_location_data);
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
