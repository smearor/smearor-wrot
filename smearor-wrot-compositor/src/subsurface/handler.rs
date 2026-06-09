use crate::SmearorCompositor;
use crate::popup::handler::PopupHandler;
use crate::subsurface::model::SubsurfacePositionData;
use smearor_wrot_model_geometry::Position;
use smithay::desktop::PopupKind;
use smithay::desktop::find_popup_root_surface;
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

    fn find_surface_absolute_position(&self, surface: &WlSurface) -> Option<Position<i32>>;
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
                let position = cached_state.current().location.into();
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

    fn find_surface_absolute_position(&self, surface: &WlSurface) -> Option<Position<i32>> {
        for window in self.space.elements() {
            if let Some(toplevel) = window.toplevel() {
                if toplevel.wl_surface() == surface {
                    if let Some(loc) = self.space.element_location(window) {
                        let geom = window.geometry();
                        return Some(Position::new(loc.x - geom.loc.x, loc.y - geom.loc.y));
                    }
                }
            }
        }

        // 2. Is it a popup?
        for (popup, position) in self.get_all_popups() {
            let popup_surface = match &popup {
                PopupKind::Xdg(xdg) => xdg.wl_surface(),
                PopupKind::InputMethod(_) => continue,
            };
            if popup_surface == surface {
                // Find parent of this popup recursively
                if let Ok(popup_root) = find_popup_root_surface(&popup) {
                    if let Some(parent_pos) = self.find_surface_absolute_position(&popup_root) {
                        return Some(Position::new(parent_pos.x + position.x, parent_pos.y + position.y));
                    }
                }
            }
        }

        // 3. Is it a subsurface?
        for sub in self.get_all_subsurfaces() {
            if &sub.subsurface == surface {
                // Find parent of this subsurface recursively
                if let Some(parent_pos) = self.find_surface_absolute_position(&sub.parent) {
                    return Some(Position::new(parent_pos.x + sub.position.x, parent_pos.y + sub.position.y));
                }
            }
        }

        None
    }
}
