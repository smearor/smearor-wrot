use crate::DEFAULT_COLOR_MASK_TOLERANCE;
use atomic_float::AtomicF32;
use std::sync::atomic::AtomicBool;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct ColorMaskState {
    #[builder(default)]
    pub background_color: Option<String>,

    #[builder(default)]
    pub color_mask: Option<String>,

    #[builder(default)]
    pub auto_color_mask: AtomicBool,

    #[builder(default)]
    pub subsurface_background_color: Option<String>,

    #[builder(default)]
    pub subsurface_color_mask: Option<String>,

    #[builder(default)]
    pub auto_subsurface_color_mask: AtomicBool,

    #[builder(default = default_color_mask_tolerance())]
    pub color_mask_tolerance: AtomicF32,

    #[builder(default)]
    pub color_mask_shader: AtomicBool,
}

fn default_color_mask_tolerance() -> AtomicF32 {
    AtomicF32::new(DEFAULT_COLOR_MASK_TOLERANCE)
}
