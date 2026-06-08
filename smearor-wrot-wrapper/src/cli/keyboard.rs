use clap::Parser;
use smearor_wrot_application::KeyboardConfig;

#[derive(Parser, Debug, Clone)]
pub struct KeyboardArguments {
    /// Keyboard layout (e.g., "de", "us"). Overrides automatic detection.
    #[arg(long)]
    pub(crate) keyboard_layout: Option<String>,

    /// Keyboard variant (e.g., "nodeadkeys"). Overrides automatic detection.
    #[arg(long)]
    pub(crate) keyboard_variant: Option<String>,
}

impl From<KeyboardArguments> for KeyboardConfig {
    fn from(args: KeyboardArguments) -> Self {
        Self {
            keyboard_layout: args.keyboard_layout,
            keyboard_variant: args.keyboard_variant,
        }
    }
}
