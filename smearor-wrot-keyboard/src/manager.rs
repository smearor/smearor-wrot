use crate::KeyboardConfig;
use std::sync::Arc;

#[derive(Debug)]
pub struct KeyboardManager {
    pub keyboard_config: Arc<KeyboardConfig>,
}

impl KeyboardManager {
    pub fn new(config: KeyboardConfig) -> Self {
        Self {
            keyboard_config: Arc::new(config),
        }
    }
    pub fn keyboard_layout(&self) -> String {
        self.keyboard_config.keyboard_layout().unwrap_or("us".to_string())
    }
    pub fn keyboard_variant(&self) -> Option<String> {
        self.keyboard_config.keyboard_variant()
    }

    pub fn set_keyboard_layout(&mut self, keyboard_layout: String) {
        self.keyboard_config.set_keyboard_layout(keyboard_layout);
    }

    pub fn set_keyboard_variant(&mut self, keyboard_variant: String) {
        self.keyboard_config.set_keyboard_variant(keyboard_variant);
    }
}
