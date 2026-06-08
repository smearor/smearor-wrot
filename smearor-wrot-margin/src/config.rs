use std::sync::atomic::AtomicU32;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct MarginConfig {
    pub margin_left: AtomicU32,

    pub margin_right: AtomicU32,

    pub margin_top: AtomicU32,

    pub margin_bottom: AtomicU32,

    pub dialog_margin: AtomicU32,
}
