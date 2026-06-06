//! Texture utilities
//!
//! This module provides functions for extracting and manipulating pixel data from GDK textures.

use gtk4::gdk;
use gtk4::gdk::prelude::*;
use smearor_wrot_compositor::texture::pixel_data::BGRA;
use smearor_wrot_compositor::texture::pixel_data::PixelData;

/// Extract pixel data from a GDK texture for color detection
///
/// This downloads the texture data and converts it to a format suitable for color detection.
/// GDK texture.download() returns data in BGRA format, so we convert to RGBA for consistency.
pub fn extract_pixel_data_from_texture(texture: &gdk::Texture) -> PixelData<BGRA> {
    // Get texture dimensions using the correct GTK4 API
    let width = texture.width();
    let height = texture.height();

    // Calculate stride (bytes per row) for BGRA format
    let stride = width * 4;

    // Create buffer with correct size
    let mut data = vec![0u8; (stride * height) as usize];

    // Download texture data (returns BGRA format)
    texture.download(&mut data, stride as usize);

    PixelData::<BGRA>::new(data)
}
