use crate::color_mask::mask::ColorMask;
use image::ImageBuffer;
use image::ImageError;
use image::Rgba;
use smearor_wrot_model::color::frequency::ColorFrequencyMap;
use smearor_wrot_model::color::rgb::RgbColor24;
use smearor_wrot_model::color::rgba::RgbaColor;
use smearor_wrot_model::geometry::size::Size;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;
use thiserror::Error;

pub enum PixelDataFormat {
    BGRA,
    RGBA,
}

#[derive(Debug, Error)]
pub enum PixelDataSaveError {
    #[error("The buffer creation failed")]
    BufferCreationFailed,
    #[error("Failed to save image {0}")]
    ImageError(#[from] ImageError),
}

pub struct BGRA;
pub struct RGBA;

#[derive(Debug, Clone)]
pub struct PixelData<T>(Vec<u8>, PhantomData<T>);

impl<T> Deref for PixelData<T> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<PDF> PixelData<PDF> {
    pub fn is_zero(&self) -> bool {
        self.iter().all(|&x| x == 0)
    }
}

impl PixelData<BGRA> {
    pub fn new(pixel_data: Vec<u8>) -> Self {
        Self(pixel_data, PhantomData)
    }

    pub fn from_slice(pixel_data_slice: &[u8]) -> Self {
        Self(pixel_data_slice.to_vec(), PhantomData)
    }

    pub fn format(&self) -> PixelDataFormat {
        PixelDataFormat::BGRA
    }

    pub fn get_frequency_map(&self, quantization_step: u8) -> Option<ColorFrequencyMap<RgbColor24>> {
        if quantization_step <= 0 {
            return None;
        }
        if self.0.len() < 4 {
            return None;
        }

        let color_frequency_map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        for i in (0..self.0.len()).step_by(4) {
            let b = self.0[i];
            let g = self.0[i + 1];
            let r = self.0[i + 2];
            // Skip alpha channel at i + 3

            // Quantize colors to group similar colors
            let quantized_r = (r / quantization_step) * quantization_step;
            let quantized_g = (g / quantization_step) * quantization_step;
            let quantized_b = (b / quantization_step) * quantization_step;

            let color = RgbColor24::new(quantized_r, quantized_g, quantized_b);
            *color_frequency_map.0.entry(color).or_insert(0) += 1;
        }
        Some(color_frequency_map)
    }

    pub fn get_dominant_color(&self, quantization_step: u8) -> Option<RgbColor24> {
        self.get_frequency_map(quantization_step)
            .and_then(|color_frequency_map| color_frequency_map.get_dominant_color().map(|color_frequency| color_frequency.color))
    }

    pub fn into_rgba(mut self) -> PixelData<RGBA> {
        // Convert BGRA to RGBA for consistent color detection
        for i in (0..self.0.len()).step_by(4) {
            let b = self.0[i];
            let g = self.0[i + 1];
            let r = self.0[i + 2];
            let a = self.0[i + 3];

            // Swap to RGBA
            self.0[i] = r;
            self.0[i + 1] = g;
            self.0[i + 2] = b;
            self.0[i + 3] = a;
        }
        PixelData::<RGBA>::new(self.0)
    }

    /// Replace color in pixel data with a new color
    /// Replaces pixels matching the mask color with the replacement color
    pub fn replace_color(&mut self, color_mask: ColorMask, replacement_color: RgbaColor) {
        // let (mask_r, mask_g, mask_b, tolerance) = mask_color;
        let tolerance_sq = color_mask.tolerance * color_mask.tolerance;

        for i in (0..self.0.len()).step_by(4) {
            // TODO: check this
            let b = self.0[i] as f32 / 255.0;
            let g = self.0[i + 1] as f32 / 255.0;
            let r = self.0[i + 2] as f32 / 255.0;
            // Alpha is at i + 3

            let dr = r - color_mask.color.red;
            let dg = g - color_mask.color.green;
            let db = b - color_mask.color.blue;

            let distance_sq = dr * dr + dg * dg + db * db;

            if distance_sq <= tolerance_sq {
                self.0[i] = (replacement_color.color.blue * 255.0) as u8;
                self.0[i + 1] = (replacement_color.color.green * 255.0) as u8;
                self.0[i + 2] = (replacement_color.color.red * 255.0) as u8;
                self.0[i + 3] = (replacement_color.alpha * 255.0) as u8;
            }
        }
    }

    /// Apply color mask to pixel data
    /// Replaces pixels matching the mask color with transparency (chroma-keying)
    pub fn apply_color_mask(&mut self, color_mask: ColorMask) {
        let tolerance_sq = color_mask.tolerance * color_mask.tolerance;
        for i in (0..self.0.len()).step_by(4) {
            let b = self.0[i] as f32 / 255.0;
            let g = self.0[i + 1] as f32 / 255.0;
            let r = self.0[i + 2] as f32 / 255.0;
            // Alpha is at i + 3

            let dr = r - color_mask.color.red;
            let dg = g - color_mask.color.green;
            let db = b - color_mask.color.blue;

            let distance_sq = dr * dr + dg * dg + db * db;

            if distance_sq <= tolerance_sq {
                // Set alpha to 0 (transparent)
                self.0[i + 3] = 0;
            }
        }
    }
}

impl PixelData<RGBA> {
    pub fn new(pixel_data: Vec<u8>) -> Self {
        Self(pixel_data, PhantomData)
    }

    pub fn from_slice(pixel_data_slice: &[u8]) -> Self {
        Self(pixel_data_slice.to_vec(), PhantomData)
    }

    pub fn format(&self) -> PixelDataFormat {
        PixelDataFormat::RGBA
    }

    pub fn get_frequency_map(&self, quantization_step: u8) -> Option<ColorFrequencyMap<RgbColor24>> {
        if quantization_step <= 0 {
            return None;
        }
        if self.0.len() < 4 {
            return None;
        }

        let color_frequency_map: ColorFrequencyMap<RgbColor24> = ColorFrequencyMap::new();
        for i in (0..self.0.len()).step_by(4) {
            let r = self.0[i];
            let g = self.0[i + 1];
            let b = self.0[i + 2];
            // Skip alpha channel at i + 3

            // Quantize colors to group similar colors
            let quantized_r = (r / quantization_step) * quantization_step;
            let quantized_g = (g / quantization_step) * quantization_step;
            let quantized_b = (b / quantization_step) * quantization_step;

            let color = RgbColor24::new(quantized_r, quantized_g, quantized_b);
            *color_frequency_map.0.entry(color).or_insert(0) += 1;
        }
        Some(color_frequency_map)
    }

    pub fn save_png(&self, path: &Path, size: &Size<u32>) -> Result<(), PixelDataSaveError> {
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(size.width, size.height, self.0.as_slice()).ok_or(PixelDataSaveError::BufferCreationFailed)?;
        buffer.save(path).map_err(|e| PixelDataSaveError::ImageError(e))?;
        Ok(())
    }
}

impl From<&PixelData<BGRA>> for PixelData<RGBA> {
    fn from(bgra: &PixelData<BGRA>) -> Self {
        let mut rgba: Vec<u8> = vec![0u8; bgra.0.len()];
        for i in (0..bgra.0.len()).step_by(4) {
            rgba[i] = bgra.0[i + 2];
            rgba[i + 1] = bgra.0[i + 1];
            rgba[i + 2] = bgra.0[i];
            rgba[i + 3] = bgra.0[i + 3];
        }
        PixelData::<RGBA>::new(rgba)
    }
}

impl From<&PixelData<RGBA>> for PixelData<BGRA> {
    fn from(rgba: &PixelData<RGBA>) -> Self {
        let mut bgra: Vec<u8> = vec![0u8; rgba.0.len()];
        for i in (0..rgba.0.len()).step_by(4) {
            bgra[i] = rgba.0[i + 2];
            bgra[i + 1] = rgba.0[i + 1];
            bgra[i + 2] = rgba.0[i];
            bgra[i + 3] = rgba.0[i + 3];
        }
        PixelData::<BGRA>::new(bgra)
    }
}
