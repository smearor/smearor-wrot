use crate::color_mask::color_mask_applier::ColorMaskApplier;
use glib::Bytes;
use gtk4::Snapshot;
use gtk4::gdk;
use gtk4::graphene::Rect;
use gtk4::prelude::SnapshotExt;
use gtk4::prelude::TextureExt;
use gtk4::prelude::TextureExtManual;
use smearor_wrot_compositor::color_mask::mask::ColorMask;
use smearor_wrot_model::color::hex::ToHex;
use tracing::debug;

/// CPU-based color mask applier for SHM textures
///
/// This implementation applies color masks by directly modifying pixel data
/// in CPU memory before creating the texture. This is suitable for SHM buffers
/// where pixel data is already accessible in system memory.
pub struct ShmColorMaskApplier;

impl ColorMaskApplier for ShmColorMaskApplier {
    fn apply_color_mask(&mut self, texture: &gdk::Texture, mask_color: ColorMask, snapshot: &Snapshot, bounds: &Rect) -> Result<(), String> {
        debug!("ShmColorMaskApplier: applying color mask {mask_color}");

        let tolerance_sq = mask_color.tolerance * mask_color.tolerance;
        debug!("ShmColorMaskApplier: mask color {} with tolerance {}", mask_color.to_hex(), mask_color.tolerance);

        // Download texture data (returns BGRA format)
        let width = texture.width();
        let height = texture.height();
        let stride = width * 4;
        let mut data = vec![0u8; (stride * height) as usize];
        texture.download(&mut data, stride as usize);

        debug!("ShmColorMaskApplier: downloaded {} bytes from texture ({}x{})", data.len(), width, height);

        // Apply chroma-keying (make transparent)
        for i in (0..data.len()).step_by(4) {
            let b = data[i] as f32 / 255.0;
            let g = data[i + 1] as f32 / 255.0;
            let r = data[i + 2] as f32 / 255.0;
            // Alpha is at i + 3

            let dr = r - mask_color.color.red;
            let dg = g - mask_color.color.green;
            let db = b - mask_color.color.blue;

            let distance_sq = dr * dr + dg * dg + db * db;

            if distance_sq <= tolerance_sq {
                data[i + 3] = 0; // Set alpha to 0 (transparent)
            }
        }

        debug!("ShmColorMaskApplier: applied color mask to {} pixels", data.len() / 4);

        // Create new texture from modified data
        let pixel_bytes = Bytes::from(&data[..]);
        let gdk_memory_format = gdk::MemoryFormat::B8g8r8a8;
        let masked_texture = gdk::MemoryTexture::new(width, height, gdk_memory_format, &pixel_bytes, stride as usize);

        debug!("ShmColorMaskApplier: created masked texture {}x{}", width, height);

        // Render masked texture to snapshot
        snapshot.append_texture(&masked_texture, bounds);

        debug!("ShmColorMaskApplier: successfully rendered masked texture to snapshot");
        Ok(())
    }
}
