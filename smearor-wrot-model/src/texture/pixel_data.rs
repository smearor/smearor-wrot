use crate::color::ColorMask;
use crate::color::frequency::ColorFrequencyMap;
use crate::color::rgb::RgbColor24;
use crate::color::rgba::RgbaColor;
#[cfg(feature = "image")]
use crate::geometry::size::Size;
#[cfg(feature = "image")]
use image::ImageBuffer;
#[cfg(feature = "image")]
use image::ImageError;
#[cfg(feature = "image")]
use image::Rgba;
use std::marker::PhantomData;
use std::ops::Deref;
#[cfg(feature = "image")]
use std::path::Path;
#[cfg(feature = "image")]
use thiserror::Error;

/// Pixel data format identifier
pub enum PixelDataFormat {
    BGRA,
    RGBA,
}

/// Error type for pixel data save operations
#[cfg(feature = "image")]
#[derive(Debug, Error)]
pub enum PixelDataSaveError {
    #[error("The buffer creation failed")]
    BufferCreationFailed,
    #[error("Failed to save image {0}")]
    ImageError(#[from] ImageError),
}

/// Marker type for BGRA pixel format
pub struct BGRA;
/// Marker type for RGBA pixel format
pub struct RGBA;

/// Raw pixel data with a phantom type indicating the pixel format.
///
/// Generic over `T` which is either `BGRA` or `RGBA`.
#[derive(Debug, Clone)]
pub struct PixelData<T>(Vec<u8>, PhantomData<T>);

impl<T> Deref for PixelData<T> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<PDF> PixelData<PDF> {
    /// Checks if all pixel data is zero
    pub fn is_zero(&self) -> bool {
        self.iter().all(|&x| x == 0)
    }
}

impl PixelData<BGRA> {
    /// Creates new BGRA pixel data from a byte vector
    pub fn new(pixel_data: Vec<u8>) -> Self {
        Self(pixel_data, PhantomData)
    }

    /// Creates new BGRA pixel data from a byte slice
    pub fn from_slice(pixel_data_slice: &[u8]) -> Self {
        Self(pixel_data_slice.to_vec(), PhantomData)
    }

    /// Returns the pixel data format
    pub fn format(&self) -> PixelDataFormat {
        PixelDataFormat::BGRA
    }

    /// Returns a color frequency map with quantized colors
    pub fn get_frequency_map(&self, quantization_step: u8) -> Option<ColorFrequencyMap<RgbColor24>> {
        if quantization_step == 0 {
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

    /// Returns the dominant color in the pixel data
    pub fn get_dominant_color(&self, quantization_step: u8) -> Option<RgbColor24> {
        self.get_frequency_map(quantization_step)
            .and_then(|color_frequency_map| color_frequency_map.get_dominant_color().map(|color_frequency| color_frequency.color))
    }

    /// Converts BGRA pixel data to RGBA in-place, consuming self
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

    /// Creates a copy of the pixel data converted to RGBA, preserving the original BGRA data.
    ///
    /// Unlike `into_rgba()` which consumes self, this method allocates a new buffer
    /// and leaves the original BGRA data intact. Useful for caching scenarios where
    /// both the original and converted variant are needed.
    pub fn to_rgba(&self) -> PixelData<RGBA> {
        let mut rgba: Vec<u8> = Vec::with_capacity(self.0.len());
        for i in (0..self.0.len()).step_by(4) {
            rgba.push(self.0[i + 2]); // r
            rgba.push(self.0[i + 1]); // g
            rgba.push(self.0[i]); // b
            rgba.push(self.0[i + 3]); // a
        }
        PixelData::<RGBA>::new(rgba)
    }

    /// Replace color in pixel data with a new color.
    /// Replaces pixels matching the mask color with the replacement color.
    pub fn replace_color(&mut self, color_mask: ColorMask, replacement_color: RgbaColor) {
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
                self.0[i] = (replacement_color.color.blue * 255.0) as u8;
                self.0[i + 1] = (replacement_color.color.green * 255.0) as u8;
                self.0[i + 2] = (replacement_color.color.red * 255.0) as u8;
                self.0[i + 3] = (replacement_color.alpha * 255.0) as u8;
            }
        }
    }

    /// Apply color mask to pixel data.
    /// Replaces pixels matching the mask color with transparency (chroma-keying).
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
    /// Creates new RGBA pixel data from a byte vector
    pub fn new(pixel_data: Vec<u8>) -> Self {
        Self(pixel_data, PhantomData)
    }

    /// Creates new RGBA pixel data from a byte slice
    pub fn from_slice(pixel_data_slice: &[u8]) -> Self {
        Self(pixel_data_slice.to_vec(), PhantomData)
    }

    /// Returns the pixel data format
    pub fn format(&self) -> PixelDataFormat {
        PixelDataFormat::RGBA
    }

    /// Returns a color frequency map with quantized colors
    pub fn get_frequency_map(&self, quantization_step: u8) -> Option<ColorFrequencyMap<RgbColor24>> {
        if quantization_step == 0 {
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

    /// Saves the pixel data as a PNG image
    #[cfg(feature = "image")]
    pub fn save_png(&self, path: &Path, size: &Size<u32>) -> Result<(), PixelDataSaveError> {
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(size.width, size.height, self.0.as_slice()).ok_or(PixelDataSaveError::BufferCreationFailed)?;
        buffer.save(path).map_err(PixelDataSaveError::ImageError)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_data_bgra_new() {
        let data = PixelData::<BGRA>::new(vec![1, 2, 3, 4]);
        assert_eq!(data.len(), 4);
        assert_eq!(data[0], 1);
    }

    #[test]
    fn test_pixel_data_bgra_from_slice() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8];
        let data = PixelData::<BGRA>::from_slice(&slice);
        assert_eq!(data.len(), 8);
    }

    #[test]
    fn test_pixel_data_is_zero() {
        let zero_data = PixelData::<BGRA>::new(vec![0, 0, 0, 0]);
        assert!(zero_data.is_zero());

        let non_zero = PixelData::<BGRA>::new(vec![0, 0, 0, 1]);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_pixel_data_bgra_into_rgba() {
        let bgra = PixelData::<BGRA>::new(vec![10, 20, 30, 40]);
        let rgba = bgra.into_rgba();
        assert_eq!(rgba[0], 30); // r
        assert_eq!(rgba[1], 20); // g
        assert_eq!(rgba[2], 10); // b
        assert_eq!(rgba[3], 40); // a
    }

    #[test]
    fn test_pixel_data_bgra_to_rgba_ref() {
        let bgra = PixelData::<BGRA>::new(vec![10, 20, 30, 40]);
        let rgba: PixelData<RGBA> = (&bgra).into();
        assert_eq!(rgba[0], 30);
        assert_eq!(rgba[1], 20);
        assert_eq!(rgba[2], 10);
        assert_eq!(rgba[3], 40);
    }

    #[test]
    fn test_pixel_data_bgra_to_rgba_copy() {
        let bgra = PixelData::<BGRA>::new(vec![10, 20, 30, 40, 50, 60, 70, 80]);
        let rgba = bgra.to_rgba();
        // Original is preserved
        assert_eq!(bgra[0], 10);
        assert_eq!(bgra[1], 20);
        assert_eq!(bgra[2], 30);
        assert_eq!(bgra[3], 40);
        // Copy is converted
        assert_eq!(rgba[0], 30); // r from first pixel
        assert_eq!(rgba[1], 20); // g
        assert_eq!(rgba[2], 10); // b
        assert_eq!(rgba[3], 40); // a
        assert_eq!(rgba[4], 70); // r from second pixel
        assert_eq!(rgba[5], 60); // g
        assert_eq!(rgba[6], 50); // b
        assert_eq!(rgba[7], 80); // a
    }

    #[test]
    fn test_pixel_data_rgba_to_bgra_ref() {
        let rgba = PixelData::<RGBA>::new(vec![30, 20, 10, 40]);
        let bgra: PixelData<BGRA> = (&rgba).into();
        assert_eq!(bgra[0], 10);
        assert_eq!(bgra[1], 20);
        assert_eq!(bgra[2], 30);
        assert_eq!(bgra[3], 40);
    }

    #[test]
    fn test_pixel_data_apply_color_mask() {
        let mut data = PixelData::<BGRA>::new(vec![
            0, 0, 255, 255, // BGRA: B=0, G=0, R=255, A=255 -> r=1.0 matches mask r=1.0
            0, 0, 0, 255, // BGRA: B=0, G=0, R=0, A=255 -> r=0.0 does not match mask r=1.0
        ]);
        let mask = ColorMask::new(crate::color::RgbColor::new(1.0, 0.0, 0.0), 0.1);
        data.apply_color_mask(mask);
        // First pixel matches mask -> alpha set to 0
        assert_eq!(data[3], 0);
        // Second pixel does not match -> alpha unchanged
        assert_eq!(data[7], 255);
    }

    #[test]
    fn test_pixel_data_apply_color_mask_match() {
        let mut data = PixelData::<BGRA>::new(vec![0, 0, 255, 255]);
        // Mask matches red=1.0 (255 in u8), which is the R channel at index 2
        let mask = ColorMask::new(crate::color::RgbColor::new(1.0, 0.0, 0.0), 0.1);
        data.apply_color_mask(mask);
        // The pixel has r=255/255=1.0, g=0, b=0 -> matches mask -> alpha set to 0
        assert_eq!(data[3], 0);
    }

    #[test]
    fn test_pixel_data_bgra_get_frequency_map() {
        // Two red pixels, one blue pixel (BGRA format)
        let data = PixelData::<BGRA>::new(vec![
            0, 0, 255, 255, // red
            0, 0, 255, 255, // red
            255, 0, 0, 255, // blue
        ]);
        let freq_map = data.get_frequency_map(16).unwrap();
        // With quantization step 16, colors are quantized to multiples of 16
        // Red: (0, 0, 255) -> quantized (0, 0, 240) -> count 2
        // Blue: (255, 0, 0) -> quantized (240, 0, 0) -> count 1
        assert_eq!(freq_map.0.len(), 2);
    }

    #[test]
    fn test_pixel_data_bgra_get_frequency_map_empty() {
        let data = PixelData::<BGRA>::new(vec![]);
        assert!(data.get_frequency_map(16).is_none());
    }

    #[test]
    fn test_pixel_data_bgra_get_frequency_map_zero_step() {
        let data = PixelData::<BGRA>::new(vec![0, 0, 255, 255]);
        assert!(data.get_frequency_map(0).is_none());
    }

    #[test]
    fn test_pixel_data_bgra_get_dominant_color() {
        // Three red pixels, one blue pixel (BGRA format)
        let data = PixelData::<BGRA>::new(vec![
            0, 0, 255, 255, // red
            0, 0, 255, 255, // red
            0, 0, 255, 255, // red
            255, 0, 0, 255, // blue
        ]);
        let dominant = data.get_dominant_color(16).unwrap();
        // Red (0, 0, 255) quantized to (0, 0, 240) should be dominant
        assert_eq!(dominant.red, 240);
        assert_eq!(dominant.green, 0);
        assert_eq!(dominant.blue, 0);
    }

    #[test]
    fn test_pixel_data_rgba_get_frequency_map() {
        // Two red pixels, one blue pixel (RGBA format)
        let data = PixelData::<RGBA>::new(vec![
            255, 0, 0, 255, // red
            255, 0, 0, 255, // red
            0, 0, 255, 255, // blue
        ]);
        let freq_map = data.get_frequency_map(16).unwrap();
        assert_eq!(freq_map.0.len(), 2);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_pixel_data_save_png() {
        use crate::geometry::size::Size;
        use std::path::PathBuf;

        // 2x2 RGBA image, all red
        let data = PixelData::<RGBA>::new(vec![
            255, 0, 0, 255, 255, 0, 0, 255,
            255, 0, 0, 255, 255, 0, 0, 255,
        ]);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_save_png.png");
        let size = Size::new(2, 2);
        data.save_png(&path, &size).unwrap();
        assert!(path.exists());
        // Clean up
        let _ = std::fs::remove_file(&path);
    }
}
