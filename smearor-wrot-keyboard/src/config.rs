use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct KeyboardConfig {
    #[builder(default)]
    pub keyboard_layout: Option<String>,

    #[builder(default)]
    pub keyboard_variant: Option<String>,
}
