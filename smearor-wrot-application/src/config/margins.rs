use std::sync::atomic::AtomicU32;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct MarginConfig {
    // #[builder(default)]
    // pub margin: Option<AtomicU32>,
    #[builder(default = 0)]
    pub margin_left: AtomicU32,

    #[builder(default = 0)]
    pub margin_right: AtomicU32,

    #[builder(default = 0)]
    pub margin_top: AtomicU32,

    #[builder(default = 0)]
    pub margin_bottom: AtomicU32,

    #[builder(default = 0)]
    pub dialog_margin: AtomicU32,
}
