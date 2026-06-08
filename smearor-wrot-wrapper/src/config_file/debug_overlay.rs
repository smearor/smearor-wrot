use crate::cli::debug_overlay::DebugOverlayArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct DebugOverlayConfigFile {
    /// Enable visual debugging of pointer.
    #[serde(default)]
    pub debug_pointer: Option<bool>,

    /// Enable visual debugging of touch points.
    #[serde(default)]
    pub debug_touch: Option<bool>,
}

impl MergeWithConfigFile<DebugOverlayConfigFile> for DebugOverlayArguments {
    fn merge_with_config_file(mut self, config: &DebugOverlayConfigFile) -> Self {
        if !self.debug_pointer
            && let Some(debug_pointer) = config.debug_pointer
        {
            self.debug_pointer = debug_pointer;
        }
        if !self.debug_touch
            && let Some(debug_touch) = config.debug_touch
        {
            self.debug_touch = debug_touch;
        }
        self
    }
}
