use crate::cli::layer::LayerArguments;
use crate::config_file::merge::MergeWithConfigFile;
use serde::Deserialize;
use smearor_wrot_application::SmearorLayer;

#[derive(Debug, Deserialize, Default)]
pub struct LayerConfigFile {
    /// Specify the layer for the layer shell protocol (e.g., Background, Top).
    #[serde(default)]
    pub(crate) layer: Option<SmearorLayer>,

    /// Namespace for the layer shell, used by compositors for rules.
    #[serde(default)]
    pub(crate) namespace: Option<String>,
}
impl MergeWithConfigFile<LayerConfigFile> for LayerArguments {
    fn merge_with_config_file(mut self, config: &LayerConfigFile) -> Self {
        if self.layer.is_none() && config.layer.is_some() {
            self.layer = config.layer.clone();
        }
        if self.namespace.is_none() && config.namespace.is_some() {
            self.namespace = config.namespace.clone();
        }
        self
    }
}
