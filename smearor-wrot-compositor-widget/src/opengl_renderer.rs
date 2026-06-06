//! OpenGL renderer for DMA-BUF zero-copy rendering
//!
//! This module provides OpenGL rendering capabilities using EGL for
//! hardware-accelerated rendering and DMA-BUF buffer export.

use smithay::backend::allocator::Fourcc;
use smithay::backend::allocator::Modifier;
use smithay::backend::allocator::dmabuf::AsDmabuf;
use smithay::backend::allocator::format::FormatSet;
use smithay::backend::allocator::gbm::GbmAllocator;
use smithay::backend::allocator::gbm::GbmBufferFlags;
use smithay::backend::allocator::gbm::GbmDevice;
use smithay::backend::egl::EGLContext;
use smithay::backend::egl::EGLDisplay;
use smithay::backend::egl::ffi;
use smithay::backend::egl::native::EGLNativeDisplay;
use smithay::backend::egl::native::EGLPlatform;
use smithay::backend::renderer::Bind;
use smithay::backend::renderer::ImportDma;
use smithay::backend::renderer::Offscreen;
use smithay::backend::renderer::Renderer;
use smithay::backend::renderer::Texture;
use smithay::backend::renderer::gles::GlesRenderbuffer;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::renderer::gles::GlesTexture;
use smithay::utils::Size;
use smithay::utils::Transform;
use std::fs::File;
use tracing::debug;

use smearor_wrot_compositor::dma::buffer::DmaBufBuffer;

use gtk4::gdk;
use smithay::backend::renderer::gles::ffi::Gles2;

// OpenGL ES constants

/// Surfaceless EGL display wrapper for EGL native display
struct SurfacelessEglDisplay;

unsafe impl Send for SurfacelessEglDisplay {}
unsafe impl Sync for SurfacelessEglDisplay {}

impl EGLNativeDisplay for SurfacelessEglDisplay {
    fn supported_platforms(&self) -> Vec<EGLPlatform<'_>> {
        vec![
            // EGL_MESA_platform_surfaceless
            EGLPlatform::new(
                ffi::egl::PLATFORM_SURFACELESS_MESA,
                "PLATFORM_SURFACELESS_MESA",
                std::ptr::null_mut(),
                vec![ffi::egl::NONE as ffi::EGLint],
                &["EGL_MESA_platform_surfaceless"],
            ),
        ]
    }
}

/// OpenGL renderer with EGL context
pub struct OpenGLRenderer {
    renderer: Option<GlesRenderer>,
    gbm_device: Option<std::sync::Arc<GbmDevice<File>>>,
}

impl OpenGLRenderer {
    /// Create a new OpenGL renderer with surfaceless EGL
    ///
    /// This uses surfaceless EGL for offscreen rendering without a display device.
    /// Suitable for shader rendering and texture operations.
    ///
    /// # Arguments
    ///
    /// * `gbm_device` - GBM device for DMA-BUF buffer allocation
    pub fn new_surfaceless(gbm_device: GbmDevice<File>) -> Result<Self, OpenGLRendererError> {
        debug!("Initializing OpenGL renderer with surfaceless EGL context");

        // Create surfaceless EGL native display
        let native_display = SurfacelessEglDisplay;

        // Create EGL display from native display
        let egl_display = unsafe { EGLDisplay::new(native_display) }
            .map_err(|e| OpenGLRendererError::EglInitializationFailed(format!("Failed to create EGL display: {:?}", e)))?;

        debug!("EGL display created successfully");

        // Create EGL context from EGL display
        let egl_context =
            EGLContext::new(&egl_display).map_err(|e| OpenGLRendererError::EglContextCreationFailed(format!("Failed to create EGL context: {:?}", e)))?;

        debug!("EGL context created successfully");

        // Create GlesRenderer with EGL context
        let renderer = unsafe { GlesRenderer::new(egl_context) }
            .map_err(|e| OpenGLRendererError::RendererInitializationFailed(format!("Failed to create GlesRenderer: {:?}", e)))?;

        debug!("GlesRenderer created successfully");

        Ok(Self {
            renderer: Some(renderer),
            gbm_device: Some(std::sync::Arc::new(gbm_device)),
        })
    }

    /// Create a new OpenGL renderer without EGL context (placeholder)
    ///
    /// This creates an uninitialized renderer that can be initialized later.
    /// Used when EGL context creation is deferred.
    pub fn new_uninitialized() -> Self {
        debug!("Creating uninitialized OpenGL renderer");
        Self {
            renderer: None,
            gbm_device: None,
        }
    }

    /// Get the underlying Smithay GlesRenderer
    pub fn renderer(&self) -> Option<&GlesRenderer> {
        self.renderer.as_ref()
    }

    /// Get mutable reference to the underlying Smithay GlesRenderer
    pub fn renderer_mut(&mut self) -> Option<&mut GlesRenderer> {
        self.renderer.as_mut()
    }

    /// Check if the renderer is initialized
    pub fn is_initialized(&self) -> bool {
        self.renderer.is_some()
    }

    /// Get the supported DMA-BUF formats from the renderer
    ///
    /// This returns the set of DMA-BUF formats that the renderer can import.
    /// Used for creating DmabufFeedback to advertise supported formats to clients.
    ///
    /// TODO: Phase 7 - DMA-BUF support for hardware acceleration - Use formats for DmabufFeedback
    /// This requires:
    /// 1. Call this method when initializing DmabufGlobal
    /// 2. Pass the formats to DmabufFeedbackBuilder
    /// 3. Create DmabufGlobal with the feedback
    pub fn dmabuf_formats(&self) -> Option<FormatSet> {
        self.renderer.as_ref().map(|renderer| renderer.dmabuf_formats())
    }

    /// Render to an offscreen framebuffer
    ///
    /// This creates an offscreen renderbuffer and renders content to it.
    /// Returns the renderbuffer that can be used for further operations.
    ///
    /// # Arguments
    ///
    /// * `width` - Buffer width in pixels
    /// * `height` - Buffer height in pixels
    /// * `format` - Buffer format (e.g., ARGB8888)
    ///
    /// # Returns
    ///
    /// * `Result<GlesRenderbuffer, OpenGLRendererError>` - Renderbuffer
    pub fn render_to_offscreen(&mut self, width: u32, height: u32, format: Fourcc) -> Result<GlesRenderbuffer, OpenGLRendererError> {
        debug!("Rendering to offscreen framebuffer: {}x{}, format: {:?}", width, height, format);

        let renderer = self
            .renderer
            .as_mut()
            .ok_or_else(|| OpenGLRendererError::RendererInitializationFailed("Renderer not initialized".to_string()))?;

        let size = Size::from((width as i32, height as i32));

        // Create offscreen renderbuffer
        let mut renderbuffer = Offscreen::<GlesRenderbuffer>::create_buffer(renderer, format, size)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create renderbuffer: {:?}", e)))?;

        debug!("Created offscreen renderbuffer: {}x{}", width, height);

        {
            let mut framebuffer =
                Bind::bind(renderer, &mut renderbuffer).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind renderbuffer: {:?}", e)))?;

            let physical_size = Size::from((width as i32, height as i32));
            let mut frame = Renderer::render(renderer, &mut framebuffer, physical_size, Transform::Normal)
                .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create frame: {:?}", e)))?;

            frame
                .with_context(|gl| {
                    unsafe {
                        gl.Viewport(0, 0, width as i32, height as i32);
                        gl.ClearColor(0.0, 0.0, 0.0, 0.0);
                        gl.Clear(glow::COLOR_BUFFER_BIT);
                    }
                    Ok(())
                })
                .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to clear renderbuffer: {:?}", e)))??;
        }

        Ok(renderbuffer)
    }

    /// Render to an offscreen framebuffer and export as DMA-BUF
    ///
    /// This creates a GBM buffer, renders content to it using OpenGL, and exports it as DMA-BUF.
    /// The DMA-BUF can be used with GdkDmabufTextureBuilder for GTK4 integration.
    ///
    /// # Arguments
    ///
    /// * `width` - Buffer width in pixels
    /// * `height` - Buffer height in pixels
    /// * `format` - Buffer format (e.g., ARGB8888)
    ///
    /// # Returns
    ///
    /// * `Result<(GlesRenderbuffer, DmaBufBuffer), OpenGLRendererError>` - DMA-BUF buffer and renderbuffer
    pub fn render_to_dmabuf(&mut self, width: u32, height: u32, format: Fourcc) -> Result<(GlesRenderbuffer, DmaBufBuffer), OpenGLRendererError> {
        debug!("Rendering to DMA-BUF: {}x{}, format: {:?}", width, height, format);

        let renderer = self.renderer.as_mut().ok_or(OpenGLRendererError::RendererNotInitialized)?;

        // Check if GBM device is available
        let gbm_device_arc = self
            .gbm_device
            .as_ref()
            .ok_or_else(|| OpenGLRendererError::RenderingFailed("GBM device not available for DMA-BUF export".to_string()))?;

        let cloned_file = gbm_device_arc
            .as_ref()
            .try_clone()
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to clone file handle: {}", e)))?;

        let gbm_device = GbmDevice::new(cloned_file).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create GBM device: {}", e)))?;

        // Create GBM allocator from GBM device (requires ownership)
        // let gbm_device = (*gbm_device_arc.as_ref()).clone();
        let mut gbm_allocator = GbmAllocator::new(gbm_device, GbmBufferFlags::RENDERING);

        // Create GBM buffer with specified format and size
        let gbm_buffer = gbm_allocator
            .create_buffer_with_flags(width, height, format, &[Modifier::Invalid], GbmBufferFlags::RENDERING)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create GBM buffer: {:?}", e)))?;

        // Export GBM buffer as DMA-BUF using AsDmabuf trait
        let dmabuf = gbm_buffer
            .export()
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to export GBM buffer as DMA-BUF: {:?}", e)))?;

        // Import DMA-BUF as texture using Smithay's ImportDma trait
        let imported_texture = renderer
            .import_dmabuf(&dmabuf, None)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to import DMA-BUF: {:?}", e)))?;

        // Create renderbuffer from imported texture
        let size = Size::from((width as i32, height as i32));
        let mut renderbuffer = Offscreen::<GlesRenderbuffer>::create_buffer(renderer, format, size)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create renderbuffer: {:?}", e)))?;

        // Clone renderbuffer for return value before mutable borrow
        let renderbuffer_result = renderbuffer.clone();

        // Create physical size for frames
        let physical_size = Size::from((width as i32, height as i32));

        // Bind imported texture to create GlesTarget for texture ID access
        let mut texture_binding = imported_texture.clone();
        let mut input_target =
            Bind::bind(renderer, &mut texture_binding).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind input texture: {:?}", e)))?;

        // Bind renderbuffer for rendering
        let mut output_target =
            Bind::bind(renderer, &mut renderbuffer).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind renderbuffer: {:?}", e)))?;

        // Create input frame to access texture ID (scope-limited to avoid borrow conflicts)
        let input_frame = {
            Renderer::render(renderer, &mut input_target, physical_size, Transform::Normal)
                .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create input frame: {:?}", e)))?;
        };

        // Render the imported texture to the renderbuffer
        let mut frame = Renderer::render(renderer, &mut output_target, physical_size, Transform::Normal)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create frame: {:?}", e)))?;

        // Use with_context() to render the imported texture
        frame
            .with_context(|gl| {
                // Query the texture ID from the input framebuffer attachment
                let texture_id = unsafe {
                    let mut texture_id = 0;
                    gl.GetFramebufferAttachmentParameteriv(
                        glow::READ_FRAMEBUFFER,
                        glow::COLOR_ATTACHMENT0,
                        glow::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME,
                        &mut texture_id,
                    );
                    texture_id as u32
                };

                // Create and bind framebuffer
                let mut fbo = 0;
                unsafe { gl.GenFramebuffers(1, &mut fbo) };
                unsafe { gl.BindFramebuffer(glow::FRAMEBUFFER, fbo) };

                // Bind imported texture to texture unit 0
                unsafe { gl.ActiveTexture(0x84C0) }; // GL_TEXTURE0
                unsafe { gl.BindTexture(glow::TEXTURE_2D, texture_id) };

                // Set viewport
                unsafe { gl.Viewport(0, 0, width as i32, height as i32) };

                // Clear
                unsafe { gl.ClearColor(0.0, 0.0, 0.0, 0.0) };
                unsafe { gl.Clear(glow::COLOR_BUFFER_BIT) };

                // Simple full-screen quad rendering
                let vertices: [f32; 16] = [-1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0];

                let mut vbo = 0;
                unsafe { gl.GenBuffers(1, &mut vbo) };
                unsafe { gl.BindBuffer(glow::ARRAY_BUFFER, vbo) };
                unsafe {
                    gl.BufferData(
                        glow::ARRAY_BUFFER,
                        std::mem::size_of_val(&vertices) as isize,
                        vertices.as_ptr() as *const std::ffi::c_void,
                        glow::STATIC_DRAW,
                    )
                };

                // Simple vertex shader
                let vertex_shader_source = r#"
                    #version 320 es
                    precision highp float;
                    layout(location = 0) in vec2 a_position;
                    layout(location = 1) in vec2 a_tex_coord;
                    out vec2 v_tex_coord;
                    void main() {
                        gl_Position = vec4(a_position, 0.0, 1.0);
                        v_tex_coord = a_tex_coord;
                    }
                "#;

                // Simple fragment shader
                let fragment_shader_source = r#"
                    #version 320 es
                    precision highp float;
                    in vec2 v_tex_coord;
                    uniform sampler2D u_texture;
                    out vec4 frag_color;
                    void main() {
                        frag_color = texture(u_texture, v_tex_coord);
                    }
                "#;

                // Compile shaders
                let vertex_shader = match Self::compile_shader_internal(gl, glow::VERTEX_SHADER, vertex_shader_source) {
                    Ok(shader) => shader,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to compile vertex shader: {:?}", e))),
                };

                let fragment_shader = match Self::compile_shader_internal(gl, glow::FRAGMENT_SHADER, fragment_shader_source) {
                    Ok(shader) => shader,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to compile fragment shader: {:?}", e))),
                };

                // Link program
                let program = match Self::link_program_internal(gl, vertex_shader, fragment_shader) {
                    Ok(prog) => prog,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to link program: {:?}", e))),
                };

                // Clean up shaders
                unsafe { gl.DeleteShader(vertex_shader) };
                unsafe { gl.DeleteShader(fragment_shader) };

                // Use program
                unsafe { gl.UseProgram(program) };

                // Set up vertex attributes
                let position_loc = unsafe { gl.GetAttribLocation(program, b"a_position\0".as_ptr() as *const i8) };
                let tex_coord_loc = unsafe { gl.GetAttribLocation(program, b"a_tex_coord\0".as_ptr() as *const i8) };

                unsafe { gl.EnableVertexAttribArray(position_loc as u32) };
                unsafe { gl.VertexAttribPointer(position_loc as u32, 2, glow::FLOAT, glow::FALSE, 4 * 4, std::ptr::null()) };

                unsafe { gl.EnableVertexAttribArray(tex_coord_loc as u32) };
                unsafe { gl.VertexAttribPointer(tex_coord_loc as u32, 2, glow::FLOAT, glow::FALSE, 4 * 4, (2 * 4) as *const std::ffi::c_void) };

                // Set texture sampler uniform
                let texture_loc = unsafe { gl.GetUniformLocation(program, b"u_texture\0".as_ptr() as *const i8) };
                unsafe { gl.Uniform1i(texture_loc, 0) };

                // Draw quad
                unsafe { gl.DrawArrays(glow::TRIANGLE_STRIP, 0, 4) };

                // Clean up
                unsafe { gl.DeleteBuffers(1, &vbo) };
                unsafe { gl.DeleteFramebuffers(1, &fbo) };
                unsafe { gl.DeleteProgram(program) };

                Ok(())
            })
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("OpenGL ES rendering failed: {:?}", e)))?;

        // Convert Smithay Dmabuf to smearor-wrot DmaBufBuffer
        let dmabuf_buffer = DmaBufBuffer::from_smithay_dmabuf(dmabuf, width, height, format)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to convert DMA-BUF: {:?}", e)))?;

        Ok((renderbuffer_result, dmabuf_buffer))
    }

    /// Convert DMA-BUF buffer to gdk::Texture
    ///
    /// This uses gdk::DmabufTextureBuilder to create a hardware-accelerated texture
    /// from the DMA-BUF buffer for GTK4 integration.
    ///
    /// # Arguments
    ///
    /// * `dmabuf_buffer` - DMA-BUF buffer to convert
    /// * `display` - GDK display for texture creation
    ///
    /// # Returns
    ///
    /// * `Result<gdk::Texture, OpenGLRendererError>` - GTK4 texture
    pub fn dmabuf_to_texture(&self, dmabuf_buffer: &DmaBufBuffer, display: &gdk::Display) -> Result<gdk::Texture, OpenGLRendererError> {
        debug!("Converting DMA-BUF to gdk::Texture: {}x{}", dmabuf_buffer.width, dmabuf_buffer.height);

        use gtk4::gdk::DmabufTextureBuilder;

        // Create DmabufTextureBuilder for DMA-BUF to gdk::Texture conversion
        let mut builder = DmabufTextureBuilder::new()
            .set_display(display)
            .set_width(dmabuf_buffer.width)
            .set_height(dmabuf_buffer.height)
            .set_fourcc(dmabuf_buffer.format)
            .set_modifier(0) // Assuming linear modifier for now
            .set_n_planes(1); // Assuming single plane for now

        // Set FD, stride, and offset for plane 0
        builder = unsafe { builder.set_fd(0, dmabuf_buffer.fd) };
        builder = builder.set_stride(0, dmabuf_buffer.stride as u32).set_offset(0, 0);

        // Build the texture
        let texture =
            unsafe { builder.build() }.map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to build gdk::Texture from DMA-BUF: {:?}", e)))?;

        debug!("Successfully converted DMA-BUF to gdk::Texture");
        Ok(texture)
    }

    /// Apply color mask shader to a texture
    ///
    /// This renders a texture with a color mask shader applied.
    /// The shader replaces pixels matching the mask color with transparency.
    ///
    /// # Arguments
    ///
    /// * `texture` - Input texture to apply mask to
    /// * `mask_color` - RGB color to mask (0.0-1.0 range)
    /// * `tolerance` - Color tolerance for matching (0.0-1.0 range)
    ///
    /// # Returns
    ///
    /// * `Result<GlesRenderbuffer, OpenGLRendererError>` - Renderbuffer with masked texture
    pub fn apply_color_mask_shader(
        &mut self,
        texture: GlesTexture,
        mask_color: (f32, f32, f32),
        tolerance: f32,
    ) -> Result<GlesRenderbuffer, OpenGLRendererError> {
        debug!(
            "Applying color mask shader: color=({:.2}, {:.2}, {:.2}), tolerance={:.2}",
            mask_color.0, mask_color.1, mask_color.2, tolerance
        );

        let renderer = self.renderer.as_mut().ok_or(OpenGLRendererError::RendererNotInitialized)?;

        // Get texture dimensions
        let size = texture.size();
        let width = size.w;
        let height = size.h;

        // Custom fragment shader for color masking
        // Must support both sampler2D and samplerExternalOES for DMA-BUF
        // SIMPLIFIED VERSION FOR DEBUGGING - just pass through texture
        let fragment_shader_source = r#"
            //_DEFINES
            #if defined(EXTERNAL)
            #extension GL_OES_EGL_image_external : require
            #endif

            precision highp float;
            #if defined(EXTERNAL)
            uniform samplerExternalOES tex;
            #else
            uniform sampler2D tex;
            #endif

            uniform float alpha;
            uniform vec3 u_mask_color;
            uniform float u_tolerance;
            varying vec2 v_coords;

            void main() {
                vec4 tex_color = texture2D(tex, v_coords);
                vec3 color_diff = abs(tex_color.rgb - u_mask_color);
                float max_diff = max(max(color_diff.r, color_diff.g), color_diff.b);
                if (max_diff < u_tolerance) {
                    gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0); // Transparent
                } else {
                    gl_FragColor = tex_color * alpha;
                }
            }
        "#;

        // Compile custom shader with additional uniforms BEFORE creating frame
        use smithay::backend::renderer::gles::Uniform;
        use smithay::backend::renderer::gles::UniformName;
        use smithay::backend::renderer::gles::UniformType;
        use smithay::backend::renderer::gles::UniformValue;
        let additional_uniforms = &[
            UniformName::new("u_mask_color", UniformType::_3f),
            UniformName::new("u_tolerance", UniformType::_1f),
        ];

        let shader_program = renderer
            .compile_custom_texture_shader(fragment_shader_source, additional_uniforms)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to compile custom shader: {:?}", e)))?;

        debug!("Custom shader compiled successfully");

        // Set custom shader uniforms
        let additional_uniforms_values = &[
            Uniform::new("u_mask_color", UniformValue::_3f(mask_color.0, mask_color.1, mask_color.2)),
            Uniform::new("u_tolerance", UniformValue::_1f(tolerance)),
        ];

        debug!(
            "Setting custom shader uniforms: mask_color=({:.2}, {:.2}, {:.2}), tolerance={:.2}",
            mask_color.0, mask_color.1, mask_color.2, tolerance
        );

        // Create output renderbuffer
        let format = Fourcc::Argb8888;
        let output_size = texture.size();
        let mut output_renderbuffer = Offscreen::<GlesRenderbuffer>::create_buffer(renderer, format, output_size)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create output renderbuffer: {:?}", e)))?;

        // Clone output_renderbuffer for return value before mutable borrow
        let output_result = output_renderbuffer.clone();

        // Bind output renderbuffer
        let mut output_target = Bind::bind(renderer, &mut output_renderbuffer)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind output renderbuffer: {:?}", e)))?;

        // Create frame for rendering
        let physical_size = Size::from((width, height));
        let mut frame = Renderer::render(renderer, &mut output_target, physical_size, Transform::Normal)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create frame: {:?}", e)))?;

        // Render texture with custom shader
        use smithay::utils::Point;
        use smithay::utils::Rectangle;
        let src = Rectangle::new(Point::from((0.0, 0.0)), Size::from((width as f64, height as f64)));
        let dst = Rectangle::new(Point::from((0, 0)), physical_size);
        let damage = &[dst];

        frame
            .render_texture_from_to(
                &texture,
                src,
                dst,
                damage,
                damage,
                Transform::Normal,
                1.0,
                Some(&shader_program), // Use custom shader instead of Smithay's default
                additional_uniforms_values,
            )
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to render texture: {:?}", e)))?;

        debug!("Color mask shader applied successfully");
        Ok(output_result)
    }

    /// Apply transform shader to texture
    ///
    /// This function applies transform effects (fade, rotate, scale) to a texture using OpenGL ES shaders.
    ///
    /// # Arguments
    ///
    /// * `texture` - Input texture to apply transform to
    /// * `fade_factor` - Fade factor (0.0 = fully transparent, 1.0 = fully opaque)
    /// * `rotation_angle` - Rotation angle in degrees
    /// * `scale_factor` - Scale factor (1.0 = original size)
    ///
    /// # Returns
    ///
    /// * `Result<GlesRenderbuffer, OpenGLRendererError>` - Renderbuffer with transformed texture
    pub fn apply_transform_shader(
        &mut self,
        texture: GlesTexture,
        fade_factor: f32,
        rotation_angle: f32,
        scale_factor: f32,
    ) -> Result<GlesRenderbuffer, OpenGLRendererError> {
        debug!(
            "Applying animation shader: fade={:.2}, rotation={:.2}°, scale={:.2}",
            fade_factor, rotation_angle, scale_factor
        );

        let renderer = self.renderer.as_mut().ok_or(OpenGLRendererError::RendererNotInitialized)?;

        // Get texture dimensions from GlesTexture
        let size = texture.size();
        let width = size.w;
        let height = size.h;

        // Create output renderbuffer using Smithay's Offscreen::create_buffer
        let format = Fourcc::Argb8888;
        let output_size = texture.size();
        let mut output_renderbuffer = Offscreen::<GlesRenderbuffer>::create_buffer(renderer, format, output_size)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create output renderbuffer: {:?}", e)))?;

        // Clone output_renderbuffer for return value before mutable borrow
        let output_result = output_renderbuffer.clone();

        // Create physical size for frames
        let physical_size = Size::from((width, height));

        // Bind input texture to create GlesTarget for texture ID access
        let mut texture_binding = texture.clone();
        let mut input_target =
            Bind::bind(renderer, &mut texture_binding).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind input texture: {:?}", e)))?;

        // Bind output renderbuffer to create GlesTarget
        let mut output_target = Bind::bind(renderer, &mut output_renderbuffer)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind output renderbuffer: {:?}", e)))?;

        // Create input frame to access texture ID
        let input_frame = {
            Renderer::render(renderer, &mut input_target, physical_size, Transform::Normal)
                .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create input frame: {:?}", e)))?;
        };

        // Create GlesFrame for rendering with Physical size
        let mut frame = Renderer::render(renderer, &mut output_target, physical_size, Transform::Normal)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create frame: {:?}", e)))?;

        // Use with_context() for direct OpenGL ES calls
        frame
            .with_context(|gl| {
                // Animation shader GLSL
                let vertex_shader_source = r#"
                #version 320 es
                precision highp float;
                layout(location = 0) in vec2 a_position;
                layout(location = 1) in vec2 a_tex_coord;
                uniform float u_rotation_angle;
                uniform float u_scale_factor;
                out vec2 v_tex_coord;
                void main() {
                    // Convert angle to radians
                    float angle = radians(u_rotation_angle);
                    float cos_a = cos(angle);
                    float sin_a = sin(angle);
                    
                    // Apply rotation and scale
                    vec2 scaled_pos = a_position * u_scale_factor;
                    vec2 rotated_pos = vec2(
                        scaled_pos.x * cos_a - scaled_pos.y * sin_a,
                        scaled_pos.x * sin_a + scaled_pos.y * cos_a
                    );
                    
                    gl_Position = vec4(rotated_pos, 0.0, 1.0);
                    v_tex_coord = a_tex_coord;
                }
            "#;

                let fragment_shader_source = r#"
                #version 320 es
                precision highp float;
                in vec2 v_tex_coord;
                uniform sampler2D u_texture;
                uniform float u_fade_factor;
                out vec4 frag_color;
                void main() {
                    vec4 tex_color = texture(u_texture, v_tex_coord);
                    frag_color = vec4(tex_color.rgb, tex_color.a * u_fade_factor);
                }
            "#;

                // Compile shaders
                let vertex_shader = match Self::compile_shader_internal(gl, glow::VERTEX_SHADER, vertex_shader_source) {
                    Ok(shader) => shader,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to compile vertex shader: {:?}", e))),
                };

                let fragment_shader = match Self::compile_shader_internal(gl, glow::FRAGMENT_SHADER, fragment_shader_source) {
                    Ok(shader) => shader,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to compile fragment shader: {:?}", e))),
                };

                // Link program
                let program = match Self::link_program_internal(gl, vertex_shader, fragment_shader) {
                    Ok(prog) => prog,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to link program: {:?}", e))),
                };

                // Clean up shaders
                unsafe { gl.DeleteShader(vertex_shader) };
                unsafe { gl.DeleteShader(fragment_shader) };

                // Set up framebuffer
                let mut fbo = 0;
                unsafe { gl.GenFramebuffers(1, &mut fbo) };
                unsafe { gl.BindFramebuffer(glow::FRAMEBUFFER, fbo) };

                // Create texture for output
                let mut output_texture = 0;
                unsafe { gl.GenTextures(1, &mut output_texture) };
                unsafe { gl.BindTexture(glow::TEXTURE_2D, output_texture) };
                unsafe { gl.TexImage2D(glow::TEXTURE_2D, 0, glow::RGBA8 as i32, width, height, 0, glow::RGBA, glow::UNSIGNED_BYTE, std::ptr::null()) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32) };

                // Attach texture to framebuffer
                unsafe { gl.FramebufferTexture2D(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, output_texture, 0) };

                // Check framebuffer status
                let status = unsafe { gl.CheckFramebufferStatus(glow::FRAMEBUFFER) };
                if status != glow::FRAMEBUFFER_COMPLETE {
                    return Err(OpenGLRendererError::RenderingFailed(format!("Framebuffer incomplete: {}", status)));
                }

                // Set viewport
                unsafe { gl.Viewport(0, 0, width, height) };

                // Clear
                unsafe { gl.ClearColor(0.0, 0.0, 0.0, 0.0) };
                unsafe { gl.Clear(glow::COLOR_BUFFER_BIT) };

                // Use program
                unsafe { gl.UseProgram(program) };

                // Set up vertex buffer for full-screen quad
                let vertices: [f32; 16] = [-1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0];

                let mut vbo = 0;
                unsafe { gl.GenBuffers(1, &mut vbo) };
                unsafe { gl.BindBuffer(glow::ARRAY_BUFFER, vbo) };
                unsafe {
                    gl.BufferData(
                        glow::ARRAY_BUFFER,
                        std::mem::size_of_val(&vertices) as isize,
                        vertices.as_ptr() as *const std::ffi::c_void,
                        glow::STATIC_DRAW,
                    )
                };

                // Set up vertex attributes
                let position_loc = unsafe { gl.GetAttribLocation(program, b"a_position\0".as_ptr() as *const i8) };
                let tex_coord_loc = unsafe { gl.GetAttribLocation(program, b"a_tex_coord\0".as_ptr() as *const i8) };

                unsafe { gl.EnableVertexAttribArray(position_loc as u32) };
                unsafe { gl.VertexAttribPointer(position_loc as u32, 2, glow::FLOAT, glow::FALSE, 4 * 4, std::ptr::null()) };

                unsafe { gl.EnableVertexAttribArray(tex_coord_loc as u32) };
                unsafe { gl.VertexAttribPointer(tex_coord_loc as u32, 2, glow::FLOAT, glow::FALSE, 4 * 4, (2 * 4) as *const std::ffi::c_void) };

                // Bind input texture for shader sampling
                let input_texture_id = unsafe {
                    let mut texture_id = 0;
                    gl.GetFramebufferAttachmentParameteriv(
                        glow::READ_FRAMEBUFFER,
                        glow::COLOR_ATTACHMENT0,
                        glow::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME,
                        &mut texture_id,
                    );
                    texture_id as u32
                };

                // Bind the input texture to texture unit 0
                unsafe { gl.ActiveTexture(0x84C0) }; // GL_TEXTURE0
                unsafe { gl.BindTexture(glow::TEXTURE_2D, input_texture_id) };

                // Set texture sampler uniform to texture unit 0
                let texture_loc = unsafe { gl.GetUniformLocation(program, b"u_texture\0".as_ptr() as *const i8) };
                unsafe { gl.Uniform1i(texture_loc, 0) };

                // Set uniform values
                let fade_factor_loc = unsafe { gl.GetUniformLocation(program, b"u_fade_factor\0".as_ptr() as *const i8) };
                let rotation_angle_loc = unsafe { gl.GetUniformLocation(program, b"u_rotation_angle\0".as_ptr() as *const i8) };
                let scale_factor_loc = unsafe { gl.GetUniformLocation(program, b"u_scale_factor\0".as_ptr() as *const i8) };

                unsafe { gl.Uniform1f(fade_factor_loc, fade_factor) };
                unsafe { gl.Uniform1f(rotation_angle_loc, rotation_angle) };
                unsafe { gl.Uniform1f(scale_factor_loc, scale_factor) };

                // Draw quad
                unsafe { gl.DrawArrays(glow::TRIANGLE_STRIP, 0, 4) };

                // Clean up
                unsafe { gl.DeleteBuffers(1, &vbo) };
                unsafe { gl.DeleteFramebuffers(1, &fbo) };
                unsafe { gl.DeleteTextures(1, &output_texture) };
                unsafe { gl.DeleteProgram(program) };

                Ok(())
            })
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("OpenGL ES rendering failed: {:?}", e)))?;

        Ok(output_result)
    }

    /// Read GlesRenderbuffer content back to pixel data
    ///
    /// This function reads the content of a GlesRenderbuffer back to CPU memory
    /// for further processing or conversion to GDK textures.
    ///
    /// # Arguments
    ///
    /// * `renderbuffer` - The GlesRenderbuffer to read from
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, OpenGLRendererError>` - Pixel data in BGRA format
    pub fn read_renderbuffer_to_pixel_data(&mut self, renderbuffer: &GlesRenderbuffer) -> Result<Vec<u8>, OpenGLRendererError> {
        debug!("Reading GlesRenderbuffer content to pixel data");

        let renderer = self.renderer.as_mut().ok_or(OpenGLRendererError::RendererNotInitialized)?;

        // Get renderbuffer dimensions
        let size = renderbuffer.size();
        let width = size.w as u32;
        let height = size.h as u32;

        // Bind renderbuffer to create GlesTarget
        let mut renderbuffer_binding = renderbuffer.clone();
        let mut target = Bind::bind(renderer, &mut renderbuffer_binding)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind renderbuffer: {:?}", e)))?;

        // Create frame for reading
        let physical_size = Size::from((width as i32, height as i32));
        let mut frame = Renderer::render(renderer, &mut target, physical_size, Transform::Normal)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create frame: {:?}", e)))?;

        // Read pixel data using with_context()
        let pixel_data = frame
            .with_context(|gl| {
                let data_size = (width * height * 4) as usize; // BGRA = 4 bytes per pixel
                let mut pixel_data = vec![0u8; data_size];

                // Read pixels from current framebuffer
                unsafe {
                    gl.ReadPixels(
                        0,
                        0,
                        width as i32,
                        height as i32,
                        glow::RGBA,
                        glow::UNSIGNED_BYTE,
                        pixel_data.as_mut_ptr() as *mut std::ffi::c_void,
                    );
                }

                // Convert RGBA to BGRA (OpenGL reads as RGBA, we need BGRA)
                for chunk in pixel_data.chunks_exact_mut(4) {
                    let r = chunk[0];
                    let b = chunk[2];
                    chunk[0] = b;
                    chunk[2] = r;
                }

                Ok(pixel_data)
            })
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to read pixel data: {:?}", e)))??;

        debug!("Successfully read {} bytes from GlesRenderbuffer", pixel_data.len());
        Ok(pixel_data)
    }

    /// Compile a GLSL shader using Smithay GL context
    fn compile_shader_internal(
        gl: &Gles2,
        shader_type: smithay::backend::renderer::gles::ffi::types::GLenum,
        source: &str,
    ) -> Result<smithay::backend::renderer::gles::ffi::types::GLuint, OpenGLRendererError> {
        let shader = unsafe { gl.CreateShader(shader_type) };
        if shader == 0 {
            return Err(OpenGLRendererError::RenderingFailed("Failed to create shader".to_string()));
        }

        let c_source = std::ffi::CString::new(source).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to convert shader source: {:?}", e)))?;
        let c_source_ptr = c_source.as_ptr();

        unsafe {
            gl.ShaderSource(shader, 1, &c_source_ptr, std::ptr::null());
            gl.CompileShader(shader);

            let mut success = 0;
            gl.GetShaderiv(shader, glow::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut info_log_length = 0;
                gl.GetShaderiv(shader, glow::INFO_LOG_LENGTH, &mut info_log_length);

                let mut info_log = vec![0u8; info_log_length as usize];
                gl.GetShaderInfoLog(
                    shader,
                    info_log_length,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut smithay::backend::renderer::gles::ffi::types::GLchar,
                );

                let error_message = String::from_utf8_lossy(&info_log).to_string();
                gl.DeleteShader(shader);
                return Err(OpenGLRendererError::RenderingFailed(format!("Shader compilation failed: {}", error_message)));
            }
        }

        Ok(shader)
    }

    /// Link a shader program using Smithay GL context
    fn link_program_internal(
        gl: &Gles2,
        vertex_shader: smithay::backend::renderer::gles::ffi::types::GLuint,
        fragment_shader: smithay::backend::renderer::gles::ffi::types::GLuint,
    ) -> Result<smithay::backend::renderer::gles::ffi::types::GLuint, OpenGLRendererError> {
        let program = unsafe { gl.CreateProgram() };
        if program == 0 {
            return Err(OpenGLRendererError::RenderingFailed("Failed to create program".to_string()));
        }

        unsafe {
            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);
            gl.LinkProgram(program);

            let mut success = 0;
            gl.GetProgramiv(program, glow::LINK_STATUS, &mut success);

            if success == 0 {
                let mut info_log_length = 0;
                gl.GetProgramiv(program, glow::INFO_LOG_LENGTH, &mut info_log_length);

                let mut info_log = vec![0u8; info_log_length as usize];
                //
                gl.GetProgramInfoLog(
                    program,
                    info_log_length,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut smithay::backend::renderer::gles::ffi::types::GLchar,
                );

                let error_message = String::from_utf8_lossy(&info_log).to_string();
                gl.DeleteProgram(program);
                return Err(OpenGLRendererError::RenderingFailed(format!("Program linking failed: {}", error_message)));
            }
        }

        Ok(program)
    }

    /// Apply animation shader to a texture
    ///
    /// This renders a texture with an animation shader applied.
    /// Supports transitions, fades, and other visual effects.
    ///
    /// # Arguments
    ///
    /// * `texture` - Input texture to animate
    /// * `animation_type` - Type of animation (fade, slide, etc.)
    /// * `progress` - Animation progress (0.0-1.0 range)
    ///
    /// # Returns
    ///
    /// * `Result<GlesRenderbuffer, OpenGLRendererError>` - Renderbuffer with animated texture
    pub fn apply_animation_shader(&mut self, texture: GlesRenderbuffer, animation_type: &str, progress: f32) -> Result<GlesRenderbuffer, OpenGLRendererError> {
        debug!("Applying animation shader: type={}, progress={:.2}", animation_type, progress);

        let renderer = self.renderer.as_mut().ok_or(OpenGLRendererError::RendererNotInitialized)?;

        // Get texture dimensions from GlesRenderbuffer
        let size = texture.size();
        let width = size.w as u32;
        let height = size.h as u32;

        // Create output renderbuffer using Smithay's Offscreen::create_buffer
        let format = Fourcc::Argb8888;
        let output_size = Size::from((width as i32, height as i32));
        let mut output_renderbuffer = Offscreen::<GlesRenderbuffer>::create_buffer(renderer, format, output_size)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create output renderbuffer: {:?}", e)))?;

        // Clone output_renderbuffer for return value before mutable borrow
        let output_result = output_renderbuffer.clone();

        // Create physical size for frames
        let physical_size = Size::from((width as i32, height as i32));

        // Bind input texture to create GlesTarget for texture ID access
        let mut texture_binding = texture.clone();
        let mut input_target =
            Bind::bind(renderer, &mut texture_binding).map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind input texture: {:?}", e)))?;

        // Bind output renderbuffer to create GlesTarget
        let mut output_target = Bind::bind(renderer, &mut output_renderbuffer)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to bind output renderbuffer: {:?}", e)))?;

        // Create input frame to access texture ID (scope-limited to avoid borrow conflicts)
        let input_frame = {
            Renderer::render(renderer, &mut input_target, physical_size, Transform::Normal)
                .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create input frame: {:?}", e)))?;
        };

        // Create GlesFrame for rendering with Physical size
        let mut frame = Renderer::render(renderer, &mut output_target, physical_size, Transform::Normal)
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("Failed to create frame: {:?}", e)))?;

        // Use with_context() for direct OpenGL ES calls
        frame
            .with_context(|gl| {
                // Animation shader GLSL - supports fade animation
                let vertex_shader_source = r#"
                #version 320 es
                precision highp float;
                layout(location = 0) in vec2 a_position;
                layout(location = 1) in vec2 a_tex_coord;
                out vec2 v_tex_coord;
                void main() {
                    gl_Position = vec4(a_position, 0.0, 1.0);
                    v_tex_coord = a_tex_coord;
                }
            "#;

                let fragment_shader_source = r#"
                #version 320 es
                precision highp float;
                in vec2 v_tex_coord;
                uniform sampler2D u_texture;
                uniform float u_progress;
                out vec4 frag_color;
                void main() {
                    vec4 tex_color = texture(u_texture, v_tex_coord);
                    frag_color = vec4(tex_color.rgb, tex_color.a * u_progress);
                }
            "#;

                // Compile shaders
                let vertex_shader = match Self::compile_shader_internal(gl, glow::VERTEX_SHADER, vertex_shader_source) {
                    Ok(shader) => shader,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to compile vertex shader: {:?}", e))),
                };

                let fragment_shader = match Self::compile_shader_internal(gl, glow::FRAGMENT_SHADER, fragment_shader_source) {
                    Ok(shader) => shader,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to compile fragment shader: {:?}", e))),
                };

                // Link program
                let program = match Self::link_program_internal(gl, vertex_shader, fragment_shader) {
                    Ok(prog) => prog,
                    Err(e) => return Err(OpenGLRendererError::RenderingFailed(format!("Failed to link program: {:?}", e))),
                };

                // Clean up shaders
                unsafe { gl.DeleteShader(vertex_shader) };
                unsafe { gl.DeleteShader(fragment_shader) };

                // Set up framebuffer
                let mut fbo = 0;
                unsafe { gl.GenFramebuffers(1, &mut fbo) };
                unsafe { gl.BindFramebuffer(glow::FRAMEBUFFER, fbo) };

                // Create texture for output
                let mut output_texture = 0;
                unsafe { gl.GenTextures(1, &mut output_texture) };
                unsafe { gl.BindTexture(glow::TEXTURE_2D, output_texture) };
                unsafe {
                    gl.TexImage2D(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA8 as i32,
                        width as i32,
                        height as i32,
                        0,
                        glow::RGBA,
                        glow::UNSIGNED_BYTE,
                        std::ptr::null(),
                    )
                };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32) };
                unsafe { gl.TexParameteri(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32) };

                // Attach texture to framebuffer
                unsafe { gl.FramebufferTexture2D(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, output_texture, 0) };

                // Check framebuffer status
                let status = unsafe { gl.CheckFramebufferStatus(glow::FRAMEBUFFER) };
                if status != glow::FRAMEBUFFER_COMPLETE {
                    return Err(OpenGLRendererError::RenderingFailed(format!("Framebuffer incomplete: {}", status)));
                }

                // Set viewport
                unsafe { gl.Viewport(0, 0, width as i32, height as i32) };

                // Clear
                unsafe { gl.ClearColor(0.0, 0.0, 0.0, 0.0) };
                unsafe { gl.Clear(glow::COLOR_BUFFER_BIT) };

                // Use program
                unsafe { gl.UseProgram(program) };

                // Set up vertex buffer for full-screen quad
                let vertices: [f32; 16] = [-1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0];

                let mut vbo = 0;
                unsafe { gl.GenBuffers(1, &mut vbo) };
                unsafe { gl.BindBuffer(glow::ARRAY_BUFFER, vbo) };
                unsafe {
                    gl.BufferData(
                        glow::ARRAY_BUFFER,
                        std::mem::size_of_val(&vertices) as isize,
                        vertices.as_ptr() as *const std::ffi::c_void,
                        glow::STATIC_DRAW,
                    )
                };

                // Set up vertex attributes
                let position_loc = unsafe { gl.GetAttribLocation(program, b"a_position\0".as_ptr() as *const i8) };
                let tex_coord_loc = unsafe { gl.GetAttribLocation(program, b"a_tex_coord\0".as_ptr() as *const i8) };

                unsafe { gl.EnableVertexAttribArray(position_loc as u32) };
                unsafe { gl.VertexAttribPointer(position_loc as u32, 2, glow::FLOAT, glow::FALSE, 4 * 4, std::ptr::null()) };

                unsafe { gl.EnableVertexAttribArray(tex_coord_loc as u32) };
                unsafe { gl.VertexAttribPointer(tex_coord_loc as u32, 2, glow::FLOAT, glow::FALSE, 4 * 4, (2 * 4) as *const std::ffi::c_void) };

                // Bind input texture for shader sampling
                // Query the texture ID from the input framebuffer attachment
                let input_texture_id = unsafe {
                    let mut texture_id = 0;
                    gl.GetFramebufferAttachmentParameteriv(
                        glow::READ_FRAMEBUFFER,
                        glow::COLOR_ATTACHMENT0,
                        glow::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME,
                        &mut texture_id,
                    );
                    texture_id as u32
                };

                // Bind the input texture to texture unit 0
                unsafe { gl.ActiveTexture(0x84C0) }; // GL_TEXTURE0
                unsafe { gl.BindTexture(glow::TEXTURE_2D, input_texture_id) };

                // Set texture sampler uniform to texture unit 0
                let texture_loc = unsafe { gl.GetUniformLocation(program, b"u_texture\0".as_ptr() as *const i8) };
                unsafe { gl.Uniform1i(texture_loc, 0) };

                // Set animation progress uniform
                let progress_loc = unsafe { gl.GetUniformLocation(program, b"u_progress\0".as_ptr() as *const i8) };
                unsafe { gl.Uniform1f(progress_loc, progress) };

                // Draw quad
                unsafe { gl.DrawArrays(glow::TRIANGLE_STRIP, 0, 4) };

                // Clean up
                unsafe { gl.DeleteBuffers(1, &vbo) };
                unsafe { gl.DeleteFramebuffers(1, &fbo) };
                unsafe { gl.DeleteTextures(1, &output_texture) };
                unsafe { gl.DeleteProgram(program) };

                Ok(())
            })
            .map_err(|e| OpenGLRendererError::RenderingFailed(format!("OpenGL ES rendering failed: {:?}", e)))?;

        Ok(output_result)
    }
}

impl Default for OpenGLRenderer {
    fn default() -> Self {
        Self {
            renderer: None,
            gbm_device: None,
        }
    }
}

impl Drop for OpenGLRenderer {
    fn drop(&mut self) {
        debug!("Destroying OpenGL renderer");
        // TODO: Phase 4 - Properly cleanup EGL resources
        // This requires:
        // 1. Destroy EGL context
        // 2. Terminate EGL display
    }
}

/// Errors that can occur during OpenGL renderer initialization
#[derive(Debug, thiserror::Error)]
pub enum OpenGLRendererError {
    #[error("OpenGL renderer not yet implemented")]
    NotImplemented,

    #[error("EGL initialization failed: {0}")]
    EglInitializationFailed(String),

    #[error("EGL config selection failed: {0}")]
    EglConfigSelectionFailed(String),

    #[error("EGL context creation failed: {0}")]
    EglContextCreationFailed(String),

    #[error("Smithay GlesRenderer initialization failed: {0}")]
    RendererInitializationFailed(String),

    #[error("OpenGL renderer not initialized")]
    RendererNotInitialized,

    #[error("OpenGL context not current")]
    ContextNotCurrent,

    #[error("Rendering operation failed: {0}")]
    RenderingFailed(String),
}
