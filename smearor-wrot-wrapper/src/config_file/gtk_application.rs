use crate::cli::gtk_application::GtkApplicationArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct GtkApplicationConfigFile {
    /// The application id.
    #[serde(default)]
    pub id: Option<String>,

    /// Maximum frames per second (default: 60).
    #[serde(default)]
    pub max_fps: Option<i64>,
}

impl MergeWithConfigFile<GtkApplicationConfigFile> for GtkApplicationArguments {
    fn merge_with_config_file(mut self, config: &GtkApplicationConfigFile) -> Self {
        if self.id.is_none() && config.id.is_some() {
            self.id = config.id.clone();
        }
        if self.max_fps == 60
            && let Some(max_fps) = config.max_fps
        {
            self.max_fps = max_fps;
        }
        self
    }
}
