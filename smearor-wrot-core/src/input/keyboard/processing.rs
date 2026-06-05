use crate::SmearorCompositor;
use crate::input::keyboard::convert::GtkToSmithayKeyEventConverter;
use smithay::desktop::Window;
use smithay::input::keyboard::FilterResult;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::SERIAL_COUNTER;
use smithay::wayland::shell::xdg::ToplevelSurface;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::error;
use tracing::trace;

/// Trait for input processing methods
pub trait KeyboardInputProcessing {
    /// Process a GTK key press event
    fn process_gtk_key_press(&mut self, keycode: u32);

    /// Process a GTK key release event
    fn process_gtk_key_release(&mut self, keycode: u32);

    /// Set keyboard focus to a specific window
    fn focus_window(&mut self, window: &Window);

    /// Set keyboard focus to a specific toplevel
    fn focus_toplevel(&mut self, toplevel: &ToplevelSurface);

    /// Set keyboard focus to a specific surface
    fn focus_surface(&mut self, surface: &WlSurface);

    /// Clear keyboard focus
    fn clear_focus(&mut self);
}

impl KeyboardInputProcessing for SmearorCompositor {
    fn process_gtk_key_press(&mut self, keycode: u32) {
        let key_event = Self::convert_gtk_key_press(keycode);

        let Some(keyboard) = self.seat.get_keyboard() else {
            error!("Keyboard not available for GTK key press");
            return;
        };

        keyboard.input::<Self, _>(self, key_event.key, key_event.state, key_event.serial, key_event.time, |_, _, _| FilterResult::Forward);
    }

    fn process_gtk_key_release(&mut self, keycode: u32) {
        let key_event = Self::convert_gtk_key_release(keycode);

        let Some(keyboard) = self.seat.get_keyboard() else {
            error!("Keyboard not available for GTK key release");
            return;
        };

        keyboard.input::<Self, _>(self, key_event.key, key_event.state, key_event.serial, key_event.time, |_, _, _| FilterResult::Forward);
    }

    fn focus_window(&mut self, window: &Window) {
        self.space.raise_element(window, true);
        if let Some(toplevel) = window.toplevel() {
            trace!("Setting focus to toplevel surface");
            self.focus_toplevel(toplevel);
        }
        self.space.elements().for_each(|window| {
            if let Some(toplevel) = window.toplevel() {
                toplevel.send_pending_configure();
            }
        });
    }

    fn focus_toplevel(&mut self, toplevel: &ToplevelSurface) {
        self.focus_surface(toplevel.wl_surface());
    }

    fn focus_surface(&mut self, surface: &WlSurface) {
        let Some(keyboard) = self.seat.get_keyboard() else {
            error!("Keyboard not available for GTK key release");
            return;
        };
        keyboard.set_focus(self, Some(surface.clone()), SERIAL_COUNTER.next_serial());
    }

    fn clear_focus(&mut self) {
        let Some(keyboard) = self.seat.get_keyboard() else {
            error!("Keyboard not available in seat");
            return;
        };
        self.space.elements().for_each(|window| {
            window.set_activated(false);
            if let Some(toplevel) = window.toplevel() {
                toplevel.send_pending_configure();
            }
        });
        keyboard.set_focus(self, Option::<WlSurface>::None, SERIAL_COUNTER.next_serial());
    }
}

impl KeyboardInputProcessing for Arc<Mutex<SmearorCompositor>> {
    fn process_gtk_key_press(&mut self, keycode: u32) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_key_press(&mut *guard, keycode);
        }
    }

    fn process_gtk_key_release(&mut self, keycode: u32) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_key_release(&mut *guard, keycode);
        }
    }

    fn focus_window(&mut self, window: &Window) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::focus_window(&mut *guard, window);
        }
    }

    fn focus_toplevel(&mut self, toplevel: &ToplevelSurface) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::focus_toplevel(&mut *guard, toplevel);
        }
    }

    fn focus_surface(&mut self, surface: &WlSurface) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::focus_surface(&mut *guard, surface);
        }
    }

    fn clear_focus(&mut self) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::clear_focus(&mut *guard);
        }
    }
}
