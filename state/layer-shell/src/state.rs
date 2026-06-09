use smearor_wrot_rotation::layer::SmearorLayer;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct LayerShellState {
    /// The layer position
    #[builder(default)]
    pub layer: Option<SmearorLayer>,

    /// The layer shell namespace
    #[builder(default)]
    pub namespace: Option<String>,
}
