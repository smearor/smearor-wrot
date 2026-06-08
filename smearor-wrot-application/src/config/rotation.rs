use atomic_float::AtomicF32;
use atomic_float::AtomicF64;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct RotationConfig {
    #[builder(default)]
    pub disable_rotation: AtomicBool,

    #[builder(default = 0.0)]
    pub rotation: AtomicF32,

    #[builder(default = 500)]
    pub animation_speed: AtomicU64,

    #[builder(default = 1.7)]
    pub animation_overshoot: AtomicF64,

    #[builder(default)]
    pub disable_animations: AtomicBool,
}
