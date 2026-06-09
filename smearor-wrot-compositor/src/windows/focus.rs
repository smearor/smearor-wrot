//! Window focus management

use smithay::desktop::Window;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Serial;

use crate::compositor::SmearorCompositor;
use crate::surface::query::SurfaceQuery;
use crate::windows::activation::WindowActivation;

use tracing::error;

/// Trait for window focus management
pub trait WindowFocus {
    /// Set keyboard focus to a window
    fn set_keyboard_focus(&mut self, window: Option<&Window>, serial: Serial);

    /// Get the currently focused window
    fn focused_window(&self) -> Option<Window>;

    /// Set focus to the active Wayland window when the compositor window gets focus
    fn set_focus_to_active_window(&mut self);

    /// Clear focus from all Wayland windows when the compositor window loses focus
    fn clear_focus(&mut self);
}

impl WindowFocus for SmearorCompositor {
    fn set_keyboard_focus(&mut self, window: Option<&Window>, serial: Serial) {
        let seat = &self.states.seat;
        let Some(keyboard) = seat.get_keyboard() else {
            error!("Keyboard not available for focus setting");
            return;
        };

        if let Some(window) = window {
            self.activate_window(window);
            if let Some(toplevel) = window.toplevel() {
                keyboard.set_focus(self, Some(toplevel.wl_surface().clone()), serial);
            }
        } else {
            self.deactivate_all_windows();
            keyboard.set_focus(self, Option::<WlSurface>::None, serial);
        }
    }

    fn focused_window(&self) -> Option<Window> {
        let seat = &self.states.seat;
        let Some(keyboard) = seat.get_keyboard() else {
            error!("Keyboard not available for focus query");
            return None;
        };

        keyboard.current_focus().and_then(|surface| self.window_for_surface(&surface))
    }

    fn set_focus_to_active_window(&mut self) {
        let serial = smithay::utils::SERIAL_COUNTER.next_serial();
        if let Some(keyboard) = self.states.seat.get_keyboard() {
            let surface_to_focus = self
                .space
                .elements()
                .find_map(|window| window.toplevel().map(|toplevel| toplevel.wl_surface().clone()));
            if let Some(surface) = surface_to_focus {
                keyboard.set_focus(self, Some(surface), serial);
            }
        }
    }

    fn clear_focus(&mut self) {
        let serial = smithay::utils::SERIAL_COUNTER.next_serial();
        if let Some(keyboard) = self.states.seat.get_keyboard() {
            keyboard.set_focus(self, Option::<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface>::None, serial);
        }
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;
    use smithay::utils::Serial;

    #[test]
    fn test_window_focus_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::focus::WindowFocus;

        let _ = |c: &mut SmearorCompositor, w: Option<&Window>, s: Serial| {
            c.set_keyboard_focus(w, s);
        };
    }

    #[test]
    fn test_window_focused_window_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::focus::WindowFocus;

        let _ = |c: &SmearorCompositor| {
            c.focused_window();
        };
    }
}
