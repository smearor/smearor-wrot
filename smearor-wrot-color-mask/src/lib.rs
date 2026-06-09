pub mod color_mask;
pub mod error;
pub mod manager;
pub mod state;

pub const DEFAULT_COLOR_MASK_TOLERANCE: f32 = 0.1;

pub use color_mask::ColorMask;
pub use error::ColorMaskError;
pub use manager::ColorMaskManager;
pub use state::ColorMaskState;
