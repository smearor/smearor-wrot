//! Color mask application trait and implementations
//!
//! This module provides a trait-based abstraction for applying color masks
//! to textures, with separate implementations for SHM (CPU-based) and DMA-BUF (shader-based)
//! render paths.
//!
//! It also provides utility functions for CPU-based color mask application on raw pixel data.

use gtk4::Snapshot;
use gtk4::gdk;
use gtk4::graphene::Rect;
use smearor_wrot_core::color_mask::mask::ColorMask;

/// Trait for applying color masks to textures
///
/// This trait provides a unified interface for color masking across different
/// render paths (SHM vs DMA-BUF), allowing the rendering pipeline to use the
/// appropriate implementation based on the texture type.
pub trait ColorMaskApplier {
    /// Apply color mask to a texture and render it to a snapshot
    ///
    /// # Arguments
    ///
    /// * `texture` - The texture to apply the color mask to
    /// * `mask_color` - The mask color as (r, g, b, tolerance) in 0.0-1.0 range
    /// * `snapshot` - The GTK snapshot to render to
    /// * `bounds` - The rendering bounds for the texture
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the color mask was applied successfully
    /// * `Err(String)` - If the color mask application failed
    fn apply_color_mask(&mut self, texture: &gdk::Texture, mask_color: ColorMask, snapshot: &Snapshot, bounds: &Rect) -> Result<(), String>;
}

pub mod dma_buf;
pub mod open_gl;
pub mod shm;
