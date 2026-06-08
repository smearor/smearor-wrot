use std::sync::Arc;
use std::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct KeyboardConfig {
    #[builder(default)]
    pub keyboard_layout: Arc<RwLock<Option<String>>>,

    #[builder(default)]
    pub keyboard_variant: Arc<RwLock<Option<String>>>,
}

impl KeyboardConfig {
    pub fn keyboard_layout(&self) -> Option<String> {
        let Ok(guard) = self.keyboard_layout.read() else {
            return None;
        };
        guard.clone()
    }

    pub fn keyboard_variant(&self) -> Option<String> {
        let Ok(guard) = self.keyboard_variant.read() else {
            return None;
        };
        guard.clone()
    }

    pub fn set_keyboard_layout(&self, keyboard_layout: String) {
        let Ok(mut guard) = self.keyboard_layout.write() else {
            return;
        };
        guard.replace(keyboard_layout);
    }
    pub fn set_keyboard_variant(&self, keyboard_variant: String) {
        let Ok(mut guard) = self.keyboard_variant.write() else {
            return;
        };
        guard.replace(keyboard_variant);
    }
}
