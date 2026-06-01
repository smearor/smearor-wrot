use crate::opengl_renderer::OpenGLRenderer;
use crate::texture::extract_pixel_data_from_texture;
use crate::widget::imp::shm::texture::create_memory_texture_bgra;
use crate::widget::imp::shm::texture::create_memory_texture_from_pixel_data_bgra;
use glib::prelude::Cast;
use gtk4::gdk;
use gtk4::gdk::DmabufTextureBuilder;
use gtk4::gdk::Texture;
use gtk4::prelude::TextureExt;
use smearor_wrot_core::DmaBuffer;
use smearor_wrot_core::SmearorCompositor;
use smearor_wrot_core::buffer::metadata::BufferMetadata;
use smearor_wrot_core::color_mask::mask::ColorMask;
use smearor_wrot_core::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_core::commit::count::CommitCount;
use smearor_wrot_core::dma::count::DmaBufRenderCount;
use smearor_wrot_core::texture::cache::TextureCacheEntry;
use smearor_wrot_core::texture::pixel_data::BGRA;
use smearor_wrot_core::texture::pixel_data::PixelData;
use smearor_wrot_model::color::rgba::RgbaColor;
use smithay::backend::allocator::Buffer;
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::ImportMem;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::utils::Size;
use smithay::wayland::dmabuf::get_dmabuf;
use std::os::fd::AsRawFd;
use std::sync::atomic::Ordering;
use tracing::debug;
use tracing::error;

pub trait DmaBufRenderNode {
    fn dma_buf_render_window_to_texture(&self, compositor: &SmearorCompositor, surface_id: &ObjectId, buffer: &WlBuffer) -> Option<Texture>;

    /// Check if a buffer is a DMA-BUF buffer
    ///
    /// This function uses Smithay's get_dmabuf() to detect if a buffer
    /// is backed by DMA-BUF hardware acceleration.
    fn is_dmabuf_buffer(&self, buffer: &WlBuffer) -> bool;

    /// Import a DMA-BUF buffer directly to a GDK texture
    ///
    /// This function imports a DMA-BUF buffer from Wayland and converts it to a GDK texture
    /// using the DmabufTextureBuilder. This allows hardware-accelerated rendering without
    /// requiring an OpenGL renderer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The Wayland buffer to import
    ///
    /// # Returns
    ///
    /// * `Option<gdk::Texture>` - The imported texture or None if import fails
    fn import_dmabuf_texture(&self, buffer: &WlBuffer) -> Option<Texture>;

    /// Apply color mask shader to a texture using OpenGL renderer
    ///
    /// This function converts a GDK texture to GlesRenderbuffer, applies the color mask shader,
    /// and converts the result back to a GDK texture.
    ///
    /// # Arguments
    ///
    /// * `texture` - Input texture to apply color mask to
    /// * `renderer` - OpenGL renderer for shader execution
    /// * `mask_color` - Color mask to apply
    /// * `tolerance` - Color matching tolerance
    ///
    /// # Returns
    ///
    /// * `Option<Texture>` - Texture with color mask applied or None on failure
    fn apply_color_mask_shader_to_texture(&self, texture: &Texture, renderer: &mut OpenGLRenderer, mask_color: ColorMask, tolerance: f32) -> Option<Texture>;
}

impl DmaBufRenderNode for crate::widget::imp::CompositorWidgetImpl {
    fn dma_buf_render_window_to_texture(&self, compositor: &SmearorCompositor, surface_id: &ObjectId, buffer: &WlBuffer) -> Option<Texture> {
        // Check if buffer is DMA-BUF (hardware-accelerated)
        let Ok(_dma_buf) = get_dmabuf(buffer) else {
            debug!("Buffer is not DMA-BUF, using SHM fallback");
            return None;
        };
        debug!("Detected DMA-BUF buffer for surface: {:?}", surface_id);

        // Import DMA-BUF texture (even if DMA-BUF is disabled, we need to extract pixel data)
        debug!("Importing DMA-BUF texture for pixel data extraction");
        let Some(texture) = self.import_dmabuf_texture(buffer) else {
            debug!("Failed to import DMA-BUF texture, falling back to SHM");
            return None;
        };
        debug!("Successfully imported DMA-BUF texture for surface: {:?}", surface_id);

        // Track DMA-BUF render path
        compositor.increment_dma_buf_render_count();

        // Auto-detect color mask is now handled in Holding Area

        let current_commit_count = compositor.get_commit_count(surface_id.clone());

        // If DMA-BUF is disabled, extract pixel data and store in texture_cache
        if !compositor.is_dma_buf_available() {
            debug!("DMA-BUF is disabled, extracting pixel data for texture_cache");

            // Check if we have a cached masked texture for this surface at this commit count
            if current_commit_count > 0 {
                if let Some(cached_texture) = self.get_cached_masked_texture(surface_id, current_commit_count) {
                    debug!("Using cached masked texture for surface: {:?} at commit count {}", surface_id, current_commit_count);
                    return Some(cached_texture);
                }
            }

            // No cached texture exists, extract pixel data from DMA-BUF texture
            debug!("No cached texture found, extracting from DMA-BUF texture for surface: {:?}", surface_id);
            let pixel_data = extract_pixel_data_from_texture(&texture);

            let width = texture.width() as i32;
            let height = texture.height() as i32;
            let stride = width * 4; // BGRA has 4 bytes per pixel
            let buffer_metadata = BufferMetadata::new(width, height, stride);

            let pixel_data_bgra = PixelData::<BGRA>::from_slice(&pixel_data);
            let texture_cache_entry = TextureCacheEntry::new(buffer_metadata, pixel_data_bgra);

            // Store in texture_cache
            compositor.texture_cache.insert(surface_id.clone(), texture_cache_entry);
            debug!("Extracted and cached pixel data from DMA-BUF texture for surface: {:?}", surface_id);

            // Apply color mask if enabled using CPU-based processing
            if let Some(mask_color) = compositor.get_color_mask() {
                debug!("Applying color mask to cached pixel data for surface: {:?}", surface_id);
                if let Some(mut cache_entry) = compositor.texture_cache.get_mut(surface_id) {
                    let texture_cache_entry = cache_entry.value_mut();

                    // Check if color mask is already applied to avoid redundant processing
                    if !texture_cache_entry.color_mask_applied {
                        let replacement_color = RgbaColor::transparent();
                        texture_cache_entry.pixel_data.replace_color(mask_color, replacement_color);
                        texture_cache_entry.color_mask_applied = true;
                        debug!("Applied color mask to cached pixel data for surface: {:?}", surface_id);
                    } else {
                        debug!("Color mask already applied to cached pixel data for surface: {:?}, skipping", surface_id);
                    }
                }
            }

            // Create texture from cached pixel data
            if let Some(cache_entry) = compositor.texture_cache.get(surface_id) {
                let texture_cache_entry = cache_entry.value();
                let gdk_texture = create_memory_texture_bgra(texture_cache_entry);
                let texture = Texture::from(gdk_texture);
                debug!("Created texture from cached pixel data for surface: {:?}", surface_id);

                // Cache the masked texture for this surface at this commit count
                // This ensures that subsequent frames with the same commit_count can use the cached texture
                self.cache_masked_texture(surface_id.clone(), current_commit_count, texture.clone());
                if current_commit_count < 2 {
                    compositor.increment_commit_count(surface_id.clone());
                }

                return Some(texture);
            }
            return None;
        }

        if !compositor.color_mask_shader.load(Ordering::Relaxed) {
            // Color mask shader is disabled, apply color mask using CPU-based processing (OpenGL renderer)
            if let Some(mask_color) = compositor.get_color_mask() {
                debug!("Color mask shader is disabled, applying CPU-based color masking to DMA-BUF texture");

                // Check if we have a cached masked texture for this surface at this commit count
                if let Some(cached_texture) = self.get_cached_masked_texture(surface_id, current_commit_count) {
                    debug!("Using cached masked texture for surface: {:?} at commit count {}", surface_id, current_commit_count);
                    return Some(cached_texture);
                }

                // Get OpenGL renderer
                let mut opengl_renderer = self.opengl_renderer_mut();
                let Some(renderer) = opengl_renderer.as_mut() else {
                    debug!("OpenGL renderer not available, skipping color mask shader");
                    return Some(texture);
                };

                // Apply color mask shader using GPU-based processing
                let tolerance = mask_color.tolerance();
                // Clone the ColorMask for the shader function
                let mask_color_clone = mask_color;
                if let Some(masked_texture) = self.apply_color_mask_shader_to_texture(&texture, renderer, mask_color_clone, tolerance) {
                    debug!("Successfully applied color mask shader to DMA-BUF texture");
                    // Cache the masked texture for this surface at this commit count
                    self.cache_masked_texture(surface_id.clone(), current_commit_count, masked_texture.clone());
                    if current_commit_count < 2 {
                        compositor.increment_commit_count(surface_id.clone());
                    }
                    return Some(masked_texture);
                } else {
                    debug!("Failed to apply color mask shader, returning original texture");
                    return Some(texture);
                }
            }
        } else {
            // Color mask shader is enabled, shader will be applied in snapshot() for better performance
            // This avoids CPU-GPU roundtrips that were causing 80% CPU usage during video playback
            if let Some(_mask_color) = compositor.get_color_mask() {
                debug!("Color mask shader is enabled, shader application moved to snapshot() for performance");
                // Return original texture - shader will be applied in snapshot() rendering
                return Some(texture);
            }
        }

        Some(texture)
    }

    /// Check if a buffer is a DMA-BUF buffer
    ///
    /// This function uses Smithay's get_dmabuf() to detect if a buffer
    /// is backed by DMA-BUF hardware acceleration.
    fn is_dmabuf_buffer(&self, buffer: &WlBuffer) -> bool {
        match get_dmabuf(buffer) {
            Ok(_dmabuf) => {
                debug!("Detected DMA-BUF buffer: {:?}", buffer.id());
                true
            }
            Err(e) => {
                debug!("Not a DMA-BUF buffer: {}", e);
                false
            }
        }
    }

    fn import_dmabuf_texture(&self, buffer: &WlBuffer) -> Option<Texture> {
        let dmabuf = match get_dmabuf(buffer) {
            Ok(d) => d,
            Err(_) => {
                debug!("Buffer is not a DMA-BUF buffer");
                return None;
            }
        };

        debug!(
            "Importing DMA-BUF: {}x{}, format: {:?}, planes: {}",
            dmabuf.size().w,
            dmabuf.size().h,
            dmabuf.format(),
            dmabuf.num_planes()
        );

        let current_format = dmabuf.format();
        // Extract u32 value from Fourcc for DRM code comparison
        // DrmFourcc(AR24) and gdk::Fourcc::Argb8888 are technically identical
        // but incompatible in Rust's type system. Both use standardized DRM values.
        // Note: We no longer pre-filter formats here. Let GDK's DmabufTextureBuilder
        // handle format validation. This allows GPU-specific modifiers (tiling, compression)
        // that are valid but not in our hardcoded whitelist.
        debug!("Attempting to import DMA-BUF with format: {:?}, modifier: {:?}", current_format.code, current_format.modifier);

        let display = match gdk::Display::default() {
            Some(d) => d,
            None => {
                debug!("No default GDK display found");
                return None;
            }
        };
        let num_planes = dmabuf.num_planes();
        let mut builder = DmabufTextureBuilder::new()
            .set_display(&display)
            .set_width(dmabuf.size().w as u32)
            .set_height(dmabuf.size().h as u32)
            .set_fourcc(dmabuf.format().code as u32)
            .set_modifier(u64::from(dmabuf.format().modifier))
            .set_n_planes(num_planes as u32);

        for i in 0..num_planes {
            let fd = match dmabuf.handles().nth(i) {
                Some(f) => f.as_raw_fd(),
                None => {
                    debug!("No FD found for plane {}", i);
                    return None;
                }
            };
            let stride = match dmabuf.strides().nth(i) {
                Some(s) => s as i32,
                None => {
                    debug!("No stride found for plane {}", i);
                    return None;
                }
            };
            let offset = match dmabuf.offsets().nth(i) {
                Some(o) => o as i32,
                None => {
                    debug!("No offset found for plane {}", i);
                    return None;
                }
            };

            builder = unsafe { builder.set_fd(i as u32, fd) };
            builder = builder.set_stride(i as u32, stride as u32).set_offset(i as u32, offset as u32);
        }

        // 4. FD-Management via Destroy Notify
        // Since Smithay tracks the buffer, we can hold a reference.
        // We use the DashMap registry in CompositorWidget to keep the Dmabuf
        // objects alive as long as the textures exist.
        let dmabuf_capture = dmabuf.clone();

        let texture = unsafe { builder.build() };
        match texture {
            Ok(texture) => {
                debug!("Successfully built GDK DMA-BUF texture");
                self.register_dmabuf_texture(texture.clone(), dmabuf_capture);
                debug!("Registered DMA-BUF texture in registry");
                Some(texture)
            }
            Err(err) => {
                error!("GDK DmabufTextureBuilder failed: {:?}", err);
                None
            }
        }
    }

    fn apply_color_mask_shader_to_texture(&self, texture: &Texture, renderer: &mut OpenGLRenderer, mask_color: ColorMask, tolerance: f32) -> Option<Texture> {
        debug!("Applying color mask shader to texture: color={:?}, tolerance={:.2}", mask_color, tolerance);

        // Extract pixel data from GDK texture
        let pixel_data = extract_pixel_data_from_texture(texture);
        let width = texture.width();
        let height = texture.height();

        debug!("Extracted pixel data: {}x{}, {} bytes", width, height, pixel_data.len());

        // Convert pixel data to GlesTexture using OpenGLRenderer
        let gles_renderer = renderer.renderer_mut()?;
        let format = Fourcc::Argb8888;
        let size = Size::from((width as i32, height as i32));

        // Create GlesTexture from pixel data
        let gles_texture = gles_renderer.import_memory(pixel_data.as_slice(), format, size, false).ok()?;

        debug!("Created GlesTexture from pixel data");

        // Apply color mask shader
        let mask_color_rgb = mask_color.color();
        let mask_rgb = (mask_color_rgb.red, mask_color_rgb.green, mask_color_rgb.blue);
        let masked_renderbuffer = renderer.apply_color_mask_shader(gles_texture, mask_rgb, tolerance).ok()?;

        debug!("Applied color mask shader successfully");

        // Convert result back to pixel data using the new method
        let masked_pixel_data = renderer.read_renderbuffer_to_pixel_data(&masked_renderbuffer).ok()?;

        debug!("Read {} bytes from masked renderbuffer", masked_pixel_data.len());

        // Create BufferMetadata from texture dimensions
        let stride = width * 4; // BGRA = 4 bytes per pixel
        let buffer_metadata = smearor_wrot_core::buffer::metadata::BufferMetadata::new(width as i32, height as i32, stride as i32);

        // Convert pixel data back to GDK texture
        let masked_pixel_data_bgra = PixelData::<BGRA>::from_slice(&masked_pixel_data);
        let memory_texture = create_memory_texture_from_pixel_data_bgra(&buffer_metadata, &masked_pixel_data_bgra);
        let gdk_texture = memory_texture.upcast::<Texture>();

        debug!("Successfully created GDK texture from masked pixel data");
        Some(gdk_texture)
    }
}
