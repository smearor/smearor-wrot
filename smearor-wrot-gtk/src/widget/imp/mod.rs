use super::*;
use crate::color_mask::color_mask_applier::dma_buf::DmaBufColorMaskApplier;
use crate::opengl_renderer::OpenGLRenderer;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::config::handler::ConfigHandler;
use crate::widget::imp::dmabuf::formats::DmabufFormats;
use crate::widget::imp::event::touch::TouchEventSetup;
use crate::widget::resize::handler::ResizeHandler;
use dashmap::DashMap;
use dashmap::DashSet;
use glib::ControlFlow;
use glib::object::ObjectExt;
use gtk4::graphene::Rect;
use gtk4::prelude::SnapshotExt;
use gtk4::prelude::WidgetExtManual;
use smearor_wrot_core::damage::surface::SurfaceDamage;
use smearor_wrot_core::frame::count::FrameCounter;
use smearor_wrot_core::frame::limit::FrameLimiter;
use smearor_wrot_model::Socket;
use smithay::backend::allocator::Fourcc;
use smithay::backend::allocator::dmabuf::Dmabuf;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tracing::debug;
use tracing::error;
use tracing::warn;

pub(crate) mod buffer;
pub(crate) mod clipboard;
pub(crate) mod color_mask;
pub(crate) mod commit;
pub(crate) mod compositor;
pub(crate) mod config;
pub(crate) mod dmabuf;
pub(crate) mod event;
pub(crate) mod focus_synchronization;
pub(crate) mod header_bar;
pub(crate) mod holding_area;
pub(crate) mod render;
pub(crate) mod render_node;
pub(crate) mod resize;
pub(crate) mod shm;
pub(crate) mod shutdown;
pub(crate) mod size;
pub(crate) mod snapshot;
pub(crate) mod socket;
pub(crate) mod window_state;

#[derive(Clone, Debug)]
pub struct TouchPoint {
    pub gtk_x: f64,
    pub gtk_y: f64,
    pub app_x: f64,
    pub app_y: f64,
}

#[derive(Clone, Debug)]
pub struct PointerPoint {
    pub gtk_x: f64,
    pub gtk_y: f64,
    pub app_x: f64,
    pub app_y: f64,
}

#[derive(Clone, Debug)]
pub enum ApplicationError {
    NotFound(String),
    NotSpecified,
}

pub struct CompositorWidgetImpl {
    compositor: RefCell<Option<Arc<Mutex<SmearorCompositor>>>>,
    config: Mutex<CompositorWidgetConfig>,
    header_bar: RefCell<Option<gtk4::HeaderBar>>,
    header_bar_title_label: RefCell<Option<gtk4::Label>>,
    pending_resize: RefCell<Option<Size<i32>>>,
    resize_timeout: RefCell<Option<glib::SourceId>>,
    socket: RefCell<Option<Socket>>,
    auto_resize_handling: RefCell<bool>,
    touch_transform_callback: RefCell<Option<Box<dyn Fn(usize, f64, f64) -> (f64, f64) + 'static>>>,
    pointer_transform_callback: RefCell<Option<Box<dyn Fn(f64, f64) -> (f64, f64) + 'static>>>,
    last_render_time: Arc<AtomicI64>,
    opengl_renderer: RefCell<Option<OpenGLRenderer>>,
    dmabuf_registry: DashMap<gtk4::gdk::Texture, Dmabuf>,
    supported_gtk_formats: DashSet<(Fourcc, u64)>,
    touch_points: DashMap<usize, TouchPoint>,
    pointer_point: RefCell<Option<PointerPoint>>,
    application_error: RefCell<Option<ApplicationError>>,
    color_mask_shader: RefCell<Option<gtk4::gsk::GLShader>>,
    dma_buf_color_mask_applier: RefCell<Option<DmaBufColorMaskApplier>>,
    open_gl_masked_texture_cache: DashMap<smithay::reexports::wayland_server::backend::ObjectId, (u32, gtk4::gdk::Texture)>,
    input_blocked: Arc<AtomicBool>,
}

impl Default for CompositorWidgetImpl {
    fn default() -> Self {
        Self {
            compositor: RefCell::new(None),
            config: Mutex::new(CompositorWidgetConfig::default()),
            header_bar: RefCell::new(None),
            header_bar_title_label: RefCell::new(None),
            pending_resize: RefCell::new(None),
            resize_timeout: RefCell::new(None),
            socket: RefCell::new(None),
            auto_resize_handling: RefCell::new(true),
            touch_transform_callback: RefCell::new(None),
            pointer_transform_callback: RefCell::new(None),
            last_render_time: Arc::new(AtomicI64::new(0)),
            opengl_renderer: RefCell::new(None),
            dmabuf_registry: DashMap::new(),
            supported_gtk_formats: DashSet::new(),
            touch_points: DashMap::new(),
            pointer_point: RefCell::new(None),
            application_error: RefCell::new(None),
            color_mask_shader: RefCell::new(None),
            dma_buf_color_mask_applier: RefCell::new(None),
            open_gl_masked_texture_cache: DashMap::new(),
            input_blocked: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CompositorWidgetImpl {
    const NAME: &'static str = "CompositorWidget";
    type Type = CompositorWidget;
    type ParentType = gtk4::Widget;
}

impl ObjectImpl for CompositorWidgetImpl {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        self.setup_widget_config(&obj);
        self.setup_header_bar(&obj);
        self.setup_mouse_events(&obj);
        self.setup_touch_events(&obj);
        self.setup_keyboard_events(&obj);
        self.setup_focus_synchronization(&obj);
        self.setup_tick_callback(&obj);
    }

    fn dispose(&self) {
        if let Some(timeout_id) = self.resize_timeout.borrow_mut().take() {
            timeout_id.remove();
        }
    }
}

impl CompositorWidgetImpl {
    /// Set the OpenGL renderer for DMA-BUF hardware acceleration
    ///
    /// TODO: Phase 7 - DMA-BUF support for hardware acceleration - Initialize renderer
    /// This requires:
    /// 1. Get Wayland display pointer from compositor
    /// 2. Initialize OpenGL renderer with display pointer
    /// 3. Set renderer in widget
    pub fn set_opengl_renderer(&self, renderer: OpenGLRenderer) {
        // Renderer-Formate abrufen
        if let Some(renderer_formats) = renderer.dmabuf_formats() {
            debug!("Retrieved DMA-BUF formats from renderer");

            // Renderer-Formate in DashSet konvertieren
            let renderer_formats_set: DashSet<(Fourcc, u64)> = renderer_formats.iter().map(|format| (format.code, format.modifier.into())).collect();

            let display = gtk4::gdk::Display::default();
            if let Some(display) = display {
                let compatible_formats = self.get_compatible_dmabuf_formats(&display, renderer_formats_set);
                self.set_supported_gtk_formats(compatible_formats);
            } else {
                warn!("No default GDK display available, skipping format compatibility check");
            }
        } else {
            warn!("OpenGL renderer does not provide DMA-BUF formats, skipping format compatibility check");
        }

        // Initialize DMA-BUF global with renderer formats
        if let Some(compositor) = self.compositor.borrow().as_ref() {
            if let Ok(mut comp) = compositor.lock() {
                if let Some(renderer_formats) = renderer.dmabuf_formats() {
                    match comp.init_dmabuf_global(renderer_formats) {
                        Ok(_) => {
                            debug!("DMA-BUF global initialized successfully");
                        }
                        Err(e) => {
                            error!("Failed to initialize DMA-BUF global: {e}");
                        }
                    }
                }
            }
        }

        self.opengl_renderer.replace(Some(renderer));
        debug!("OpenGL renderer set for DMA-BUF hardware acceleration");
    }

    /// Get the OpenGL renderer if available
    pub fn opengl_renderer(&self) -> Ref<Option<OpenGLRenderer>> {
        self.opengl_renderer.borrow()
    }

    /// Get the OpenGL renderer if available
    pub fn opengl_renderer_mut(&self) -> RefMut<Option<OpenGLRenderer>> {
        self.opengl_renderer.borrow_mut()
    }

    /// Get or create the color mask shader
    ///
    /// This creates the GLSL shader for color masking if it hasn't been created yet.
    #[allow(deprecated)]
    fn get_color_mask_shader(&self) -> Option<gtk4::gsk::GLShader> {
        if let Some(shader) = self.color_mask_shader.borrow().as_ref() {
            return Some(shader.clone());
        }

        // Define shader source directly
        let shader_source = r#"
uniform float threshold;
uniform vec3 mask_color;

void main() {
    vec4 tex_color = GskTexture(u_texture[0], gsk_get_tex_coord(0));
    float d = distance(tex_color.rgb, mask_color);
    if (d < threshold) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    } else {
        gl_FragColor = tex_color;
    }
}
"#;

        #[allow(deprecated)]
        let shader = gtk4::gsk::GLShader::from_bytes(&glib::Bytes::from(shader_source.as_bytes()));
        self.color_mask_shader.replace(Some(shader.clone()));
        Some(shader)
    }

    /// Get cached masked texture for a surface if commit count matches
    pub fn get_cached_masked_texture(
        &self,
        surface_id: &smithay::reexports::wayland_server::backend::ObjectId,
        commit_count: u32,
    ) -> Option<gtk4::gdk::Texture> {
        self.open_gl_masked_texture_cache.get(surface_id).and_then(|v| {
            let (cached_commit_count, texture) = v.value();
            if *cached_commit_count == commit_count { Some(texture.clone()) } else { None }
        })
    }

    /// Cache masked texture for a surface with commit count
    pub fn cache_masked_texture(&self, surface_id: smithay::reexports::wayland_server::backend::ObjectId, commit_count: u32, texture: gtk4::gdk::Texture) {
        self.open_gl_masked_texture_cache.insert(surface_id, (commit_count, texture));
    }

    /// Register a DMA-BUF buffer with a GDK texture
    ///
    /// This ensures the DMA-BUF file descriptors remain valid as long as the texture exists.
    pub fn register_dmabuf_texture(&self, texture: gtk4::gdk::Texture, dmabuf: Dmabuf) {
        self.dmabuf_registry.insert(texture, dmabuf);
    }

    /// Unregister a DMA-BUF buffer when the texture is destroyed
    pub fn unregister_dmabuf_texture(&self, texture: &gtk4::gdk::Texture) {
        self.dmabuf_registry.remove(texture);
    }

    /// Cleanup destroyed DMA-BUF textures from the registry
    ///
    /// This function removes textures that are no longer referenced (ref count = 1, only the registry holds a reference).
    /// This is called periodically to prevent memory leaks from destroyed textures.
    pub fn cleanup_dmabuf_registry(&self) {
        let textures_to_remove: Vec<gtk4::gdk::Texture> = self
            .dmabuf_registry
            .iter()
            .filter(|entry| entry.key().ref_count() == 1)
            .map(|entry| entry.key().clone())
            .collect();

        for texture in &textures_to_remove {
            debug!("Removing destroyed DMA-BUF texture from registry");
            self.dmabuf_registry.remove(texture);
        }

        if !textures_to_remove.is_empty() {
            debug!("Cleaned up {} destroyed DMA-BUF textures from registry", textures_to_remove.len());
        }
    }

    pub fn trigger_buffer_snapshot(&self, surface_id: smithay::reexports::wayland_server::backend::ObjectId) {
        debug!("trigger_buffer_snapshot called for surface: {:?}", surface_id);
        let compositor = self.compositor.borrow();
        let Some(compositor) = compositor.as_ref() else {
            debug!("No compositor available for buffer snapshot");
            return;
        };
        let Ok(compositor) = compositor.lock() else {
            debug!("Failed to lock compositor for buffer snapshot");
            return;
        };

        // Try to get buffer from holding area and render it to texture immediately
        if let Ok(holding_area) = compositor.buffer_holding_area.lock() {
            if let Some(buffer) = holding_area.get(&surface_id) {
                debug!("Found buffer in holding area for snapshot: {:?}", surface_id);
                // Render buffer to texture immediately to create persistent copy
                if let Some(texture) = crate::widget::imp::holding_area::BufferHoldingArea::render_buffer_from_holding_area(self, &compositor, &surface_id) {
                    debug!("Successfully created buffer snapshot texture for surface: {:?}", surface_id);
                    // Store texture in a separate snapshot cache to prevent release
                    // This ensures the texture persists even after wl_buffer is released
                } else {
                    debug!("Failed to create buffer snapshot texture for surface: {:?}", surface_id);
                }
            } else {
                debug!("No buffer found in holding area for snapshot: {:?}", surface_id);
            }
        }

        // Trigger force render immediately after snapshot
        debug!("Triggering force render after buffer snapshot");
        drop(compositor);
        self.request_render_force();
    }

    fn setup_tick_callback(&self, widget: &CompositorWidget) {
        let widget_weak = widget.downgrade();
        widget.add_tick_callback(move |widget, frame_clock| {
            let Some(widget) = widget_weak.upgrade() else {
                return ControlFlow::Continue;
            };
            let Ok(compositor) = widget.compositor() else {
                return ControlFlow::Continue;
            };
            let Ok(compositor) = compositor.lock() else {
                return ControlFlow::Continue;
            };
            match compositor.frame_rate_limit() {
                Some(frame_rate_limit_ms) => {
                    let frame_duration = frame_rate_limit_ms; // ms
                    let now_millis = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
                    let last_render_time = widget.imp().last_render_time.load(Ordering::Relaxed);
                    if (now_millis - last_render_time) >= frame_duration {
                        widget.imp().last_render_time.store(now_millis, Ordering::Relaxed);
                        let all_damage = compositor.get_all_surface_damage();
                        if !all_damage.is_empty() {
                            widget.queue_draw();
                            debug!("tick_callback: Queueing draw with {} damage regions", all_damage.len());
                            compositor.increment_frame_count();
                        }
                    }
                }
                None => {
                    let all_damage = compositor.get_all_surface_damage();
                    if !all_damage.is_empty() {
                        widget.queue_draw();
                    }
                }
            }
            ControlFlow::Continue
        });
    }
}

impl WidgetImpl for CompositorWidgetImpl {
    fn measure(&self, orientation: gtk4::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
        let compositor_option = self.compositor.borrow();
        let Some(compositor_rc) = compositor_option.as_ref() else {
            return (0, 0, -1, -1);
        };
        let Ok(compositor) = compositor_rc.lock() else {
            return (0, 0, -1, -1);
        };
        let Some(window) = compositor.space.elements().next() else {
            return (0, 0, -1, -1);
        };
        let Some(geometry) = compositor.space.element_geometry(window) else {
            return (0, 0, -1, -1);
        };

        let min_size = self.min_size_by_orientation(orientation);

        let window_size = if orientation == gtk4::Orientation::Horizontal {
            // geometry.size.w
            geometry.size.w.max(min_size)
        } else {
            // geometry.size.h
            geometry.size.h.max(min_size)
        };
        (min_size, window_size, -1, -1)
    }

    fn realize(&self) {
        self.parent_realize();
        self.obj().set_focusable(true);
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        // Call parent implementation
        self.parent_size_allocate(width, height, baseline);

        // Handle resize event when actual widget size changes
        // Only if auto_resize_handling is enabled
        if *self.auto_resize_handling.borrow() {
            let widget = self.obj();
            widget.handle_resize(Size::new(width, height));
            // widget.queue_resize();
        }
    }

    fn snapshot(&self, snapshot: &gtk4::Snapshot) {
        self.render_snapshot(snapshot);

        // Draw touch points for visual debugging only if debug_touch is enabled
        let config = self.config.lock();
        let Ok(config) = config.as_ref() else {
            return;
        };

        if config.debug_touch {
            for touch_point in self.touch_points.iter() {
                // Draw red filled rectangle for GTK coordinates
                let gtk_color = gtk4::gdk::RGBA::new(1.0, 0.0, 0.0, 1.0);
                let gtk_bounds = Rect::new(touch_point.gtk_x as f32 - 20.0, touch_point.gtk_y as f32 - 20.0, 40.0, 40.0);
                snapshot.append_color(&gtk_color, &gtk_bounds);

                // Draw green unfilled rectangle (border only) for app coordinates
                let app_color = gtk4::gdk::RGBA::new(0.0, 1.0, 0.0, 1.0);
                let app_bounds = Rect::new(touch_point.app_x as f32 - 20.0, touch_point.app_y as f32 - 20.0, 40.0, 40.0);
                let border_width = 2.0;

                // Top border
                let top_bounds = Rect::new(app_bounds.x(), app_bounds.y(), app_bounds.width(), border_width);
                snapshot.append_color(&app_color, &top_bounds);

                // Bottom border
                let bottom_bounds = Rect::new(app_bounds.x(), app_bounds.y() + app_bounds.height() - border_width, app_bounds.width(), border_width);
                snapshot.append_color(&app_color, &bottom_bounds);

                // Left border
                let left_bounds = Rect::new(app_bounds.x(), app_bounds.y(), border_width, app_bounds.height());
                snapshot.append_color(&app_color, &left_bounds);

                // Right border
                let right_bounds = Rect::new(app_bounds.x() + app_bounds.width() - border_width, app_bounds.y(), border_width, app_bounds.height());
                snapshot.append_color(&app_color, &right_bounds);
            }
        }

        // Draw pointer for visual debugging only if debug_pointer is enabled
        if config.debug_pointer {
            let pointer_point = self.pointer_point.borrow();
            if let Some(pointer) = pointer_point.as_ref() {
                // Draw blue filled rectangle for GTK coordinates
                let gtk_color = gtk4::gdk::RGBA::new(0.0, 0.0, 1.0, 1.0);
                let gtk_bounds = Rect::new(pointer.gtk_x as f32 - 20.0, pointer.gtk_y as f32 - 20.0, 40.0, 40.0);
                snapshot.append_color(&gtk_color, &gtk_bounds);

                // Draw magenta unfilled rectangle (border only) for app coordinates
                let app_color = gtk4::gdk::RGBA::new(1.0, 0.0, 1.0, 1.0);
                let app_bounds = Rect::new(pointer.app_x as f32 - 20.0, pointer.app_y as f32 - 20.0, 40.0, 40.0);
                let border_width = 2.0;

                // Top border
                let top_bounds = Rect::new(app_bounds.x(), app_bounds.y(), app_bounds.width(), border_width);
                snapshot.append_color(&app_color, &top_bounds);

                // Bottom border
                let bottom_bounds = Rect::new(app_bounds.x(), app_bounds.y() + app_bounds.height() - border_width, app_bounds.width(), border_width);
                snapshot.append_color(&app_color, &bottom_bounds);

                // Left border
                let left_bounds = Rect::new(app_bounds.x(), app_bounds.y(), border_width, app_bounds.height());
                snapshot.append_color(&app_color, &left_bounds);

                // Right border
                let right_bounds = Rect::new(app_bounds.x() + app_bounds.width() - border_width, app_bounds.y(), border_width, app_bounds.height());
                snapshot.append_color(&app_color, &right_bounds);
            }
        }
        let Ok(compositor) = self.compositor() else {
            return;
        };
        let Ok(compositor) = compositor.lock() else {
            return;
        };
        compositor.send_pending_frame_callbacks();
    }
}

impl CompositorWidgetImpl {
    pub fn set_auto_resize_handling(&self, enabled: bool) {
        *self.auto_resize_handling.borrow_mut() = enabled;
    }

    pub fn set_touch_transform_callback<F>(&self, callback: F)
    where
        F: Fn(usize, f64, f64) -> (f64, f64) + 'static,
    {
        *self.touch_transform_callback.borrow_mut() = Some(Box::new(callback));
    }

    pub fn set_pointer_transform_callback<F>(&self, callback: F)
    where
        F: Fn(f64, f64) -> (f64, f64) + 'static,
    {
        *self.pointer_transform_callback.borrow_mut() = Some(Box::new(callback));
    }

    pub fn apply_touch_transform(&self, sequence: usize, x: f64, y: f64) -> (f64, f64) {
        if let Some(callback) = self.touch_transform_callback.borrow().as_ref() {
            callback(sequence, x, y)
        } else {
            (x, y)
        }
    }

    pub fn apply_pointer_transform(&self, x: f64, y: f64) -> (f64, f64) {
        if let Some(callback) = self.pointer_transform_callback.borrow().as_ref() {
            callback(x, y)
        } else {
            (x, y)
        }
    }

    pub fn show_touch_overlay(&self) {
        // Touch points are always rendered in snapshot when available
    }

    pub fn hide_touch_overlay(&self) {
        // Clear all touch points
        self.touch_points.clear();
    }

    pub fn update_touch_point(&self, sequence: usize, gtk_x: f64, gtk_y: f64, app_x: f64, app_y: f64) {
        let touch_point = TouchPoint { gtk_x, gtk_y, app_x, app_y };
        self.touch_points.insert(sequence, touch_point);
    }

    pub fn remove_touch_point(&self, sequence: usize) {
        self.touch_points.remove(&sequence);
    }

    pub fn update_pointer_point(&self, gtk_x: f64, gtk_y: f64, app_x: f64, app_y: f64) {
        let pointer_point = PointerPoint { gtk_x, gtk_y, app_x, app_y };
        self.pointer_point.borrow_mut().replace(pointer_point);
    }

    pub fn clear_pointer_point(&self) {
        self.pointer_point.borrow_mut().take();
    }

    pub fn set_application_error(&self, error: Option<ApplicationError>) {
        *self.application_error.borrow_mut() = error;
        self.obj().queue_draw();
    }

    pub fn application_error(&self) -> Option<ApplicationError> {
        self.application_error.borrow().clone()
    }
}
