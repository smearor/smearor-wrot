//! Rendering pipeline
//!
//! This module provides the rendering pipeline for the compositor including
//! surface rendering, output rendering, and frame management.

pub mod count;
pub mod double_buffer;
pub mod frame_limiter;
pub mod output;
pub mod pipeline;
pub mod surface;

pub use double_buffer::DoubleBuffer;
pub use frame_limiter::FrameLimiter;
pub use output::OutputRendering;
pub use pipeline::RenderingPipeline;
pub use surface::SurfaceRendering;
