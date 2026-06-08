use crate::SmearorCompositor;
use crate::input::time::get_time;
use smearor_wrot_geometry::Position;
use smithay::backend::input::ButtonState;
use smithay::input::pointer::ButtonEvent;
use smithay::input::pointer::MotionEvent;
use smithay::utils::SERIAL_COUNTER;

/// GTK to Smithay mouse event conversion
pub trait GtkToSmithayMouseEventConverter {
    /// Convert GTK mouse motion event data to Smithay motion event data
    fn convert_gtk_mouse_motion(position: Position<f64>) -> MotionEvent;

    /// Convert GTK mouse press event data to Smithay button event data
    fn convert_gtk_mouse_press(gtk_button_code: u32) -> ButtonEvent;

    /// Convert GTK mouse release event data to Smithay button event data
    fn convert_gtk_mouse_release(gtk_button_code: u32) -> ButtonEvent;

    /// Convert GTK button code to Smithay button code
    fn convert_gtk_button_code_to_input_event_code(gtk_button_code: u32) -> u32;
}

impl GtkToSmithayMouseEventConverter for SmearorCompositor {
    fn convert_gtk_mouse_motion(position: Position<f64>) -> MotionEvent {
        MotionEvent {
            location: position.into(),
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        }
    }

    fn convert_gtk_mouse_press(gtk_button_code: u32) -> ButtonEvent {
        ButtonEvent {
            button: Self::convert_gtk_button_code_to_input_event_code(gtk_button_code),
            state: ButtonState::Pressed,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        }
    }

    fn convert_gtk_mouse_release(gtk_button_code: u32) -> ButtonEvent {
        ButtonEvent {
            button: Self::convert_gtk_button_code_to_input_event_code(gtk_button_code),
            state: ButtonState::Released,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        }
    }

    fn convert_gtk_button_code_to_input_event_code(button: u32) -> u32 {
        // Map GTK button codes to Smithay button codes
        // GTK: 1=left, 2=middle, 3=right
        // Smithay: 0x110=left, 0x112=middle, 0x113=right
        match button {
            1 => input_event_codes::BTN_LEFT!(),
            2 => input_event_codes::BTN_MIDDLE!(),
            3 => input_event_codes::BTN_RIGHT!(),
            4 => input_event_codes::BTN_SIDE!(),
            5 => input_event_codes::BTN_EXTRA!(),
            _ => input_event_codes::BTN_LEFT!(),
        }
    }
}
