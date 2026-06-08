use crate::opengl_renderer::OpenGLRenderer;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::error::CompositorInitializationError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::config::handler::ConfigHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::size::handler::WidgetSizeHandler;
use glib::ControlFlow;
use glib::object::Cast;
use glib::subclass::prelude::ObjectSubclassExt;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk4::prelude::GtkWindowExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_compositor::CalloopData;
use smearor_wrot_compositor::CommitCallbackAware;
use smearor_wrot_compositor::DmaBufAllocator;
use smearor_wrot_compositor::OutputGeometry;
use smearor_wrot_compositor::SmearorCompositor;
use smearor_wrot_compositor::WindowSizeCallbackAware;
use smearor_wrot_geometry::Size;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::warn;

impl CompositorHandler for CompositorWidgetImpl {
    fn set_compositor(&self, compositor: Option<Arc<Mutex<SmearorCompositor>>>) {
        *self.compositor.borrow_mut() = compositor;
    }

    fn compositor(&self) -> Result<Arc<Mutex<SmearorCompositor>>, CompositorError> {
        self.compositor.borrow().clone().ok_or(CompositorError::CompositorNotFound)
    }

    fn initialize_compositor(&self) -> Result<(), CompositorInitializationError> {
        if self.compositor.borrow().is_some() {
            debug!("Compositor already initialized, skipping");
            return Ok(());
        }

        let socket = self.socket.borrow().clone();

        // let Some(socket) = self.socket.borrow().as_ref() else {
        //     return Err(CompositorInitializationError::CompositorSocketNotInitialized);
        // };

        debug!("Initializing compositor with socket: {socket:?}");

        // Create Smithay event loop
        let mut event_loop = match EventLoop::try_new() {
            Ok(el) => el,
            Err(e) => {
                error!("Failed to create Smithay event loop: {}", e);
                return Err(CompositorInitializationError::FailedToCreateEventLoop);
            }
        };

        // Create Smithay display
        let display = match Display::new() {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to create Smithay display: {}", e);
                return Err(CompositorInitializationError::FailedToCreateDisplay);
            }
        };

        // Wrap display in Arc<Mutex> for sharing
        let shared_display = Arc::new(Mutex::new(display));

        // Get initial size from config
        let config = self.config();
        let initial_width = config.initial_width;
        let initial_height = config.initial_height;

        // Initialize compositor
        let dma_buf_enabled = config.dma_buf_enabled;
        let keyboard_layout = config.keyboard_layout.clone();
        let keyboard_variant = config.keyboard_variant.clone();
        let max_fps = config.max_fps;
        let compositor = match SmearorCompositor::new(
            child_process_manager,
            margin_manager,
            &mut event_loop,
            shared_display.clone(),
            socket,
            initial_width,
            initial_height,
            dma_buf_enabled,
            keyboard_layout,
            keyboard_variant,
        ) {
            Ok(c) => Arc::new(Mutex::new(c)),
            Err(e) => {
                return Err(e.into());
            }
        };

        // Store compositor
        *self.compositor.borrow_mut() = Some(compositor.clone());

        // Register window size callback for application -> compositor size coupling
        let widget = self.obj().clone();
        let callback = Arc::new(move |width: i32, height: i32| {
            debug!("Window size callback: updating compositor window to {}x{}", width, height);
            // Get the root window and resize it
            if let Some(root) = widget.root() {
                if let Some(application_window) = root.downcast_ref::<gtk4::ApplicationWindow>() {
                    application_window.set_default_size(width, height);
                }
            }
        });

        // Register commit callback for compositor for redrawing the GTK widget if a subsurface commits
        let widget_weak = self.obj().downgrade();
        let commit_cb = Arc::new(move |surface_id| {
            if let Some(widget) = widget_weak.upgrade() {
                widget.queue_draw();
            }
        });

        if let Ok(comp) = compositor.lock() {
            comp.set_window_size_callback(callback);
            comp.set_commit_callback(commit_cb);
        }

        // Initialize OpenGL renderer with surfaceless EGL and GBM device
        let gbm_device = match compositor.lock() {
            Ok(guard) => guard.dma_buf_allocator.as_ref().and_then(|allocator| allocator.gbm_device()),
            Err(_) => {
                error!("Failed to lock compositor for OpenGL renderer initialization");
                return Err(CompositorInitializationError::FailedToLockCompositor);
            }
        };

        match gbm_device {
            Some(gbm_device) => {
                match OpenGLRenderer::new_surfaceless(gbm_device) {
                    Ok(renderer) => {
                        self.set_opengl_renderer(renderer);
                        debug!("OpenGL renderer initialized successfully with surfaceless EGL and GBM device");
                    }
                    Err(e) => {
                        error!("Failed to initialize OpenGL renderer: {}", e);
                        // Continue without OpenGL renderer - will use SHM fallback
                    }
                }
            }
            None => {
                warn!("GBM device not available, skipping OpenGL renderer initialization");
                // Continue without OpenGL renderer - will use SHM fallback
            }
        }

        // Extract listening socket from compositor
        let listening_socket = match compositor.lock() {
            Ok(mut guard) => guard.listening_socket.take(),
            Err(_) => {
                error!("Failed to lock compositor");
                return Err(CompositorInitializationError::FailedToLockCompositor);
            }
        };

        let listening_socket = match listening_socket {
            Some(socket) => socket,
            None => {
                error!("Listening socket not found in compositor");
                return Err(CompositorInitializationError::FailedToGetListeningSocket);
            }
        };

        // Wrap listening socket in Arc<Mutex> for thread-safe access
        let listening_socket_shared = Arc::new(Mutex::new(listening_socket));

        // Dispatch Smithay event loop periodically in GTK main loop
        let event_loop_shared = Arc::new(Mutex::new(event_loop));
        let compositor_clone = compositor.clone();
        let display_clone = shared_display.clone();
        let listening_socket_clone = listening_socket_shared.clone();

        // Pre-compute display_handle to avoid deadlock
        let display_handle = match display_clone.lock() {
            Ok(guard) => guard.handle(),
            Err(_) => {
                error!("Failed to lock display");
                return Err(CompositorInitializationError::FailedToLockDisplay);
            }
        };
        let glib_frame_interval = Duration::from_millis(1000 / config.max_fps as u64);
        debug!("glib_frame_interval {}", glib_frame_interval.as_millis());
        glib::timeout_add_local(glib_frame_interval, move || {
            // Dispatch Smithay event loop
            if let Ok(mut event_loop) = event_loop_shared.lock() {
                if let Err(e) = event_loop.dispatch(
                    Duration::ZERO,
                    &mut CalloopData {
                        state: compositor_clone.clone(),
                        display_handle: display_handle.clone(),
                    },
                ) {
                    error!("Smithay event loop dispatch error: {}", e);
                }
            }

            // Dispatch Wayland events manually
            if let Ok(mut display) = display_clone.lock() {
                if let Ok(mut compositor) = compositor_clone.lock() {
                    if let Err(e) = display.dispatch_clients(&mut *compositor) {
                        error!("Failed to dispatch Wayland clients: {}", e);
                    }
                    if let Err(e) = display.flush_clients() {
                        error!("Failed to flush Wayland clients: {}", e);
                    }
                }
            }

            // Accept new client connections manually
            if let Ok(listening_socket) = listening_socket_clone.lock() {
                match listening_socket.accept() {
                    Ok(Some(client_stream)) => {
                        let display = match display_clone.lock() {
                            Ok(guard) => guard,
                            Err(_) => {
                                error!("Failed to lock display");
                                return ControlFlow::Continue;
                            }
                        };
                        let mut display_handle = display.handle();
                        match display_handle.insert_client(client_stream, Arc::new(smearor_wrot_compositor::state::client::ClientState::default())) {
                            Ok(_) => {
                                debug!("Client inserted successfully");
                            }
                            Err(e) => {
                                error!("Failed to insert client: {}", e);
                            }
                        }
                    }
                    Ok(None) => {
                        // No client connection available
                    }
                    Err(e) => {
                        error!("Failed to accept client connection: {}", e);
                    }
                }
            }

            ControlFlow::Continue
        });

        debug!("Smithay event loop and Wayland dispatch started in GTK main loop");

        // Start periodic size polling to detect window resize events
        // size_allocate is not always triggered, so we poll for size changes
        let widget_weak = self.obj().downgrade();
        let mut last_size = Size::new(0, 0);
        glib::timeout_add_local(glib_frame_interval, move || {
            let Some(widget) = widget_weak.upgrade() else {
                return ControlFlow::Break;
            };
            let size = widget.widget_size();
            if size.width > 0 && size.height > 0 && (size.width != last_size.width || size.height != last_size.height) {
                debug!("Size polling detected resize: {last_size} -> {size}");
                last_size = size;
                // Call update_output_size directly without debounce
                if let Some(compositor_mutex) = widget.imp().compositor.borrow().clone() {
                    if let Ok(mut compositor) = compositor_mutex.lock() {
                        debug!("Sending configure events to application for size {size}");
                        compositor.update_output_size(size);
                    }
                }
                widget.request_render();
            }
            ControlFlow::Continue
        });
        debug!("Size polling timeout started");

        // // Trigger a redraw after compositor initialization
        // let widget_weak = self.obj().downgrade();
        // glib::idle_add_local_once(move || {
        //     debug!("Idle callback: triggering redraw after compositor init");
        //     if let Some(widget) = widget_weak.upgrade() {
        //         widget.queue_draw();
        //     }
        // });

        Ok(())
    }
}
