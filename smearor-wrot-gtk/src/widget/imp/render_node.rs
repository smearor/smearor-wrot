//! Rendering logic for Wayland buffers to GTK textures

use crate::opengl_renderer::OpenGLRenderer;
use crate::widget::imp::dmabuf::render_node::DmaBufRenderNode;
use crate::widget::imp::holding_area::BufferHoldingArea;
use crate::widget::imp::shm::texture::create_memory_texture_bgra;
use crate::widget::imp::shm::texture::create_memory_texture_from_pixel_data_bgra;
use gtk4::gdk;
use gtk4::glib::Bytes;
use gtk4::prelude::Cast;
use smearor_wrot_core::SmearorCompositor;
use smearor_wrot_core::background::subsurface::SubsurfaceBackground;
use smearor_wrot_core::background::toplevel::ToplevelBackground;
use smearor_wrot_core::buffer::metadata::BufferMetadata;
use smearor_wrot_core::color_mask::subsurface::SubSurfaceColorMask;
use smearor_wrot_core::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_core::dma::buffer::DmaBuffer;
use smearor_wrot_core::texture::cache::TextureCacheEntry;
use smearor_wrot_core::texture::pixel_data::BGRA;
use smearor_wrot_core::texture::pixel_data::PixelData;
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::Bind;
use smithay::backend::renderer::Frame;
use smithay::backend::renderer::ImportMem;
use smithay::backend::renderer::Renderer;
use smithay::desktop::Window;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::utils::Physical;
use smithay::utils::Point;
use smithay::utils::Rectangle;
use smithay::utils::Size;
use smithay::utils::Transform;
use smithay::wayland::compositor::BufferAssignment;
use smithay::wayland::compositor::SurfaceAttributes;
use smithay::wayland::compositor::SurfaceData;
use smithay::wayland::compositor::with_states;
use smithay::wayland::dmabuf::get_dmabuf;
use smithay::wayland::shm::with_buffer_contents;
use tracing::debug;
use tracing::info;

impl crate::widget::imp::CompositorWidgetImpl {
    /// Render a Smithay window's current buffer into an offscreen FBO using OpenGL renderer
    ///
    /// This function uses hardware-accelerated OpenGL rendering to render Wayland buffers
    /// to an offscreen framebuffer for DMA-BUF export.
    ///
    /// # Arguments
    ///
    /// * `target_window` - The Smithay window to render
    /// * `renderer` - The OpenGL renderer to use for rendering
    /// * `target_width` - The desired width of the FBO
    /// * `target_height` - The desired height of the FBO
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If rendering succeeded
    /// * `Err(String)` - If rendering failed
    #[allow(dead_code)]
    pub fn render_window_to_offscreen_fbo(
        &self,
        target_window: &Window,
        renderer: &mut OpenGLRenderer,
        target_width: i32,
        target_height: i32,
    ) -> Result<(), String> {
        // TODO: Phase 5 - Implement actual offscreen FBO rendering
        // This requires:
        // 1. Get Wayland surface buffer from window
        // 2. Bind buffer to GlesRenderer using Bind trait
        // 3. Render to offscreen renderbuffer using Renderer trait
        // 4. Handle DMA-BUF and SHM buffers differently
        // Note: This requires investigation of Smithay's rendering pipeline

        debug!("Rendering window to offscreen FBO: {}x{}", target_width, target_height);

        // Try to get the Wayland surface
        let wayland_surface = target_window.toplevel().map(|t| t.wl_surface()).ok_or("Window has no Wayland surface")?;

        debug!("Wayland surface found: {:?}", wayland_surface.id());

        // TODO: Phase 5 - Get buffer and render to FBO
        // This requires:
        // 1. Get buffer from surface using with_states
        // 2. Import buffer as texture using ImportMem::import_memory
        // 3. Create offscreen renderbuffer using OpenGLRenderer::render_to_offscreen
        // 4. Bind renderbuffer to renderer using Bind::bind
        // 5. Render buffer content to renderbuffer using Renderer::render and Frame::render_texture_from_to
        // 6. Handle errors gracefully

        // Get the renderer's GlesRenderer
        let gles_renderer = renderer.renderer_mut().ok_or("OpenGL renderer not initialized")?;

        // Get buffer from surface
        let buffer_handle = with_states(&wayland_surface, |surface_data| {
            let buffer_assignment = self.get_buffer_assignment(surface_data);
            // let buffer_assignment = surface_data
            //     .cached_state
            //     .get::<SurfaceAttributes>()
            //     .current()
            //     .buffer
            //     .as_ref()
            //     .map(|assignment_type| match assignment_type {
            //         BufferAssignment::NewBuffer(buffer) => BufferAssignment::NewBuffer(buffer.clone()),
            //         BufferAssignment::Removed => BufferAssignment::Removed,
            //     })
            //     .unwrap_or(BufferAssignment::Removed);

            buffer_assignment
        });

        let buffer_handle = match buffer_handle {
            BufferAssignment::NewBuffer(buffer) => buffer,
            BufferAssignment::Removed => return Err("No buffer attached to surface".to_string()),
        };

        // TODO: Phase 5 - Import buffer as texture using ImportMem::import_memory
        // Read buffer contents and import as texture
        let texture_id = with_buffer_contents(&buffer_handle, |memory_pointer, data_length, buffer_metadata| {
            debug!(
                "Importing SHM buffer: {}x{} with stride {}",
                buffer_metadata.width, buffer_metadata.height, buffer_metadata.stride
            );

            if data_length == 0 {
                return Err("Buffer data length is zero".to_string());
            }

            let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };
            let size = Size::from((buffer_metadata.width, buffer_metadata.height));

            // Use ARGB8888 format for SHM buffers
            let format = Fourcc::Argb8888;

            // Import buffer as texture using ImportMem::import_memory
            gles_renderer
                .import_memory(pixel_data_slice, format, size, false)
                .map_err(|e| format!("Failed to import buffer as texture: {:?}", e))
        })
        .map_err(|e| format!("Failed to read buffer contents: {:?}", e))??;

        debug!("Buffer imported as texture successfully");

        // TODO: Phase 5 - Create offscreen renderbuffer and render texture to it
        // This requires:
        // 1. Create offscreen renderbuffer using OpenGLRenderer::render_to_offscreen
        // 2. Bind renderbuffer to renderer using Bind::bind
        // 3. Render texture to renderbuffer using Renderer::render and Frame::render_texture_from_to

        let offscreen_target_width = target_width.try_into().map_err(|e| format!("Failed to convert width: {:?}", e))?;
        let offscreen_target_height = target_height.try_into().map_err(|e| format!("Failed to convert height: {:?}", e))?;
        let mut renderbuffer = renderer
            .render_to_offscreen(offscreen_target_width, offscreen_target_height, Fourcc::Argb8888)
            .map_err(|e| format!("Failed to create offscreen renderbuffer: {:?}", e))?;

        debug!("Created offscreen renderbuffer: {}x{}", target_width, target_height);

        // Phase 5 - Bind renderbuffer and render texture
        // Get mutable reference to GlesRenderer for Bind trait
        let gles_renderer = renderer.renderer_mut().ok_or("OpenGL renderer not initialized")?;

        // Bind renderbuffer to renderer using Bind trait
        // Note: GlesRenderer implements Bind<GlesRenderbuffer>
        let mut framebuffer = Bind::bind(gles_renderer, &mut renderbuffer).map_err(|e| format!("Failed to bind renderbuffer: {:?}", e))?;

        debug!("Renderbuffer bound successfully");

        // Phase 5 - Render texture to framebuffer
        // Use Renderer::render to get a Frame
        let output_size = Size::from((target_width, target_height));
        let transform = Transform::Normal;

        let mut frame =
            Renderer::render(gles_renderer, &mut framebuffer, output_size, transform).map_err(|e| format!("Failed to create rendering frame: {:?}", e))?;

        debug!("Rendering frame created successfully");

        // Phase 5 - Render texture to framebuffer using Frame
        // Calculate source and destination regions
        let texture_size = Size::from((target_width, target_height));

        // Use Rectangle<f64, Buffer> for source region as required by render_texture_from_to
        let src = Rectangle::new(Point::from((0.0, 0.0)), Size::from((target_width as f64, target_height as f64)));

        // Use Rectangle<i32, Physical> for destination region
        let dst = Rectangle::new(Point::from((0, 0)), texture_size);

        // Empty damage arrays (no damage to handle)
        let damage: &[Rectangle<i32, Physical>] = &[];

        // Render texture to framebuffer with correct parameters
        Frame::render_texture_from_to(
            &mut frame,
            &texture_id,
            src,
            dst,
            damage,
            damage,
            Transform::Normal,
            1.0, // alpha
        )
        .map_err(|e| format!("Failed to render texture to framebuffer: {:?}", e))?;

        debug!("Texture rendered to framebuffer successfully");

        // Complete the rendering frame
        let _sync_point = Frame::finish(frame).map_err(|e| format!("Failed to finish rendering frame: {:?}", e))?;

        debug!("Rendering frame completed successfully");

        // TODO: Phase 5 - Return the rendered framebuffer for export
        // The renderbuffer now contains the rendered texture
        // This can be exported as DMA-BUF for GTK4 integration
        Ok(())
    }

    /// Render a Smithay window's current buffer into a GDK texture
    ///
    /// This function converts Wayland SHM buffers into GDK memory textures
    /// for display in GTK4 widgets. It uses the texture_cache to avoid timing issues
    /// where buffers are marked as "Removed" before rendering.
    ///
    /// TODO: Phase 7 - DMA-BUF support for hardware acceleration - Add OpenGL renderer parameter
    /// This requires:
    /// 1. Accept OpenGL renderer parameter for DMA-BUF import
    /// 2. Call import_dmabuf_texture() when DMA-BUF buffer is detected
    /// 3. Fallback to SHM rendering if DMA-BUF import fails
    ///
    /// # Arguments
    ///
    /// * `target_window` - The Smithay window to render
    /// * `compositor` - The compositor instance to access texture_cache
    /// * `target_width` - The desired width of the resulting texture
    /// * `target_height` - The desired height of the resulting texture
    /// * `renderer` - The OpenGL renderer for DMA-BUF import
    ///
    /// # Returns
    ///
    /// * `Some(gdk::Texture)` - The rendered texture if successful
    /// * `None` - If no buffer is attached or rendering fails
    pub fn render_window_to_texture(
        &self,
        target_window: &Window,
        compositor: &SmearorCompositor,
        target_width: i32,
        target_height: i32,
    ) -> Option<gdk::Texture> {
        debug!("render_window_to_texture called for window");
        // Try to get the Wayland surface and render the actual buffer
        if let Some(wayland_surface) = target_window.toplevel().map(|t| t.wl_surface()) {
            debug!("Attempting to render Wayland surface: {:?}", wayland_surface.id());

            let surface_id = wayland_surface.id();

            // Render buffer from Buffer-Holding-Area
            if let Some(texture) = self.render_buffer_from_holding_area(compositor, &surface_id) {
                return Some(texture);
            }

            // Check if buffer data is cached from commit time (fallback)
            if let Some(mut cache_entry) = compositor.texture_cache.get_mut(&surface_id) {
                debug!("Using cached buffer data for surface: {:?}", surface_id);
                let texture_cache_entry = cache_entry.value_mut();
                debug!("Cached data dimensions: {texture_cache_entry}");

                // Auto-detect color mask is now handled in Holding Area

                // Apply color mask if set and not already applied
                if let Some(mask_color) = compositor.get_color_mask() {
                    if !texture_cache_entry.color_mask_applied {
                        info!("Applying color mask to cached buffer data for surface: {:?} {mask_color}", surface_id);
                        if let Some(background_color) = compositor.get_background_color() {
                            texture_cache_entry.replace_color(mask_color, background_color);
                        } else {
                            // Apply chroma-keying (make transparent)
                            texture_cache_entry.apply_color_mask(mask_color);
                        }
                    } else {
                        debug!("Color mask already applied to cached buffer data for surface: {:?}", surface_id);
                    }
                } else {
                    info!("No color mask set, skipping color mask application");
                }

                let gdk_texture = create_memory_texture_bgra(&texture_cache_entry);

                debug!("Created GDK texture from cached buffer data: {}", texture_cache_entry.buffer_metadata);
                return Some(gdk_texture.upcast_ref::<gdk::Texture>().clone());
            }

            debug!("No cached buffer data found, falling back to direct buffer read");

            // Smithay's on_commit_buffer_handler manages buffer state internally
            // We directly access the buffer from the current state
            if let Some(texture) = with_states(&wayland_surface, |surface_data| {
                debug!("Successfully accessed surface data for rendering");
                let buffer_assignment = self.get_buffer_assignment(surface_data);

                debug!("Buffer assignment state: {:?}", buffer_assignment);

                // Try to get buffer from current state first, then pending state
                let buffer_handle = if let BufferAssignment::NewBuffer(buffer) = buffer_assignment {
                    Some(buffer)
                } else {
                    // Try to get buffer from pending state
                    surface_data
                        .cached_state
                        .get::<SurfaceAttributes>()
                        .pending()
                        .buffer
                        .as_ref()
                        .and_then(|assignment| match assignment {
                            BufferAssignment::NewBuffer(buffer) => Some(buffer.clone()),
                            BufferAssignment::Removed => None,
                        })
                };

                if let Some(buffer_handle) = buffer_handle {
                    let buffer_ptr = buffer_handle.id().as_ptr() as u64;
                    debug!("New buffer assignment found with pointer address: {}", buffer_ptr);

                    // Check if buffer is DMA-BUF
                    if self.is_dmabuf_buffer(&buffer_handle) {
                        debug!("Buffer is DMA-BUF - hardware acceleration available");

                        // Check if DMA-BUF is enabled
                        if compositor.is_dma_buf_available() {
                            debug!("DMA-BUF is enabled, importing DMA-BUF texture");

                            // Try to import DMA-BUF buffer using DmabufTextureBuilder
                            if let Some(dmabuf_texture) = self.import_dmabuf_texture(&buffer_handle) {
                                info!("Successfully imported DMA-BUF texture");
                                debug!(
                                    "DMA-BUF: Checking auto-detection: auto_color_mask={}, color_mask_detected={}",
                                    compositor.get_auto_color_mask(),
                                    compositor.is_color_mask_detected()
                                );

                                // Auto-detect color mask is now handled in Holding Area

                                return Some(dmabuf_texture);
                            }
                        } else {
                            debug!("DMA-BUF is disabled via --disable-dma-buf flag, skipping DMA-BUF import");
                        }
                        debug!("DMA-BUF import failed - falling back to SHM");
                    }

                    debug!("Starting read operation for buffer contents");

                    if let Some(buffer_data) = buffer_handle.data::<smithay::wayland::shm::ShmBufferUserData>() {
                        debug!("Buffer is SHM with metadata: {:?}", buffer_data);
                    } else {
                        debug!("Buffer type is not SHM or data is inaccessible");
                        debug!("Buffer is not SHM - likely DMA-BUF. This requires DMA-BUF support for rendering.");
                    }

                    debug!("Attempting to read buffer contents via with_buffer_contents");
                    if let Ok(texture) = with_buffer_contents(&buffer_handle, |memory_pointer, data_length, buffer_metadata| {
                        let buffer_metadata = BufferMetadata::from(&buffer_metadata);
                        debug!("Rendering SHM buffer: {buffer_metadata}");

                        if data_length == 0 {
                            debug!("Buffer data length is zero, skipping render");
                            return None;
                        }

                        // let source_width = buffer_metadata.width;
                        // let source_height = buffer_metadata.height;
                        // let source_stride = buffer_metadata.stride;

                        let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };
                        let mut pixel_data = PixelData::<BGRA>::from_slice(pixel_data_slice);

                        // Auto-detect color mask is now handled in Holding Area

                        // Check if buffer data is empty (all zeros)
                        if pixel_data.is_zero() {
                            debug!("Buffer data is all zeros - buffer is empty");
                        } else {
                            debug!("Buffer data contains non-zero values - buffer has content");
                        }

                        // Apply color mask if set
                        if let Some(mask_color) = compositor.get_color_mask() {
                            info!("Applying color mask to direct buffer read for surface: {:?} {mask_color}", surface_id);
                            // let mut data = pixel_data_vec;
                            if let Some(background_color) = compositor.get_background_color() {
                                // Replace mask color with background color
                                pixel_data.replace_color(mask_color, background_color);
                            } else {
                                // Apply chroma-keying (make transparent)
                                pixel_data.apply_color_mask(mask_color);
                            }
                        }

                        let gdk_texture = create_memory_texture_from_pixel_data_bgra(&buffer_metadata, &pixel_data);
                        debug!("Created GDK texture from Wayland buffer: {buffer_metadata}");
                        Some(gdk_texture.upcast_ref::<gdk::Texture>().clone())
                    }) {
                        return texture;
                    } else {
                        debug!("Could not read buffer contents via Smithay");
                    }
                } else {
                    debug!("No buffer on surface, generating test pattern with size {}x{}", target_width, target_height);
                    return self.create_test_texture(target_width, target_height);
                }
                None
            }) {
                return Some(texture);
            } else {
                debug!("Failed to access Wayland surface states");
            }
        } else {
            debug!("Window has no associated Wayland surface");
        }

        debug!("Using final fallback test texture at {}x{}", target_width, target_height);
        self.create_test_texture(target_width, target_height)
    }

    /// TODO: Phase 5 - Subsurface Rendering - Render subsurface to texture
    /// Renders a subsurface (like GTK4 native popups) to a GDK texture
    pub fn render_subsurface_to_texture(
        &self,
        subsurface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
        compositor: &SmearorCompositor,
    ) -> Option<gdk::Texture> {
        debug!("render_subsurface_to_texture called for subsurface");
        debug!("Attempting to render subsurface surface: {:?}", subsurface.id());

        let surface_id = subsurface.id();

        // Check if subsurface buffer is in Buffer-Holding-Area (DMA-BUF support)
        if let Ok(holding_area) = compositor.buffer_holding_area.lock() {
            if let Some(buffer) = holding_area.get(&surface_id) {
                debug!("Using subsurface buffer from Buffer-Holding-Area for surface: {:?}", surface_id);

                // Check if subsurface buffer is DMA-BUF (hardware-accelerated)
                if let Ok(_) = get_dmabuf(buffer) {
                    debug!("Detected DMA-BUF buffer for subsurface surface: {:?}", surface_id);

                    // Check if DMA-BUF is enabled
                    if compositor.is_dma_buf_available() {
                        debug!("DMA-BUF is enabled, importing DMA-BUF texture for subsurface");

                        // Import DMA-BUF buffer directly to GDK texture
                        if let Some(texture) = self.import_dmabuf_texture(buffer) {
                            debug!("Successfully imported DMA-BUF texture for subsurface surface: {:?}", surface_id);
                            return Some(texture);
                        } else {
                            debug!("Failed to import DMA-BUF texture for subsurface, falling back to SHM");
                        }
                    } else {
                        debug!("DMA-BUF is disabled via --disable-dma-buf flag, falling back to SHM for subsurface");
                    }
                } else {
                    debug!("Subsurface buffer is not DMA-BUF, using SHM fallback");
                }
            }
        }

        // Check if buffer data is cached from commit time
        if let Some(mut texture_cache_entry) = compositor.texture_cache.get_mut(&surface_id) {
            debug!("Using cached buffer data for subsurface surface: {:?}", surface_id);
            let texture_cache_entry = texture_cache_entry.value_mut();
            debug!("Cached subsurface data dimensions: {texture_cache_entry}");

            // Apply color mask if set and not already applied
            if let Some(subsurface_mask_color) = compositor.get_subsurface_color_mask() {
                if !texture_cache_entry.color_mask_applied {
                    debug!("Applying subsurface color mask to cached subsurface buffer data for surface: {:?}", subsurface.id());
                    let background_color = compositor.get_subsurface_background_color().or_else(|| compositor.get_background_color());
                    if let Some(bg_color) = background_color {
                        // Replace mask color with background color (subsurface-specific if available)
                        texture_cache_entry.replace_color(subsurface_mask_color, bg_color);
                    } else {
                        // Apply chroma-keying (make transparent)
                        texture_cache_entry.apply_color_mask(subsurface_mask_color);
                    }
                } else {
                    debug!("Color mask already applied to cached subsurface buffer data for surface: {:?}", subsurface.id());
                }
            } else if let Some(mask_color) = compositor.get_color_mask() {
                if !texture_cache_entry.color_mask_applied {
                    debug!("Applying color mask to cached subsurface buffer data for surface: {:?}", subsurface.id());
                    let background_color = compositor.get_subsurface_background_color().or_else(|| compositor.get_background_color());
                    if let Some(bg_color) = background_color {
                        // Replace mask color with background color (subsurface-specific if available)
                        texture_cache_entry.replace_color(mask_color, bg_color);
                    } else {
                        // Apply chroma-keying (make transparent)
                        texture_cache_entry.apply_color_mask(mask_color);
                    }
                } else {
                    debug!("Color mask already applied to cached subsurface buffer data for surface: {:?}", subsurface.id());
                }
            }

            // Auto-detect color mask is now handled in Holding Area
            // Auto-detect subsurface color mask is now handled in Holding Area

            let gdk_texture = create_memory_texture_bgra(&texture_cache_entry);

            // let pixel_bytes = Bytes::from(&data[..]);
            // let gdk_memory_format = gdk::MemoryFormat::B8g8r8a8;
            // let gdk_texture = gdk::MemoryTexture::new(width, height, gdk_memory_format, &pixel_bytes, stride as usize);

            debug!("Created GDK texture from cached subsurface buffer data: {}", texture_cache_entry.buffer_metadata);
            return Some(gdk_texture.upcast_ref::<gdk::Texture>().clone());
        }

        debug!("No cached buffer data found for subsurface, falling back to direct buffer read");

        with_states(subsurface, |surface_data| {
            debug!("Successfully accessed subsurface surface data for rendering");
            let Some(buffer) = self.get_new_buffer_assignment(&surface_data) else {
                return None;
            };

            debug!("Attempting to read subsurface buffer data directly");
            let Ok(Some(texture)) = with_buffer_contents(&buffer, |memory_pointer, data_length, buffer_metadata| {
                debug!(
                    "Rendering subsurface SHM buffer: {}x{} with stride {}",
                    buffer_metadata.width, buffer_metadata.height, buffer_metadata.stride
                );

                if data_length == 0 {
                    debug!("Subsurface buffer data length is zero, skipping render");
                    return None;
                }

                let source_width = buffer_metadata.width;
                let source_height = buffer_metadata.height;
                let source_stride = buffer_metadata.stride;

                let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };

                let pixel_bytes = Bytes::from_owned(pixel_data_slice.to_vec());
                let gdk_memory_format = gdk::MemoryFormat::B8g8r8a8;

                let gdk_texture = gdk::MemoryTexture::new(source_width, source_height, gdk_memory_format, &pixel_bytes, source_stride as usize);

                debug!("Created GDK texture from subsurface Wayland buffer: {}x{}", source_width, source_height);
                Some(gdk_texture.upcast_ref::<gdk::Texture>().clone())
            }) else {
                return None;
            };
            Some(texture)
        })
    }

    fn get_buffer_assignment(&self, surface_data: &SurfaceData) -> BufferAssignment {
        surface_data
            .cached_state
            .get::<SurfaceAttributes>()
            .current()
            .buffer
            .as_ref()
            .map(|assignment_type| match assignment_type {
                BufferAssignment::NewBuffer(buffer) => BufferAssignment::NewBuffer(buffer.clone()),
                BufferAssignment::Removed => BufferAssignment::Removed,
            })
            .unwrap_or(BufferAssignment::Removed)
    }

    fn get_new_buffer_assignment(&self, surface_data: &SurfaceData) -> Option<WlBuffer> {
        surface_data
            .cached_state
            .get::<SurfaceAttributes>()
            .current()
            .buffer
            .as_ref()
            .map(|assignment_type| match assignment_type {
                BufferAssignment::NewBuffer(buffer) => Some(buffer.clone()),
                BufferAssignment::Removed => None,
            })
            .flatten()
    }

    /// Renders a popup surface (like menus or tooltips) to a GDK texture
    pub fn render_popup_to_texture(&self, popup: &smithay::desktop::PopupKind, compositor: &SmearorCompositor) -> Option<gdk::Texture> {
        // Get the Wayland surface from the popup
        let wayland_surface = match popup {
            smithay::desktop::PopupKind::Xdg(xdg) => xdg.wl_surface(),
            smithay::desktop::PopupKind::InputMethod(_input_method) => {
                debug!("Input method popup rendering not yet implemented");
                return None;
            }
        };

        debug!("Attempting to render popup surface: {:?}", wayland_surface.id());

        let surface_id = wayland_surface.id();

        // Check if popup buffer is in Buffer-Holding-Area (DMA-BUF support)
        if let Ok(holding_area) = compositor.buffer_holding_area.lock() {
            if let Some(buffer) = holding_area.get(&surface_id) {
                debug!("Using popup buffer from Buffer-Holding-Area for surface: {:?}", surface_id);

                // Check if popup buffer is DMA-BUF (hardware-accelerated)
                if let Ok(_) = get_dmabuf(buffer) {
                    debug!("Detected DMA-BUF buffer for popup surface: {:?}", surface_id);

                    // Check if DMA-BUF is enabled
                    if compositor.is_dma_buf_available() {
                        debug!("DMA-BUF is enabled, importing DMA-BUF texture for popup");

                        // Import DMA-BUF buffer directly to GDK texture
                        if let Some(texture) = self.import_dmabuf_texture(buffer) {
                            debug!("Successfully imported DMA-BUF texture for popup surface: {:?}", surface_id);
                            return Some(texture);
                        } else {
                            debug!("Failed to import DMA-BUF texture for popup, falling back to SHM");
                        }
                    } else {
                        debug!("DMA-BUF is disabled via --disable-dma-buf flag, falling back to SHM for popup");
                    }
                } else {
                    debug!("Popup buffer is not DMA-BUF, using SHM fallback");
                }
            }
        }

        // Check if buffer data is cached from commit time
        if let Some(mut texture_cache_entry) = compositor.texture_cache.get_mut(&surface_id) {
            debug!("Using cached buffer data for popup surface: {:?}", surface_id);
            let texture_cache_entry = texture_cache_entry.value_mut();
            debug!("Cached popup data dimensions: {texture_cache_entry}");

            // Apply color mask if set and not already applied
            if let Some(mask_color) = compositor.get_color_mask() {
                if !texture_cache_entry.color_mask_applied {
                    debug!("Applying color mask to cached popup buffer data for surface: {:?}", surface_id);
                    if let Some(background_color) = compositor.get_background_color() {
                        // Replace mask color with background color
                        texture_cache_entry.replace_color(mask_color, background_color);
                    } else {
                        // Apply chroma-keying (make transparent)
                        texture_cache_entry.apply_color_mask(mask_color);
                    }
                } else {
                    debug!("Color mask already applied to cached popup buffer data for surface: {:?}", surface_id);
                }
            }

            let gdk_texture = create_memory_texture_bgra(&texture_cache_entry);
            // let pixel_bytes = Bytes::from(&data[..]);
            // let gdk_memory_format = gdk::MemoryFormat::B8g8r8a8;
            // let gdk_texture = gdk::MemoryTexture::new(
            //     buffer_metadata.width,
            //     buffer_metadata.height,
            //     gdk_memory_format,
            //     &pixel_bytes,
            //     buffer_metadata.stride as usize,
            // );

            debug!("Created GDK texture from cached popup buffer data: {}", texture_cache_entry.buffer_metadata);
            return Some(gdk_texture.upcast_ref::<gdk::Texture>().clone());
        }

        debug!("No cached buffer data found for popup, falling back to direct buffer read");

        if let Some(texture) = with_states(&wayland_surface, |surface_data| {
            debug!("Successfully accessed popup surface data for rendering");

            let buffer_assignment = self.get_buffer_assignment(surface_data);

            debug!("Popup buffer assignment state: {:?}", buffer_assignment);

            if let BufferAssignment::NewBuffer(buffer_handle) = buffer_assignment {
                if let Ok(texture) = with_buffer_contents(&buffer_handle, |memory_pointer, data_length, buffer_metadata| {
                    debug!(
                        "Rendering popup SHM buffer: {}x{} with stride {}",
                        buffer_metadata.width, buffer_metadata.height, buffer_metadata.stride
                    );

                    if data_length == 0 {
                        debug!("Popup buffer data length is zero, skipping render");
                        return None;
                    }

                    let source_width = buffer_metadata.width;
                    let source_height = buffer_metadata.height;
                    let source_stride = buffer_metadata.stride;

                    let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };

                    let pixel_bytes = Bytes::from_owned(pixel_data_slice.to_vec());
                    let gdk_memory_format = gdk::MemoryFormat::B8g8r8a8;

                    let gdk_texture = gdk::MemoryTexture::new(source_width, source_height, gdk_memory_format, &pixel_bytes, source_stride as usize);

                    debug!("Created GDK texture from popup Wayland buffer: {}x{}", source_width, source_height);
                    Some(gdk_texture.upcast_ref::<gdk::Texture>().clone())
                }) {
                    return texture;
                } else {
                    debug!("Could not read popup buffer contents via Smithay");
                }
            } else {
                debug!("No buffer on popup surface");
            }
            None
        }) {
            return Some(texture);
        } else {
            debug!("Failed to access popup Wayland surface states");
        }

        None
    }

    fn create_test_texture(&self, width: i32, height: i32) -> Option<gdk::Texture> {
        debug!("Creating test texture (fallback) at {}x{}", width, height);
        let texture_stride = width * 4;
        let mut fallback_pixel_data = vec![0u8; (width * height * 4) as usize];

        // Gradient from 10% dark gray (top) to 20% dark gray (bottom)
        // Both with 50% transparency
        let top_gray = 26; // 10% of 255
        let bottom_gray = 51; // 20% of 255
        let alpha = 128; // 50% transparency

        for vertical_index in 0..height {
            let vertical_ratio = vertical_index as f32 / height as f32;
            let gray = (top_gray as f32 + (bottom_gray as f32 - top_gray as f32) * vertical_ratio) as u8;

            for _horizontal_index in 0..width {
                let pixel_offset = ((vertical_index * width + _horizontal_index) * 4) as usize;
                fallback_pixel_data[pixel_offset] = gray; // Blue
                fallback_pixel_data[pixel_offset + 1] = gray; // Green
                fallback_pixel_data[pixel_offset + 2] = gray; // Red
                fallback_pixel_data[pixel_offset + 3] = alpha; // Alpha
            }
        }

        let fallback_bytes = Bytes::from_owned(fallback_pixel_data);
        let fallback_memory_format = gdk::MemoryFormat::B8g8r8a8;

        let fallback_texture = gdk::MemoryTexture::new(width, height, fallback_memory_format, &fallback_bytes, texture_stride as usize);

        Some(fallback_texture.upcast_ref::<gdk::Texture>().clone())
    }

    /// TODO: Phase 6 - Dialog Management - Render dialog to texture
    /// Renders a dialog surface to a GDK texture for display
    pub fn render_dialog_to_texture(&self, dialog: &smithay::wayland::shell::xdg::ToplevelSurface, compositor: &SmearorCompositor) -> Option<gdk::Texture> {
        let dialog_surface = dialog.wl_surface();
        debug!("Rendering dialog to texture for surface");

        // Check if dialog buffer is in Buffer-Holding-Area (DMA-BUF support)
        let object_id = dialog_surface.id();
        if let Ok(holding_area) = compositor.buffer_holding_area.lock() {
            if let Some(buffer) = holding_area.get(&object_id) {
                debug!("Using dialog buffer from Buffer-Holding-Area for surface: {:?}", object_id);

                // Check if dialog buffer is DMA-BUF (hardware-accelerated)
                if let Ok(_) = get_dmabuf(buffer) {
                    debug!("Detected DMA-BUF buffer for dialog surface: {:?}", object_id);

                    // Check if DMA-BUF is enabled
                    if compositor.is_dma_buf_available() {
                        debug!("DMA-BUF is enabled, importing DMA-BUF texture for dialog");

                        // Import DMA-BUF buffer directly to GDK texture
                        if let Some(texture) = self.import_dmabuf_texture(buffer) {
                            debug!("Successfully imported DMA-BUF texture for dialog surface: {:?}", object_id);
                            return Some(texture);
                        } else {
                            debug!("Failed to import DMA-BUF texture for dialog, falling back to SHM");
                        }
                    } else {
                        debug!("DMA-BUF is disabled via --disable-dma-buf flag, falling back to SHM for dialog");
                    }
                } else {
                    debug!("Dialog buffer is not DMA-BUF, using SHM fallback");
                }
            }
        }

        // Try to get cached buffer data
        if let Some(mut texture_cache_entry_ref) = compositor.texture_cache.get_mut(&object_id) {
            let texture_cache_entry: &mut TextureCacheEntry<BGRA> = texture_cache_entry_ref.value_mut();
            debug!("Using cached dialog texture data: {}", texture_cache_entry.buffer_metadata);

            // Apply color mask if set and not already applied
            if let Some(mask_color) = compositor.get_color_mask() {
                if !texture_cache_entry.color_mask_applied {
                    debug!("Applying color mask to cached dialog buffer data for surface: {:?}", object_id);
                    if let Some(background_color) = compositor.get_background_color() {
                        // Replace mask color with background color
                        texture_cache_entry.replace_color(mask_color, background_color);
                    } else {
                        // Apply chroma-keying (make transparent)
                        texture_cache_entry.apply_color_mask(mask_color);
                    }
                } else {
                    debug!("Color mask already applied to cached dialog buffer data for surface: {:?}", object_id);
                }
            }
            return Some(create_memory_texture_bgra(&texture_cache_entry).upcast_ref::<gdk::Texture>().clone());
        }

        // Fall back to direct buffer reading
        if let Some(texture) = with_states(dialog_surface, |states| {
            let buffer_assignment = self.get_buffer_assignment(states);

            if let BufferAssignment::NewBuffer(buffer_handle) = buffer_assignment {
                if let Ok(texture) = with_buffer_contents(&buffer_handle, |memory_pointer, data_length, buffer_metadata| {
                    debug!(
                        "Rendering dialog SHM buffer: {}x{} with stride {}",
                        buffer_metadata.width, buffer_metadata.height, buffer_metadata.stride
                    );

                    if data_length == 0 {
                        debug!("Dialog buffer data length is zero, skipping render");
                        return None;
                    }

                    let source_width = buffer_metadata.width;
                    let source_height = buffer_metadata.height;
                    let source_stride = buffer_metadata.stride;

                    let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };

                    let pixel_bytes = Bytes::from_owned(pixel_data_slice.to_vec());
                    let gdk_memory_format = gdk::MemoryFormat::B8g8r8a8;

                    let gdk_texture = gdk::MemoryTexture::new(source_width, source_height, gdk_memory_format, &pixel_bytes, source_stride as usize);

                    debug!("Created GDK texture from dialog Wayland buffer: {}x{}", source_width, source_height);
                    Some(gdk_texture.upcast_ref::<gdk::Texture>().clone())
                }) {
                    return texture;
                }
            }
            None
        }) {
            return Some(texture);
        } else {
            debug!("Failed to access dialog Wayland surface states");
        }

        None
    }
}
