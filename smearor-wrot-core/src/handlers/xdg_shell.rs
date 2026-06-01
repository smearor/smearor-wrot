//! Wayland XDG Shell protocol handler

use crate::compositor::SmearorCompositor;
use crate::margin::handler::MarginHandler;
use crate::message::compositor_message::CompositorMessage;
use crate::message::sender::CompositorMessageSender;
use crate::popup::handler::PopupHandler;
use crate::surface::dialog::DialogSizeQuery;
use smithay::desktop::PopupKind;
use smithay::desktop::Window;
use smithay::desktop::find_popup_root_surface;
use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_seat;
use smithay::utils::Serial;
use smithay::wayland::compositor::with_states;
use smithay::wayland::shell::xdg::PopupSurface;
use smithay::wayland::shell::xdg::PositionerState;
use smithay::wayland::shell::xdg::ToplevelSurface;
use smithay::wayland::shell::xdg::XdgShellHandler;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::wayland::shell::xdg::XdgToplevelSurfaceData;
use tracing::debug;
use tracing::error;
use tracing::info;

impl XdgShellHandler for SmearorCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        debug!("new_toplevel called for surface, compositor ptr: {:p}, space ptr: {:p}", self, &self.space);
        debug!("Before mapping, space has {} elements", self.space.elements().count());
        let wl_surface = surface.wl_surface();
        debug!("new_toplevel: surface id={:?}", wl_surface.id());

        // TODO: Phase 6 - Dialog Management - Manual dialog detection
        // Detect dialogs by checking if this is a second toplevel
        let existing_toplevels_count = self.space.elements().count();
        let is_dialog = existing_toplevels_count > 0;
        debug!("new_toplevel: existing_toplevels_count={}, is_dialog (count-based)={}", existing_toplevels_count, is_dialog);

        // Also check if the XdgDialogHandler has already registered this as a dialog
        let is_dialog_via_handler = if let Ok(dialogs) = self.dialogs.lock() {
            dialogs.iter().any(|d| d.wl_surface() == wl_surface)
        } else {
            false
        };
        debug!("new_toplevel: is_dialog (via handler)={}", is_dialog_via_handler);

        // Use either detection method
        let is_dialog = is_dialog || is_dialog_via_handler;
        debug!("new_toplevel: final is_dialog={}", is_dialog);

        let window = Window::new_wayland_window(surface.clone());

        // Mark that the compositor has had a surface
        if let Ok(mut flag) = self.has_had_surface.lock() {
            *flag = true;
        }

        // This is required for GTK4 applications like GNOME Calculator
        // They need an explicit size to render properly
        // Get the initial size from the virtual output
        let output_size = if let Some(output) = &self.virtual_output {
            output.current_mode().map(|mode| (mode.size.w, mode.size.h)).unwrap_or((1920, 1080))
        } else {
            (1920, 1080)
        };

        // Apply margin-based size reduction
        let margin_left = self.get_margin_left() as i32;
        let margin_right = self.get_margin_right() as i32;
        let margin_top = self.get_margin_top() as i32;
        let margin_bottom = self.get_margin_bottom() as i32;

        let adjusted_width = output_size.0 - margin_left - margin_right;
        let adjusted_height = output_size.1 - margin_top - margin_bottom;
        let adjusted_size = (adjusted_width.max(100), adjusted_height.max(100));

        debug!(
            "Setting initial surface size to {}x{} (margins: l={}, r={}, t={}, b={})",
            adjusted_size.0, adjusted_size.1, margin_left, margin_right, margin_top, margin_bottom
        );

        surface.with_pending_state(|surface_state| {
            surface_state.size = Some(adjusted_size.into());
        });

        // TODO: Phase 5 - Send initial configure event immediately
        // This is required for GTK4 applications like GNOME Calculator
        // They wait for configure event before committing their first buffer
        if let Some(toplevel) = window.toplevel() {
            toplevel.send_configure();
            debug!("Initial configure event sent for toplevel with size {}x{}", adjusted_size.0, adjusted_size.1);
        } else {
            error!("Failed to get toplevel from window");
        }

        // Map window at margin position for left/top margins only
        // Right/bottom margins only affect size, not position
        let margin_left = self.get_margin_left() as i32;
        let margin_top = self.get_margin_top() as i32;

        // Activate dialogs when mapping them
        self.space.map_element(window.clone(), (margin_left, margin_top), is_dialog);

        // Notify GTK wrapper that a window was mapped (for rotation widget to update size)
        self.send_message(CompositorMessage::WindowMapped);

        if is_dialog {
            debug!("new_toplevel: This appears to be a dialog ({} toplevels already exist)", existing_toplevels_count);

            // Add to dialog registry manually since XdgDialogHandler is not called
            if let Ok(mut dialogs) = self.dialogs.lock() {
                if !dialogs.iter().any(|d| d.wl_surface() == wl_surface) {
                    dialogs.push(surface.clone());
                    debug!("new_toplevel: Added dialog to registry manually, total dialogs: {}", dialogs.len());
                }
            } else {
                error!("Failed to lock dialogs registry for manual dialog detection");
            }

            // Apply margin-based size reduction
            let margin_left = self.get_margin_left() as i32;
            let margin_right = self.get_margin_right() as i32;
            let margin_top = self.get_margin_top() as i32;
            let margin_bottom = self.get_margin_bottom() as i32;

            let dialog_margin = self.get_dialog_margin() as i32;

            let adjusted_width = output_size.0 - margin_left - margin_right - 2 * dialog_margin;
            let adjusted_height = output_size.1 - margin_top - margin_bottom - 2 * dialog_margin;

            // Ensure adjusted size is positive
            let adjusted_width = adjusted_width.max(100);
            let adjusted_height = adjusted_height.max(100);

            debug!("new_toplevel: Configuring dialog with adjusted size {}x{}", adjusted_width, adjusted_height);

            surface.with_pending_state(|surface_state| {
                surface_state.size = Some((adjusted_width, adjusted_height).into());
            });

            // Send configure event to the dialog
            if let Some(toplevel) = window.toplevel() {
                toplevel.send_configure();
                // Store the dialog size so it can be used for input bounds calculation
                self.set_dialog_size(surface.wl_surface(), adjusted_width, adjusted_height);
                debug!("new_toplevel: Sent configure event to dialog with size {}x{}", adjusted_width, adjusted_height);
            }
        }

        // Set window geometry manually to allow element_under to find the window
        // This is a workaround for the fact that Smithay only updates geometry when a buffer is attached
        // The actual geometry will be updated when the client commits a buffer
        // Note: We can't directly set window geometry in Smithay
        // The geometry is calculated from the buffer size
        // This is a known limitation - we need to ensure the client commits a buffer
        debug!("Window will have geometry {}x{} when buffer is attached", adjusted_size.0, adjusted_size.1);

        debug!("After mapping, space has {} elements", self.space.elements().count());

        // Don't flush clients here - it might block the GTK main loop
        // The Wayland event loop will handle flushing
    }

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {
        debug!("new_popup called for surface: {:?}", surface.wl_surface().id());
        self.unconstrain_popup(&surface);
        let popup_kind = PopupKind::Xdg(surface.clone());
        let _ = self.popups.track_popup(popup_kind.clone());

        // Activate popup grab for interactive popups
        // Smithay's PopupManager manages the grab automatically
        // when the SeatHandler is implemented
        if let Some(_popup) = self.popups.find_popup(surface.wl_surface()) {
            // Activate grab - Smithay automatically forwards events to the popup
            // This requires that the SeatHandler is implemented
            debug!("Popup grab activated for interactive popup");
        }

        debug!("new_popup completed for surface: {:?}", surface.wl_surface().id());
    }

    fn move_request(&mut self, _surface: ToplevelSurface, _seat: wl_seat::WlSeat, serial: Serial) {
        debug!("Client requested move for toplevel");
        // Send MoveRequest message to GTK wrapper to move the outer window
        self.send_message(CompositorMessage::MoveRequest(serial.into()));
    }

    fn resize_request(&mut self, _surface: ToplevelSurface, _seat: wl_seat::WlSeat, serial: Serial, _edges: xdg_toplevel::ResizeEdge) {
        debug!("Client requested resize for toplevel");
        // Send ResizeRequest message to GTK wrapper to resize the outer window
        // TODO: pass through edges
        self.send_message(CompositorMessage::ResizeRequest(serial.into()));
    }

    fn grab(&mut self, surface: PopupSurface, _seat: wl_seat::WlSeat, serial: Serial) {
        // Activate popup grab for interactive popups
        // This ensures that the popup receives all input events
        debug!("Popup grab requested for surface: {:?}", surface.wl_surface().id());

        // Smithay's PopupManager manages the grab automatically
        // We must activate the grab via grab_popup
        let popup_kind = PopupKind::Xdg(surface.clone());

        // Find the root surface for the popup grab
        let root_surface = match find_popup_root_surface(&popup_kind) {
            Ok(surface) => surface,
            Err(_) => {
                error!("Failed to find popup root surface");
                return;
            }
        };

        // Activate the popup grab via Smithay's PopupManager
        match self.popups.grab_popup::<SmearorCompositor>(root_surface, popup_kind, &self.seat, serial) {
            Ok(_popup_grab) => {
                debug!("Popup grab activated successfully");
            }
            Err(e) => {
                error!("Failed to activate popup grab: {}", e);
            }
        }
    }

    fn maximize_request(&mut self, surface: ToplevelSurface) {
        debug!("Client requested maximize for toplevel");
        // When a client requests maximize, we send a message to the GTK wrapper to maximize the window
        self.send_message(CompositorMessage::Maximize);
        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Maximized);
        });
        surface.send_configure();
    }

    fn unmaximize_request(&mut self, surface: ToplevelSurface) {
        debug!("Client requested unmaximize for toplevel");
        // TODO: Phase 3 - Synchronize maximize button between client window and compositor window
        // When a client requests unmaximize, we send a message to the GTK wrapper to unmaximize the window
        self.send_message(CompositorMessage::Unmaximize);
        surface.with_pending_state(|state| {
            state.states.unset(xdg_toplevel::State::Maximized);
        });
        surface.send_configure();
    }

    fn fullscreen_request(&mut self, surface: ToplevelSurface, _output: Option<smithay::reexports::wayland_server::protocol::wl_output::WlOutput>) {
        debug!("Client requested fullscreen for toplevel");
        // When a client requests fullscreen, we send a message to the GTK wrapper to fullscreen the window
        self.send_message(CompositorMessage::Fullscreen);
        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Fullscreen);
        });
        surface.send_configure();
    }

    fn unfullscreen_request(&mut self, surface: ToplevelSurface) {
        debug!("Client requested unfullscreen for toplevel");
        // When a client requests unfullscreen, we send a message to the GTK wrapper to unfullscreen the window
        self.send_message(CompositorMessage::Unfullscreen);
        surface.with_pending_state(|state| {
            state.states.unset(xdg_toplevel::State::Fullscreen);
        });
        surface.send_configure();
    }

    fn minimize_request(&mut self, _surface: ToplevelSurface) {
        debug!("Client requested minimize for toplevel");
        // When a client requests minimize, we send a message to the GTK wrapper to minimize the window
        // Note: Wayland XDG Shell does not have a Minimized state, so we only send the message to GTK
        self.send_message(CompositorMessage::Minimize);
    }

    fn reposition_request(&mut self, surface: PopupSurface, positioner: PositionerState, token: u32) {
        surface.with_pending_state(|state| {
            let geometry = positioner.get_geometry();
            state.geometry = geometry;
            state.positioner = positioner;
        });
        self.unconstrain_popup(&surface);
        surface.send_repositioned(token);
    }

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        info!("Toplevel destroyed: {:?}", surface.wl_surface().id());
        // TODO: Phase 6 - Dialog Management - Clean up dialog from registry and space
        // Only remove from registry if the surface is actually in the registry
        // This prevents removing the main window if it was incorrectly added
        if let Ok(mut dialogs) = self.dialogs.lock() {
            let surface_id = surface.wl_surface();
            let was_in_registry = dialogs.iter().any(|d| d.wl_surface() == surface_id);
            if was_in_registry {
                dialogs.retain(|d| d.wl_surface() != surface_id);
                debug!("Removed dialog from registry after toplevel_destroyed");

                // Also remove the dialog from the Smithay space
                // Find the window corresponding to this surface
                let window_to_unmap = self
                    .space
                    .elements()
                    .find(|w| w.toplevel().map(|t| t.wl_surface() == surface_id).unwrap_or(false))
                    .cloned();

                if let Some(window) = window_to_unmap {
                    self.space.unmap_elem(&window);
                    debug!("Unmapped dialog from Smithay space");
                }
            } else {
                debug!("Toplevel was not in dialog registry, skipping cleanup");
            }
        }
    }

    fn app_id_changed(&mut self, surface: ToplevelSurface) {
        debug!("app_id changed toplevel surface {}", surface.wl_surface().id());
        let app_id = with_states(surface.wl_surface(), |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .and_then(|data| data.lock().ok())
                .and_then(|attributes| attributes.app_id.clone())
        });
        if let Some(app_id) = app_id {
            debug!("app_id of toplevel surface {} changed to {app_id}", surface.wl_surface().id());
            self.send_message(CompositorMessage::AppIdChanged(app_id));
        } else {
            debug!("app_id removed from toplevel surface {}", surface.wl_surface().id());
        }
    }
}

smithay::delegate_xdg_shell!(SmearorCompositor);
