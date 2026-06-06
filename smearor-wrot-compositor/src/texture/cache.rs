use crate::buffer::metadata::BufferMetadata;
use crate::color_mask::mask::ColorMask;
use crate::texture::pixel_data::BGRA;
use crate::texture::pixel_data::PixelData;
use smearor_wrot_model::color::rgb::RgbColor24;
use smearor_wrot_model::color::rgba::RgbaColor;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TextureCacheEntry<PDF> {
    pub buffer_metadata: BufferMetadata,
    pub pixel_data: PixelData<PDF>,
    pub color_mask_applied: bool,
    pub commit_count: u32,
}

impl<PDF> TextureCacheEntry<PDF> {
    pub fn new(buffer_metadata: BufferMetadata, pixel_data: PixelData<PDF>) -> Self {
        Self {
            buffer_metadata,
            pixel_data,
            color_mask_applied: false,
            commit_count: 0,
        }
    }

    pub fn with_commit_count(buffer_metadata: BufferMetadata, pixel_data: PixelData<PDF>, commit_count: u32) -> Self {
        Self {
            buffer_metadata,
            pixel_data,
            color_mask_applied: false,
            commit_count,
        }
    }
}

impl<PDF> Display for TextureCacheEntry<PDF> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TextureCacheEntry {{ buffer_metadata: {:?}, pixel_data: {:?} }}",
            self.buffer_metadata,
            self.pixel_data.len()
        )
    }
}

impl TextureCacheEntry<BGRA> {
    pub fn get_dominant_color(&self, quantization_step: u8) -> Option<RgbColor24> {
        self.pixel_data.get_dominant_color(quantization_step)
    }

    pub fn replace_color(&mut self, color_mask: ColorMask, replacement_color: RgbaColor) {
        self.pixel_data.replace_color(color_mask, replacement_color);
        self.color_mask_applied = true;
    }

    pub fn apply_color_mask(&mut self, color_mask: ColorMask) {
        self.pixel_data.apply_color_mask(color_mask);
        self.color_mask_applied = true;
    }
}
