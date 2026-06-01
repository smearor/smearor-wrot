//! GTK4 Paintable for dynamic texture display

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::TextureExt;

/// Paintable implementation for displaying dynamic textures
#[derive(Debug, Clone)]
pub struct TexturePaintable {
    texture: gdk::Texture,
    width: i32,
    height: i32,
}

impl TexturePaintable {
    /// Create a new TexturePaintable from RGBA data
    pub fn from_rgba(data: &[u8], width: i32, height: i32) -> Self {
        let bytes = glib::Bytes::from(data);
        let stride = width * 4; // 4 bytes per pixel (RGBA)
        let texture = gdk::MemoryTexture::new(
            width,
            height,
            gdk::MemoryFormat::B8g8r8a8,
            &bytes,
            stride as usize,
        );

        Self {
            texture: texture.into(),
            width,
            height,
        }
    }

    /// Create a simple test texture with a color pattern
    pub fn create_test_pattern(width: i32, height: i32) -> Self {
        let mut data = Vec::with_capacity((width * height * 4) as usize);

        // Gradient from 10% dark gray (top) to 20% dark gray (bottom)
        // Both with 50% transparency
        let top_gray = 26; // 10% of 255
        let bottom_gray = 51; // 20% of 255
        let alpha = 128; // 50% transparency

        for y in 0..height {
            let y_ratio = y as f32 / height as f32;
            let gray = (top_gray as f32 + (bottom_gray as f32 - top_gray as f32) * y_ratio) as u8;

            for _x in 0..width {
                data.push(gray);
                data.push(gray);
                data.push(gray);
                data.push(alpha);
            }
        }

        Self::from_rgba(&data, width, height)
    }

    /// Get the underlying texture
    pub fn texture(&self) -> &gdk::Texture {
        &self.texture
    }

    /// Get the texture size
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// Create a new TexturePaintable from a gdk::Texture
    pub fn from_gdk_texture(texture: &gdk::Texture) -> Self {
        let width = texture.width();
        let height = texture.height();
        Self {
            texture: texture.clone(),
            width,
            height,
        }
    }
}

impl Default for TexturePaintable {
    fn default() -> Self {
        Self::create_test_pattern(640, 480)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_paintable_creation() {
        let paintable = TexturePaintable::create_test_pattern(100, 100);
        let (width, height) = paintable.size();
        assert_eq!(width, 100);
        assert_eq!(height, 100);
    }

    #[test]
    fn test_texture_paintable_from_rgba() {
        let data = vec![255u8, 0, 0, 255]; // 1x1 red pixel (RGBA)
        let paintable = TexturePaintable::from_rgba(&data, 1, 1);
        let (width, height) = paintable.size();
        assert_eq!(width, 1);
        assert_eq!(height, 1);
    }
}
