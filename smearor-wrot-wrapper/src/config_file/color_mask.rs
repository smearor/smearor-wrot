use crate::cli::color_mask::ColorMaskArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ColorMaskConfigFile {
    #[serde(default)]
    pub background_color: Option<String>,

    #[serde(default)]
    pub color_mask: Option<String>,

    #[serde(default)]
    pub auto_color_mask: Option<bool>,

    #[serde(default)]
    pub subsurface_background_color: Option<String>,

    #[serde(default)]
    pub subsurface_color_mask: Option<String>,

    #[serde(default)]
    pub auto_subsurface_color_mask: Option<bool>,

    #[serde(default)]
    pub color_mask_tolerance: Option<f32>,

    #[serde(default)]
    pub color_mask_shader: Option<bool>,
}

impl MergeWithConfigFile<ColorMaskConfigFile> for ColorMaskArguments {
    fn merge_with_config_file(mut self, config: &ColorMaskConfigFile) -> Self {
        if self.background_color.is_none() && config.background_color.is_some() {
            self.background_color = config.background_color.clone();
        }
        if self.color_mask.is_none() && config.color_mask.is_some() {
            self.color_mask = config.color_mask.clone();
        }
        if !self.auto_color_mask
            && let Some(auto_color_mask) = config.auto_color_mask
        {
            self.auto_color_mask = auto_color_mask;
        }
        if self.subsurface_background_color.is_none() && config.subsurface_background_color.is_some() {
            self.subsurface_background_color = config.subsurface_background_color.clone();
        }
        if self.subsurface_color_mask.is_none() && config.subsurface_color_mask.is_some() {
            self.subsurface_color_mask = config.subsurface_color_mask.clone();
        }
        if self.auto_subsurface_color_mask
            && let Some(auto_subsurface_color_mask) = config.auto_subsurface_color_mask
        {
            self.auto_subsurface_color_mask = auto_subsurface_color_mask;
        }
        if self.color_mask_tolerance == 0.0
            && let Some(color_mask_tolerance) = config.color_mask_tolerance
        {
            self.color_mask_tolerance = color_mask_tolerance;
        }
        if !self.color_mask_shader
            && let Some(color_mask_shader) = config.color_mask_shader
        {
            self.color_mask_shader = color_mask_shader;
        }
        self
    }
}
