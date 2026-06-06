use crate::opengl_renderer::OpenGLRenderer;
use crate::widget::imp::shm::texture::create_memory_texture_bgra;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::prelude::Cast;
use gtk4::gdk::Texture;
use smearor_wrot_core::SmearorCompositor;
use smearor_wrot_core::background::toplevel::ToplevelBackground;
use smearor_wrot_core::buffer::metadata::BufferMetadata;
use smearor_wrot_core::color_mask::mask::ColorMask;
use smearor_wrot_core::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_core::commit::count::CommitCount;
use smearor_wrot_core::render::count::ShmRenderCount;
use smearor_wrot_core::texture::cache::TextureCacheEntry;
use smearor_wrot_core::texture::pixel_data::BGRA;
use smearor_wrot_core::texture::pixel_data::PixelData;
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::ImportMem;
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::utils::Size;
use smithay::wayland::shm::with_buffer_contents;
use tracing::debug;

pub trait ShmRenderNode {
    fn shm_render_window_to_texture(&self, compositor: &SmearorCompositor, surface_id: &ObjectId, buffer: &WlBuffer) -> Option<Texture>;

    /// Apply color mask shader to pixel data using OpenGL renderer
    ///
    /// This function converts pixel data to GlesRenderbuffer, applies the color mask shader,
    /// and converts the result back to pixel data.
    ///
    /// # Arguments
    ///
    /// * `pixel_data` - Input pixel data to apply color mask to
    /// * `width` - Width of the pixel data
    /// * `height` - Height of the pixel data
    /// * `renderer` - OpenGL renderer for shader execution
    /// * `mask_color` - Color mask to apply
    /// * `tolerance` - Color matching tolerance
    ///
    /// # Returns
    ///
    /// * `Option<PixelData<BGRA>>` - Pixel data with color mask applied or None on failure
    fn apply_color_mask_shader_to_pixel_data(
        &self,
        pixel_data: &[u8],
        width: u32,
        height: u32,
        renderer: &mut OpenGLRenderer,
        mask_color: ColorMask,
        tolerance: f32,
    ) -> Option<PixelData<BGRA>>;
}

impl ShmRenderNode for CompositorWidgetImpl {
    fn shm_render_window_to_texture(&self, compositor: &SmearorCompositor, surface_id: &ObjectId, buffer: &WlBuffer) -> Option<Texture> {
        debug!("shm_render_window_to_texture called for surface: {:?}", surface_id);

        // Check if buffer is DMA-BUF - if so, SHM rendering is not applicable
        use smithay::wayland::dmabuf::get_dmabuf;
        if let Ok(_dmabuf) = get_dmabuf(buffer) {
            debug!("Buffer is DMA-BUF, SHM rendering not applicable for surface: {:?}", surface_id);
            return None;
        }

        debug!("Buffer is not DMA-BUF, attempting SHM rendering for surface: {:?}", surface_id);

        // Fallback to SHM (software rendering) - always execute if DMA-BUF import failed or was disabled
        if let Ok(Some(texture)) = with_buffer_contents(&buffer, |memory_pointer, data_length, buffer_metadata| {
            let buffer_metadata = BufferMetadata::from(&buffer_metadata);
            debug!("Buffer contents accessible from holding area (SHM): {}", buffer_metadata);
            if data_length == 0 {
                debug!("Buffer data length is zero, skipping texture creation");
                return None;
            }
            let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };
            debug!("Extracting {} bytes of SHM buffer data from holding area", data_length);
            let pixel_data = PixelData::<BGRA>::from_slice(pixel_data_slice);

            // Get current commit count for this surface
            let current_commit_count = compositor.get_commit_count(surface_id.clone());

            // Track SHM render path
            compositor.increment_shm_render_count();

            // Check if we have a cached texture entry for this surface
            if let Some(mut texture_cache_entry) = compositor.texture_cache.get_mut(surface_id) {
                debug!(
                    "Found cached texture entry for surface: {:?}, commit_count={}, color_mask_applied={}",
                    surface_id, texture_cache_entry.commit_count, texture_cache_entry.color_mask_applied
                );

                // Check if commit count has changed - if so, replace the cache entry
                if texture_cache_entry.commit_count != current_commit_count {
                    debug!(
                        "Commit count changed from {} to {}, replacing cache entry",
                        texture_cache_entry.commit_count, current_commit_count
                    );
                    let new_entry = TextureCacheEntry::with_commit_count(buffer_metadata, pixel_data, current_commit_count);
                    *texture_cache_entry = new_entry;
                }
            } else {
                // No cache entry exists, create a new one
                debug!(
                    "No cached texture entry found, creating new one for surface: {:?}, commit_count={}",
                    surface_id, current_commit_count
                );
                let texture_cache_entry = TextureCacheEntry::with_commit_count(buffer_metadata, pixel_data, current_commit_count);
                compositor.texture_cache.insert(surface_id.clone(), texture_cache_entry);
            }

            let Some(mut texture_cache_entry) = compositor.texture_cache.get_mut(surface_id) else {
                return None;
            };
            debug!("Buffer data cached successfully from holding area for surface: {:?}", surface_id);

            // Auto-detect color mask is now handled in Holding Area

            if let Some(mask_color) = compositor.get_color_mask() {
                // Check if color mask has already been applied at this commit count
                if !texture_cache_entry.color_mask_applied || texture_cache_entry.commit_count != current_commit_count {
                    debug!("Applying color mask to holding area buffer data for surface: {:?}", surface_id);

                    // Check if OpenGL renderer is available for shader-based color masking
                    let mut opengl_renderer = self.opengl_renderer_mut();
                    if let Some(renderer) = opengl_renderer.as_mut() {
                        debug!("OpenGL renderer available, using shader-based color masking for SHM pipeline");

                        // Apply color mask shader using GPU-based processing directly on pixel data
                        let pixel_data_slice = texture_cache_entry.pixel_data.as_slice();
                        let width = texture_cache_entry.buffer_metadata.width as u32;
                        let height = texture_cache_entry.buffer_metadata.height as u32;
                        let tolerance = mask_color.tolerance();
                        let mask_color_clone = mask_color;

                        if let Some(masked_pixel_data) =
                            self.apply_color_mask_shader_to_pixel_data(pixel_data_slice, width, height, renderer, mask_color_clone, tolerance)
                        {
                            debug!("Successfully applied color mask shader to SHM pixel data");

                            // Update texture cache entry with masked pixel data
                            texture_cache_entry.pixel_data = masked_pixel_data;
                            texture_cache_entry.color_mask_applied = true;
                            texture_cache_entry.commit_count = current_commit_count;
                            let gdk_texture = create_memory_texture_bgra(&texture_cache_entry);
                            return Some(gdk_texture.upcast_ref::<Texture>().clone());
                        } else {
                            debug!("Failed to apply color mask shader, falling back to CPU-based color masking");
                        }
                    }

                    // Fallback to CPU-based color masking
                    debug!("Using CPU-based color masking as fallback for SHM pipeline");
                    if let Some(background_color) = compositor.get_background_color() {
                        // Replace mask color with background color
                        texture_cache_entry.pixel_data.replace_color(mask_color, background_color);
                    } else {
                        // Apply chroma-keying (make transparent)
                        texture_cache_entry.pixel_data.apply_color_mask(mask_color);
                    }
                    texture_cache_entry.color_mask_applied = true;
                    texture_cache_entry.commit_count = current_commit_count;
                } else {
                    debug!("Color mask already applied at commit count {} for surface: {:?}", current_commit_count, surface_id);
                }
            }

            let gdk_texture = create_memory_texture_bgra(&texture_cache_entry);
            debug!(
                "Created GDK texture from holding area SHM buffer: {} (width={}, height={}, stride={}, pixel_data_len={})",
                texture_cache_entry.buffer_metadata,
                texture_cache_entry.buffer_metadata.width,
                texture_cache_entry.buffer_metadata.height,
                texture_cache_entry.buffer_metadata.stride,
                texture_cache_entry.pixel_data.as_slice().len()
            );
            Some(gdk_texture.upcast_ref::<Texture>().clone())
        }) {
            return Some(texture);
        }
        None
    }

    fn apply_color_mask_shader_to_pixel_data(
        &self,
        pixel_data: &[u8],
        width: u32,
        height: u32,
        renderer: &mut OpenGLRenderer,
        mask_color: ColorMask,
        tolerance: f32,
    ) -> Option<PixelData<BGRA>> {
        debug!("Applying color mask shader to pixel data: {}x{}, tolerance={:.2}", width, height, tolerance);

        // Convert pixel data to GlesRenderbuffer using OpenGLRenderer
        let gles_renderer = renderer.renderer_mut()?;
        let format = Fourcc::Argb8888;
        let size = Size::from((width as i32, height as i32));

        // Create GlesRenderbuffer from pixel data
        let renderbuffer = gles_renderer.import_memory(pixel_data, format, size, false).ok()?;

        debug!("Created GlesRenderbuffer from pixel data");

        // Apply color mask shader
        let mask_color_rgb = mask_color.color();
        let mask_rgb = (mask_color_rgb.red, mask_color_rgb.green, mask_color_rgb.blue);
        let masked_renderbuffer = renderer.apply_color_mask_shader(renderbuffer, mask_rgb, tolerance).ok()?;

        debug!("Applied color mask shader successfully");

        // Convert result back to pixel data using the new method
        let masked_pixel_data = renderer.read_renderbuffer_to_pixel_data(&masked_renderbuffer).ok()?;

        debug!("Read {} bytes from masked renderbuffer", masked_pixel_data.len());

        // Convert to PixelData<BGRA>
        let pixel_data_bgra = PixelData::<BGRA>::from_slice(&masked_pixel_data);

        debug!("Successfully created PixelData from masked pixel data");
        Some(pixel_data_bgra)
    }
}
