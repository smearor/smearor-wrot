//! Event data types for GTK to Smithay conversion

use smithay::backend::input::KeyState;
use smithay::utils::Serial;
use xkeysym::KeyCode;

/// Key event data for GTK to Smithay conversion
#[derive(Debug, Clone, PartialEq)]
pub struct KeyEventData {
    pub key: KeyCode,
    pub state: KeyState,
    pub serial: Serial,
    pub time: u32,
}
