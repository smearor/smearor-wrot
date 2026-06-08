use crate::color_mask::color_mask_applier::ColorMaskApplier;
use glib::Bytes;
use gtk4::Snapshot;
use gtk4::gdk;
use gtk4::graphene::Rect;
use gtk4::gsk::GLShader;
use gtk4::gsk::GLShaderNode;
use gtk4::gsk::TextureNode;
use gtk4::prelude::IsRenderNode;
use gtk4::prelude::SnapshotExt;
use smearor_wrot_color::ToHex;
use smearor_wrot_color_mask::ColorMask;
use tracing::debug;

/// Shader-based color mask applier for DMA-BUF textures
///
/// This implementation applies color masks using GLSL shaders during rendering.
/// This is suitable for DMA-BUF textures where pixel data is in GPU memory
/// and CPU-based processing would require expensive buffer reads.
pub struct DmaBufColorMaskApplier {
    shader: Option<GLShader>,
}

impl DmaBufColorMaskApplier {
    /// Create a new DMA-BUF color mask applier
    pub fn new() -> Self {
        Self { shader: None }
    }

    /// Get or create the color mask shader
    #[allow(deprecated)]
    fn get_color_mask_shader(&mut self) -> Option<GLShader> {
        if let Some(shader) = self.shader.as_ref() {
            return Some(shader.clone());
        }

        // Load shader source from file at compile time
        let shader_source = include_str!("color_mask.glsl");

        #[allow(deprecated)]
        let shader = GLShader::from_bytes(&Bytes::from(shader_source.as_bytes()));
        self.shader = Some(shader.clone());
        Some(shader)
    }
}

impl Default for DmaBufColorMaskApplier {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorMaskApplier for DmaBufColorMaskApplier {
    fn apply_color_mask(&mut self, texture: &gdk::Texture, mask_color: ColorMask, snapshot: &Snapshot, bounds: &Rect) -> Result<(), String> {
        debug!("DmaBufColorMaskApplier: applying color mask {mask_color} with bounds {}x{}", bounds.width(), bounds.height());

        let shader = self.get_color_mask_shader().ok_or("Failed to create color mask shader")?;
        let n_textures = shader.n_textures();
        debug!("Shader expects {} textures", n_textures);
        debug!("DmaBufColorMaskApplier: mask color {} with tolerance {}", mask_color.to_hex(), mask_color.tolerance);

        let mut data = vec![0u8; 16];
        data[0..4].copy_from_slice(&mask_color.tolerance.to_ne_bytes());
        data[4..8].copy_from_slice(&mask_color.color.red.to_ne_bytes());
        data[8..12].copy_from_slice(&mask_color.color.green.to_ne_bytes());
        data[12..16].copy_from_slice(&mask_color.color.blue.to_ne_bytes());
        let args = Bytes::from_owned(data);

        let texture_node = TextureNode::new(texture, bounds);
        let shader_node = GLShaderNode::new(&shader, bounds, &args, &[texture_node.upcast()]);
        snapshot.append_node(&shader_node);
        debug!("DmaBufColorMaskApplier: successfully applied color mask shader");

        Ok(())
    }
}
