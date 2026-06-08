use atomic_float::AtomicF32;
use atomic_float::AtomicF64;
use clap::Parser;
use smearor_wrot_application::RotationConfig;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;

#[derive(Parser, Debug, Clone)]
pub struct RotationArguments {
    /// Disable the rotation widget even if a rotation value is provided.
    #[arg(short = 'R', long, action)]
    pub(crate) disable_rotation: bool,

    /// Rotation angle in degrees.
    #[arg(short, long, default_value_t = 0.0)]
    pub(crate) rotation: f32,

    /// Animation speed in milliseconds for rotation overshoot animation (default: 500).
    #[arg(long, default_value_t = 500)]
    pub(crate) animation_speed: u64,

    /// Animation overshoot amount for rotation gesture (default: 1.7).
    #[arg(long, default_value_t = 1.7)]
    pub(crate) animation_overshoot: f64,

    /// Disable all animations.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_animations: bool,
}

impl From<RotationArguments> for RotationConfig {
    fn from(args: RotationArguments) -> Self {
        Self {
            disable_rotation: AtomicBool::new(args.disable_rotation),
            rotation: AtomicF32::new(args.rotation),
            animation_speed: AtomicU64::new(args.animation_speed),
            animation_overshoot: AtomicF64::new(args.animation_overshoot),
            disable_animations: AtomicBool::new(args.disable_animations),
        }
    }
}
