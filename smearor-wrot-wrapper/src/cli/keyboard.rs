use clap::Parser;
use smearor_wrot_application::KeyboardState;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Parser, Debug, Clone)]
pub struct KeyboardArguments {
    /// Keyboard layout (e.g., "de", "us"). Overrides automatic detection.
    #[arg(long)]
    pub(crate) keyboard_layout: Option<String>,

    /// Keyboard variant (e.g., "nodeadkeys"). Overrides automatic detection.
    #[arg(long)]
    pub(crate) keyboard_variant: Option<String>,
}

impl From<KeyboardArguments> for KeyboardState {
    fn from(args: KeyboardArguments) -> Self {
        Self {
            keyboard_layout: Arc::new(RwLock::new(args.keyboard_layout)),
            keyboard_variant: Arc::new(RwLock::new(args.keyboard_variant)),
        }
    }
}
