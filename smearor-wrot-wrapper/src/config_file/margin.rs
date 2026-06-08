use crate::cli::margin::MarginArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

/// Compositor configuration section
#[derive(Debug, Deserialize, Default)]
pub struct MarginConfigFile {
    /// Dialog margin in pixels for dialog positioning (default: 0)
    #[serde(default)]
    pub dialog_margin: Option<u32>,

    /// Left margin in pixels for window rendering
    #[serde(default)]
    pub margin_left: Option<u32>,

    /// Right margin in pixels for window rendering
    #[serde(default)]
    pub margin_right: Option<u32>,

    /// Top margin in pixels for window rendering
    #[serde(default)]
    pub margin_top: Option<u32>,

    /// Bottom margin in pixels for window rendering
    #[serde(default)]
    pub margin_bottom: Option<u32>,
}

impl MergeWithConfigFile<MarginConfigFile> for MarginArguments {
    fn merge_with_config_file(mut self, config: &MarginConfigFile) -> Self {
        if self.dialog_margin == 0
            && let Some(dialog_margin) = config.dialog_margin
        {
            self.dialog_margin = dialog_margin;
        }
        if self.margin_left == 0
            && let Some(margin_left) = config.margin_left
        {
            self.margin_left = margin_left;
        }
        if self.margin_right == 0
            && let Some(margin_right) = config.margin_right
        {
            self.margin_right = margin_right;
        }
        if self.margin_top == 0
            && let Some(margin_top) = config.margin_top
        {
            self.margin_top = margin_top;
        }
        if self.margin_bottom == 0
            && let Some(margin_bottom) = config.margin_bottom
        {
            self.margin_bottom = margin_bottom;
        }
        self
    }
}
