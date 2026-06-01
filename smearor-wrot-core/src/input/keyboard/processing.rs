use crate::SmearorCompositor;
use crate::input::keyboard::convert::GtkToSmithayKeyEventConverter;
use smithay::input::keyboard::FilterResult;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::error;

/// Trait for input processing methods
pub trait KeyboardInputProcessing {
    fn process_gtk_key_press(&mut self, keycode: u32);
    fn process_gtk_key_release(&mut self, keycode: u32);
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
}
