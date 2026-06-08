pub mod color_mask;
pub mod config;
pub mod error;
pub mod manager;

pub const DEFAULT_COLOR_MASK_TOLERANCE: f32 = 0.1;

pub use color_mask::ColorMask;
pub use config::ColorMaskConfig;
pub use error::ColorMaskError;
pub use manager::ColorMaskManager;
