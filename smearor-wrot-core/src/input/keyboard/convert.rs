use crate::SmearorCompositor;
use crate::input::keyboard::event::KeyEventData;
use crate::input::time::get_time;
use smithay::backend::input::KeyState;
use smithay::utils::SERIAL_COUNTER;
use xkeysym::KeyCode;

pub trait GtkToSmithayKeyEventConverter {
    /// Convert GTK key press event data to Smithay key event data
    fn convert_gtk_key_press(keycode: u32) -> KeyEventData;

    /// Convert GTK key release event data to Smithay key event data
    fn convert_gtk_key_release(keycode: u32) -> KeyEventData;
}

impl GtkToSmithayKeyEventConverter for SmearorCompositor {
    fn convert_gtk_key_press(keycode: u32) -> KeyEventData {
        KeyEventData {
            key: KeyCode::from(keycode),
            state: KeyState::Pressed,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        }
    }

    fn convert_gtk_key_release(keycode: u32) -> KeyEventData {
        KeyEventData {
            key: KeyCode::from(keycode),
            state: KeyState::Released,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        }
    }
}
