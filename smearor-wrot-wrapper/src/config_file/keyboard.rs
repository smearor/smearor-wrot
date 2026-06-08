use crate::cli::keyboard::KeyboardArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct KeyboardConfigFile {
    /// Keyboard layout (e.g., "de", "us"). Overrides automatic detection.
    #[serde(default)]
    pub(crate) keyboard_layout: Option<String>,

    /// Keyboard variant (e.g., "nodeadkeys"). Overrides automatic detection.
    #[serde(default)]
    pub(crate) keyboard_variant: Option<String>,
}

impl MergeWithConfigFile<KeyboardConfigFile> for KeyboardArguments {
    fn merge_with_config_file(mut self, config: &KeyboardConfigFile) -> Self {
        if self.keyboard_layout.is_none() && config.keyboard_layout.is_some() {
            self.keyboard_layout = config.keyboard_layout.clone();
        }
        if self.keyboard_variant.is_none() && config.keyboard_variant.is_some() {
            self.keyboard_variant = config.keyboard_variant.clone();
        }
        self
    }
}
