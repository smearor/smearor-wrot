use crate::SmearorCompositor;
use crate::surface::SurfaceQuery;
use smithay::desktop::PopupKind;
use smithay::desktop::PopupManager;
use smithay::desktop::find_popup_root_surface;
use smithay::desktop::get_popup_toplevel_coords;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Logical;
use smithay::utils::Point;
use smithay::wayland::shell::xdg::PopupSurface;
use tracing::debug;
use tracing::error;

pub trait PopupHandler {
    /// Returns all currently tracked popups for rendering
    /// This is needed to render popup surfaces like menus and tooltips
    fn get_all_popups(&self) -> Vec<(PopupKind, Point<i32, Logical>)>;

    /// Handle popup commits
    /// Should be called on `WlSurface::commit`
    fn handle_popup_commits(&mut self, surface: &WlSurface);

    /// Unconstrain a popup surface
    fn unconstrain_popup(&self, popup: &PopupSurface);
}

impl PopupHandler for SmearorCompositor {
    fn get_all_popups(&self) -> Vec<(PopupKind, Point<i32, Logical>)> {
        // Use PopupManager::popups_for_surface() static method to get popups for each window
        let mut all_popups = Vec::new();
        for window in self.space.elements() {
            if let Some(toplevel) = window.toplevel() {
                let surface = toplevel.wl_surface();
                let popup_iter = PopupManager::popups_for_surface(&surface);
                for (popup, position) in popup_iter {
                    all_popups.push((popup, position));
                }
            }
        }
        debug!("get_all_popups found {} popups", all_popups.len());
        all_popups
    }

    fn handle_popup_commits(&mut self, surface: &WlSurface) {
        self.popups.commit(surface);
        if let Some(popup) = self.popups.find_popup(surface) {
            match popup {
                PopupKind::Xdg(ref xdg) => {
                    if !xdg.is_initial_configure_sent() {
                        if let Err(e) = xdg.send_configure() {
                            error!("Failed to send initial configure for popup: {}", e);
                        }
                    }
                }
                PopupKind::InputMethod(ref _input_method) => {}
            }
        }
    }

    fn unconstrain_popup(&self, popup: &PopupSurface) {
        let Ok(root) = find_popup_root_surface(&PopupKind::Xdg(popup.clone())) else {
            return;
        };
        let Some(window) = self.window_for_surface(&root) else {
            return;
        };

        let Some(output) = self.space.outputs().next() else {
            error!("No output available for popup constraint");
            return;
        };

        let Some(output_geo) = self.space.output_geometry(output) else {
            error!("Failed to get output geometry for popup constraint");
            return;
        };

        let Some(window_geo) = self.space.element_geometry(&window) else {
            error!("Failed to get window geometry for popup constraint");
            return;
        };

        let mut target = output_geo;
        target.loc -= get_popup_toplevel_coords(&PopupKind::Xdg(popup.clone()));
        target.loc -= window_geo.loc;

        popup.with_pending_state(|state| {
            state.geometry = state.positioner.get_unconstrained_geometry(target);
        });
    }
}
