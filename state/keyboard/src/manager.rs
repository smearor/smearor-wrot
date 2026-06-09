use crate::KeyboardState;
use crate::accessor::KeyboardStateAccessor;
use std::sync::Arc;

#[derive(Debug)]
pub struct KeyboardManager {
    pub(crate) state: Arc<KeyboardState>,
}

impl KeyboardManager {
    pub fn new(config: KeyboardState) -> Self {
        Self { state: Arc::new(config) }
    }
    pub fn keyboard_layout(&self) -> String {
        self.state.keyboard_layout().unwrap_or("us".to_string())
    }
    pub fn keyboard_variant(&self) -> Option<String> {
        self.state.keyboard_variant()
    }

    pub fn set_keyboard_layout(&mut self, keyboard_layout: String) {
        self.state.set_keyboard_layout(keyboard_layout);
    }

    pub fn set_keyboard_variant(&mut self, keyboard_variant: String) {
        self.state.set_keyboard_variant(keyboard_variant);
    }
}

impl KeyboardStateAccessor for KeyboardManager {
    fn keyboard_layout(&self) -> Option<String> {
        self.state.keyboard_layout()
    }

    fn keyboard_variant(&self) -> Option<String> {
        self.state.keyboard_variant()
    }

    fn set_keyboard_layout(&self, keyboard_layout: String) {
        self.state.set_keyboard_layout(keyboard_layout);
    }

    fn set_keyboard_variant(&self, keyboard_variant: String) {
        self.state.set_keyboard_variant(keyboard_variant);
    }
}
