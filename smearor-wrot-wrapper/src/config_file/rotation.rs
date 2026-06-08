use crate::cli::rotation::RotationArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct RotationConfigFile {
    /// Disable the rotation widget even if a rotation value is provided.
    #[serde(default)]
    pub(crate) disable_rotation: Option<bool>,

    /// Rotation angle in degrees.
    #[serde(default)]
    pub(crate) rotation: Option<f32>,

    /// Animation speed in milliseconds for rotation overshoot animation (default: 500).
    #[serde(default)]
    pub(crate) animation_speed: Option<u64>,

    /// Animation overshoot amount for rotation gesture (default: 1.7).
    #[serde(default)]
    pub(crate) animation_overshoot: Option<f64>,

    /// Disable all animations.
    #[serde(default)]
    pub(crate) disable_animations: Option<bool>,
}

impl MergeWithConfigFile<RotationConfigFile> for RotationArguments {
    fn merge_with_config_file(mut self, config: &RotationConfigFile) -> Self {
        if !self.disable_rotation
            && let Some(disable_rotation) = config.disable_rotation
        {
            self.disable_rotation = disable_rotation;
        }
        if self.rotation == 0.0
            && let Some(rotation) = config.rotation
        {
            self.rotation = rotation;
        }
        if self.animation_speed == 500
            && let Some(animation_speed) = config.animation_speed
        {
            self.animation_speed = animation_speed;
        }
        if self.animation_overshoot == 1.7
            && let Some(animation_overshoot) = config.animation_overshoot
        {
            self.animation_overshoot = animation_overshoot;
        }
        if !self.disable_animations
            && let Some(disable_animations) = config.disable_animations
        {
            self.disable_animations = disable_animations;
        }
        self
    }
}
