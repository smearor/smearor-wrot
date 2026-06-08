//! Wayland compositor implementation based on smallvil
//!
//! This module provides the core compositor functionality using Smithay.

use std::sync::Arc;
use std::sync::Mutex;

use crate::callback::commit::CommitCallback;
use crate::callback::window_size::WindowSizeCallback;
use crate::color_mask::toplevel::TopLevelColorMask;
use crate::commit::count::CommitCount;
use crate::dma::allocator::DmaBufAllocator;
use crate::dma::allocator::DmaBufAllocatorImpl;
use crate::dma::format::DmabufFormatProvider;
use crate::error::CoreError;
use crate::error::Result;
use crate::input::touch::slot_manager::ThreadSafeTouchSlotManager;
use crate::message::compositor_message::CompositorMessage;
use crate::subsurface::model::SubsurfaceData;
use crate::texture::cache::TextureCacheEntry;
use crate::texture::pixel_data::BGRA;
use dashmap::DashMap;
use dashmap::DashSet;
use smearor_wrot_child_process::ChildProcessManager;
use smearor_wrot_color::RgbaColor;
use smearor_wrot_color_mask::ColorMask;
use smearor_wrot_margin::MarginManager;
use smithay::backend::allocator::format::FormatSet;
use smithay::desktop::PopupManager;
use smithay::desktop::Space;
use smithay::desktop::Window;
use smithay::input::Seat;
use smithay::input::SeatState;
use smithay::input::keyboard::XkbConfig;
use smithay::output::Output;
use smithay::output::PhysicalProperties;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::calloop::LoopSignal;
use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::wayland_server::DisplayHandle;
use smithay::reexports::wayland_server::ListeningSocket;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Logical;
use smithay::utils::Physical;
use smithay::utils::Rectangle;
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::dmabuf;
use smithay::wayland::dmabuf::DmabufFeedbackBuilder;
use smithay::wayland::dmabuf::DmabufState;
use smithay::wayland::output::OutputManagerState;
use smithay::wayland::selection::SelectionSource;
use smithay::wayland::selection::data_device::DataDeviceState;
use smithay::wayland::shell::xdg::ToplevelSurface;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::wayland::shell::xdg::decoration::XdgDecorationState;
use smithay::wayland::shell::xdg::dialog::XdgDialogState;
use smithay::wayland::shm::ShmState;
use smithay::wayland::viewporter::ViewporterState;
use std::os::unix::io::RawFd;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::AtomicU32;
use std::sync::mpsc::Sender;
use std::time::Instant;
use tracing::debug;
use tracing::info;

pub struct SmearorCompositorStates {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub xdg_decoration_state: XdgDecorationState,
    pub xdg_dialog_state: XdgDialogState,
    pub viewporter_state: ViewporterState,
    pub shm_state: ShmState,
    pub output_manager_state: OutputManagerState,
    pub seat_state: SeatState<SmearorCompositor>,
    pub data_device_state: DataDeviceState,
    pub popups: PopupManager,
    pub seat: Seat<SmearorCompositor>,
}

/// Compositor state for smearor-wrot
pub struct SmearorCompositor {
    pub child_process_manager: Arc<ChildProcessManager>,
    pub margin_manager: Arc<MarginManager>,
    pub keyboard_manager: Arc<KeyboardManager>,

    pub start_time: Instant,
    // pub socket_name: OsString,
    pub display_handle: DisplayHandle,
    pub display: Arc<Mutex<Display<SmearorCompositor>>>,

    pub space: Space<Window>,
    pub loop_signal: LoopSignal,

    // Smithay State
    pub states: SmearorCompositorStates,
    // pub compositor_state: CompositorState,
    // pub xdg_shell_state: XdgShellState,
    // pub xdg_decoration_state: XdgDecorationState,
    // pub xdg_dialog_state: XdgDialogState,
    // pub viewporter_state: ViewporterState,
    // pub shm_state: ShmState,
    // pub output_manager_state: OutputManagerState,
    // pub seat_state: SeatState<SmearorCompositor>,
    // pub data_device_state: DataDeviceState,
    // pub popups: PopupManager,
    // pub seat: Seat<SmearorCompositor>,

    // DMA-BUF state
    pub dma_buf_state: Option<dmabuf::DmabufState>,
    pub dma_buf_global: Option<dmabuf::DmabufGlobal>,
    pub dma_buf_feedback: Option<dmabuf::DmabufFeedback>,

    // DMA-BUF allocator
    pub dma_buf_allocator: Option<DmaBufAllocatorImpl>,

    // Touch slot manager for GTK EventSequence to Smithay TouchSlot conversion
    pub touch_slot_manager: ThreadSafeTouchSlotManager,

    // Virtual output for rendering
    pub virtual_output: Option<Output>,

    // Buffer management
    pub buffers_in_use: Arc<DashSet<ObjectId>>,
    pub damage_regions: Arc<DashMap<ObjectId, Rectangle<i32, Logical>>>,
    pub surface_buffers: Arc<DashMap<ObjectId, ObjectId>>,

    // Texture cache for rendering - stores (width, height, stride, data)
    pub texture_cache: Arc<DashMap<ObjectId, TextureCacheEntry<BGRA>>>,

    // Buffer holding area - stores hard references to WlBuffer objects
    // This prevents Smithay from releasing the buffer before GTK can render it
    pub buffer_holding_area: Arc<Mutex<std::collections::HashMap<ObjectId, WlBuffer>>>,

    // Subsurface registry for tracking subsurfaces
    pub subsurfaces: Arc<Mutex<Vec<SubsurfaceData>>>,

    // Dialog registry for tracking modal dialogs
    pub dialogs: Arc<Mutex<Vec<ToplevelSurface>>>,

    // Dialog configure sizes for tracking dialog sizes from configure events
    pub dialog_configure_sizes: Arc<DashMap<WlSurface, (i32, i32)>>,

    // Frame rate limiting
    pub frame_rate_limit_ms: Arc<AtomicI64>,
    pub last_frame_time: Arc<AtomicI64>,

    // Rendering state
    pub rendered_surfaces: Arc<DashSet<ObjectId>>,
    pub rendered_outputs: Arc<DashSet<String>>,
    pub last_render_times: Arc<DashMap<String, Instant>>,
    pub output_damage: Arc<DashMap<String, Vec<Rectangle<i32, Logical>>>>,
    pub surface_damage: Arc<DashMap<WlSurface, Vec<Rectangle<i32, Physical>>>>,
    pub render_cache: Arc<DashMap<ObjectId, Vec<u8>>>,

    // Window size callback for application -> compositor size coupling
    pub window_size_callback: Arc<Mutex<Option<WindowSizeCallback>>>,

    // Commit callback for notifying GTK widget when a surface commits
    pub commit_callback: Arc<Mutex<Option<CommitCallback>>>,

    // Pending frame callbacks to send after GTK renders frame
    pub pending_frame_callbacks: Arc<Mutex<Vec<(WlSurface, std::time::Duration)>>>,

    // Channel sender for sending messages to GTK wrapper
    pub message_sender: Arc<Mutex<Option<Sender<CompositorMessage>>>>,

    // Flag to track if the compositor has ever had a surface
    pub has_had_surface: Arc<Mutex<bool>>,
    // Flag to track if the first commit has been received (to show window)
    pub first_commit_received: Arc<AtomicBool>,

    // Double buffering
    pub double_buffer_enabled: Arc<AtomicBool>,
    pub front_buffer: Arc<Mutex<Option<Vec<u8>>>>,
    pub back_buffer: Arc<Mutex<Option<Vec<u8>>>>,

    // DMA-BUF hardware acceleration
    pub dma_buf_enabled: Arc<AtomicBool>,

    // Render path tracking for debugging
    pub dma_buf_render_count: Arc<AtomicU32>,
    pub shm_render_count: Arc<AtomicU32>,

    // Client-side decorations control
    pub client_decorations_enabled: Arc<AtomicBool>,

    // Background control for rendering
    pub opacity: Arc<Mutex<f32>>,
    pub background_color: Arc<Mutex<Option<RgbaColor>>>,
    pub subsurface_background_color: Arc<Mutex<Option<RgbaColor>>>,
    pub color_mask: Arc<Mutex<Option<ColorMask>>>,
    pub auto_color_mask: Arc<AtomicBool>,
    pub color_mask_shader: Arc<AtomicBool>,
    pub color_mask_detected: Arc<AtomicBool>,
    pub color_mask_tolerance: Arc<Mutex<f32>>,
    pub subsurface_color_mask: Arc<Mutex<Option<ColorMask>>>,
    pub auto_subsurface_color_mask: Arc<AtomicBool>,
    pub subsurface_color_mask_detected: Arc<AtomicBool>,
    pub frame_count: Arc<AtomicU32>,
    pub commit_counts: Arc<DashMap<ObjectId, Arc<AtomicU32>>>,
    pub first_commit_times: Arc<DashMap<ObjectId, Arc<Mutex<Instant>>>>,
    pub last_window_sizes: Arc<DashMap<ObjectId, Arc<Mutex<Option<(i32, i32)>>>>>,

    // Clipboard state for bidirectional synchronization
    pub clipboard_content: Arc<Mutex<Option<String>>>,
    // TODO: Phase 1 - SelectionHandler - Store SelectionSource for Wayland→Host sync
    pub clipboard_source: Arc<Mutex<Option<SelectionSource>>>,
    // TODO: Phase 1 - SelectionHandler - Store pipe read end for async text extraction
    pub clipboard_pipe_read_end: Arc<Mutex<Option<RawFd>>>,
}

impl SmearorCompositor {
    pub fn new(
        child_process_manager: Arc<ChildProcessManager>,
        margin_manager: Arc<MarginManager>,
        // DONE
        // socket: Option<Socket>,
        // TODO
        event_loop: &mut EventLoop<CalloopData>,
        display: Arc<Mutex<Display<Self>>>,
        initial_width: i32,
        initial_height: i32,
        dma_buf_enabled: bool,
        keyboard_layout: Option<String>,
        keyboard_variant: Option<String>,
    ) -> Result<Self> {
        let start_time = Instant::now();

        // Lock the display to extract the handle
        let display_lock = display.lock().map_err(|e| CoreError::compositor(format!("Failed to lock display: {}", e)))?;
        let dh = display_lock.handle();
        drop(display_lock);

        let compositor_state = CompositorState::new::<Self>(&dh);
        let xdg_shell_state = XdgShellState::new::<Self>(&dh);
        let xdg_decoration_state = XdgDecorationState::new::<Self>(&dh);
        let xdg_dialog_state = XdgDialogState::new::<Self>(&dh);
        let viewporter_state = ViewporterState::new::<Self>(&dh);

        // The delegate_xdg_shell macro should handle this, but we need to verify
        // Add logging to confirm global creation
        debug!("XDG-Shell state initialized, checking global advertisement");
        debug!("Viewporter state initialized for wp_viewporter protocol");

        let shm_state = ShmState::new::<Self>(&dh, vec![smithay::reexports::wayland_server::protocol::wl_shm::Format::Argb8888]);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
        let mut seat_state = SeatState::new();
        let data_device_state = DataDeviceState::new::<Self>(&dh);
        let popups = PopupManager::default();

        // Initialize DMA-BUF state (always initialize, will be conditionally used)
        let mut dmabuf_state = DmabufState::new();

        // Initialize DMA-BUF allocator with DRM device if available
        let dma_buf_allocator = DmaBufAllocatorImpl::initialize_dma_buf_allocator();

        // Initialize DMA-BUF global with common formats (only if DMA-BUF is enabled)
        debug!("Starting DMA-BUF global initialization (dma_buf_enabled={})", dma_buf_enabled);
        let dmabuf_global = if dma_buf_enabled {
            debug!("Initializing DMA-BUF global with common formats");

            // Create Vec<Format> with common DMA-BUF formats
            let formats = DmabufFormatProvider::get_dma_buf_formats();

            debug!("Created DMA-BUF formats: {} formats", formats.len());

            // Get the DRM device from the DMA-BUF allocator
            debug!("Getting DRM device from DMA-BUF allocator");
            let main_device = dma_buf_allocator.drm_node().map(|node: &smithay::backend::drm::DrmNode| node.dev_id());

            debug!("DRM device result: {:?}", main_device);

            if let Some(main_device) = main_device {
                debug!("Using DRM device for DMA-BUF: dev_id={}", main_device);

                // Create DmabufFeedbackBuilder with device and formats
                let feedback = DmabufFeedbackBuilder::new(main_device, formats)
                    .build()
                    .map_err(|e| CoreError::compositor(format!("Failed to build DMA-BUF feedback: {}", e)))?;

                debug!("DMA-BUF feedback created successfully");

                // Create DmabufGlobal with default feedback
                let global = dmabuf_state.create_global_with_default_feedback::<Self>(&dh, &feedback);
                debug!("DMA-BUF global created successfully");
                Some(global)
            } else {
                debug!("No DRM device available for DMA-BUF, skipping global initialization");
                None
            }
        } else {
            debug!("DMA-BUF is disabled, skipping global initialization");
            None
        };
        debug!("DMA-BUF global initialization completed");

        // A seat is a group of keyboards, pointer and touch devices.
        // A seat typically has a pointer and maintains a keyboard focus and a pointer focus.
        let mut seat: Seat<Self> = seat_state.new_wl_seat(&dh, "winit");

        // Configure keyboard layout if provided
        let xkb_config = if keyboard_layout.is_some() || keyboard_variant.is_some() {
            let layout_ref = keyboard_layout.as_deref().unwrap_or("");
            let variant_ref = keyboard_variant.as_deref().unwrap_or("");
            debug!("Configuring keyboard with layout: {} variant: {}", layout_ref, variant_ref);
            XkbConfig {
                layout: layout_ref,
                variant: variant_ref,
                ..Default::default()
            }
        } else {
            debug!("Using default keyboard configuration");
            XkbConfig::default()
        };

        // Notify clients that we have a keyboard, for the sake of the example we assume that keyboard is always present.
        // You may want to track keyboard hot-plug in real compositor.
        seat.add_keyboard(xkb_config, 200, 25)
            .map_err(|e| CoreError::compositor(format!("Failed to add keyboard: {}", e)))?;

        // Notify clients that we have a pointer (mouse)
        // Here we assume that there is always pointer plugged in
        seat.add_pointer();

        // Notify clients that we have a touch device
        seat.add_touch();

        // A space represents a two-dimensional plane. Windows and Outputs can be mapped onto it.
        //
        // Windows get a position and stacking order through mapping.
        // Outputs become views of a part of the Space and can be rendered via Space::render_output.
        let mut space = Space::default();

        // Create a virtual output so Smithay can track surface visibility and send frame callbacks.
        let virtual_output = Output::new(
            "Smearor-Virtual".to_string(),
            PhysicalProperties {
                size: (0, 0).into(),
                subpixel: smithay::output::Subpixel::Unknown,
                make: "Smearor".to_string(),
                model: "Virtual Output".to_string(),
            },
        );

        // Advertise the output as a global to clients
        let _output_global = virtual_output.create_global::<SmearorCompositor>(&dh);

        let initial_display_mode = smithay::output::Mode {
            size: (initial_width, initial_height).into(),
            refresh: 60_000,
        };

        virtual_output.change_current_state(Some(initial_display_mode), None, None, Some((0, 0).into()));
        virtual_output.set_preferred(initial_display_mode);
        space.map_output(&virtual_output, (0, 0));

        // let (socket_name, listening_socket) = Self::init_wayland_listener(display.clone(), event_loop, socket)?;

        // Get the loop signal, used to stop the event loop
        let loop_signal = event_loop.get_signal();

        Ok(Self {
            child_process_manager,
            margin_manager,

            start_time,
            display_handle: dh,
            display,

            space,
            loop_signal,
            // socket_name,
            states: SmearorCompositorStates {
                compositor_state,
                xdg_shell_state,
                xdg_decoration_state,
                xdg_dialog_state,
                viewporter_state,
                shm_state,
                output_manager_state,
                seat_state,
                data_device_state,
                popups,
                seat,
            },
            touch_slot_manager: ThreadSafeTouchSlotManager::new(),
            // listening_socket: Some(listening_socket),
            virtual_output: Some(virtual_output),

            buffers_in_use: Arc::new(DashSet::new()),
            texture_cache: Arc::new(DashMap::new()),
            damage_regions: Arc::new(DashMap::new()),
            surface_buffers: Arc::new(DashMap::new()),
            buffer_holding_area: Arc::new(Mutex::new(std::collections::HashMap::new())),

            subsurfaces: Arc::new(Mutex::new(Vec::new())),
            dialogs: Arc::new(Mutex::new(Vec::new())),
            dialog_configure_sizes: Arc::new(DashMap::new()),

            frame_rate_limit_ms: Arc::new(AtomicI64::new(16)),
            last_frame_time: Arc::new(AtomicI64::new(0)),

            rendered_surfaces: Arc::new(DashSet::new()),
            rendered_outputs: Arc::new(DashSet::new()),
            last_render_times: Arc::new(DashMap::new()),
            output_damage: Arc::new(DashMap::new()),
            surface_damage: Arc::new(DashMap::new()),
            render_cache: Arc::new(DashMap::new()),

            window_size_callback: Arc::new(Mutex::new(None)),
            commit_callback: Arc::new(Mutex::new(None)),
            pending_frame_callbacks: Arc::new(Mutex::new(Vec::new())),
            message_sender: Arc::new(Mutex::new(None)),
            has_had_surface: Arc::new(Mutex::new(false)),
            first_commit_received: Arc::new(AtomicBool::new(false)),

            double_buffer_enabled: Arc::new(AtomicBool::new(true)),
            front_buffer: Arc::new(Mutex::new(None)),
            back_buffer: Arc::new(Mutex::new(None)),

            dma_buf_enabled: Arc::new(AtomicBool::new(dma_buf_enabled)),
            dma_buf_render_count: Arc::new(AtomicU32::new(0)),
            shm_render_count: Arc::new(AtomicU32::new(0)),
            dma_buf_state: Some(dmabuf_state),
            dma_buf_global: dmabuf_global,
            dma_buf_allocator: Some(dma_buf_allocator),
            dma_buf_feedback: None,

            client_decorations_enabled: Arc::new(AtomicBool::new(true)),

            opacity: Arc::new(Mutex::new(1.0)),
            background_color: Arc::new(Mutex::new(None)),
            subsurface_background_color: Arc::new(Mutex::new(None)),
            color_mask: Arc::new(Mutex::new(None)),
            auto_color_mask: Arc::new(AtomicBool::new(false)),
            color_mask_shader: Arc::new(AtomicBool::new(false)),
            color_mask_detected: Arc::new(AtomicBool::new(false)),
            color_mask_tolerance: Arc::new(Mutex::new(0.1)),
            subsurface_color_mask: Arc::new(Mutex::new(None)),
            auto_subsurface_color_mask: Arc::new(AtomicBool::new(false)),
            subsurface_color_mask_detected: Arc::new(AtomicBool::new(false)),
            frame_count: Arc::new(AtomicU32::new(0)),
            commit_counts: Arc::new(DashMap::new()),
            first_commit_times: Arc::new(DashMap::new()),
            last_window_sizes: Arc::new(DashMap::new()),
            clipboard_content: Arc::new(Mutex::new(None)),
            clipboard_source: Arc::new(Mutex::new(None)),
            clipboard_pipe_read_end: Arc::new(Mutex::new(None)),
        })
    }

    /// Set opacity for background rendering
    pub fn set_opacity(&self, opacity: f32) {
        let clamped_opacity = opacity.clamp(0.0, 1.0);
        *self.opacity.lock().unwrap() = clamped_opacity;
        debug!("Opacity set to {}", clamped_opacity);
    }

    /// Get opacity
    pub fn get_opacity(&self) -> f32 {
        *self.opacity.lock().unwrap()
    }

    /// Update window size for a surface and invalidate color mask if size changed
    pub fn update_window_size(&self, surface_id: ObjectId, width: i32, height: i32) {
        let size_changed = if let Some(last_size) = self.last_window_sizes.get(&surface_id) {
            if let Ok(size) = last_size.lock() {
                if let Some((last_w, last_h)) = *size {
                    last_w != width || last_h != height
                } else {
                    true
                }
            } else {
                true
            }
        } else {
            true
        };

        if size_changed {
            info!("Window size changed for surface {:?}: {}x{}, invalidating color mask", surface_id, width, height);
            self.set_color_mask_detected(false);
            self.reset_commit_count(surface_id.clone());
        }

        let size_entry = self.last_window_sizes.entry(surface_id).or_insert_with(|| Arc::new(Mutex::new(None)));
        if let Ok(mut size) = size_entry.lock() {
            *size = Some((width, height));
        }
    }

    /// Check if enough time has passed for auto-detection (default: 2 seconds)
    pub fn should_detect_color(&self, delay_seconds: u64) -> bool {
        let elapsed = self.start_time.elapsed();
        elapsed.as_secs() >= delay_seconds
    }

    /// Force immediate color detection by bypassing the time delay
    pub fn force_detection(&self) {
        // Mark output as damaged to force re-render and detection
        // This is a placeholder - the actual implementation would need
        // access to the output object
        debug!("Force detection requested (requires output damage marking)");
    }

    /// Mark output as damaged to force re-render
    pub fn mark_output_for_redraw(&self) {
        // This is a placeholder - marking output damage requires access to Output object
        // For now, this is a no-op
        debug!("Mark output for redraw requested");
    }

    /// Send configure events to all toplevels to force them to send new buffers
    /// This triggers color detection because new buffers will be rendered
    pub fn force_buffer_update(&self) {
        for toplevel in self.xdg_shell_state.toplevel_surfaces().iter() {
            if let Some(window) = self
                .space
                .elements()
                .find(|w| w.toplevel().map(|t| t.wl_surface() == toplevel.wl_surface()).unwrap_or(false))
            {
                if let Some(toplevel_surface) = window.toplevel() {
                    let wl_surface = toplevel_surface.wl_surface();
                    debug!("Sending configure event to toplevel {:?} to force buffer update", wl_surface.id());

                    // Get current window size and send slightly different size to force buffer update
                    let bbox = window.bbox();
                    let width = bbox.size.w;
                    let height = bbox.size.h;

                    // Send slightly different size to force client to send new buffer
                    // Then send original size in second configure event
                    toplevel_surface.with_pending_state(|state| {
                        state.size = Some((width + 1, height).into());
                        state.states.set(xdg_toplevel::State::Activated);
                    });
                    let _serial1 = toplevel_surface.send_configure();
                    debug!("First configure event sent with size {}x{} to force buffer update", width + 1, height);

                    // Send original size
                    toplevel_surface.with_pending_state(|state| {
                        state.size = Some((width, height).into());
                        state.states.set(xdg_toplevel::State::Activated);
                    });
                    let _serial2 = toplevel_surface.send_configure();
                    debug!("Second configure event sent with original size {}x{}", width, height);
                }
            }
        }
    }

    // TODO: Phase 7 - DMA-BUF support for hardware acceleration - Implement DMA-BUF buffer allocation
    // This requires:
    // - DMA-BUF buffer allocation logic
    // - Integration with rendering pipeline
    // - DMA-BUF buffer lifecycle management
    // - Hardware-accelerated rendering with DMA-BUF
    // See: https://smithay.github.io/smithay/dmabuf.html

    /// Get DMA-BUF state if available
    pub fn dma_buf_state(&self) -> Option<&dmabuf::DmabufState> {
        self.dma_buf_state.as_ref()
    }

    /// Initialize DMA-BUF global with supported formats
    ///
    /// This requires a renderer with DRM device support to provide format feedback.
    /// This is typically called when the OpenGL renderer is initialized.
    ///
    /// # Arguments
    ///
    /// * `formats` - The set of DMA-BUF formats supported by the renderer
    ///
    /// TODO: Phase 7 - DMA-BUF support for hardware acceleration - Complete DmabufGlobal initialization
    /// This requires:
    /// 1. Get main device for DMA-BUF from renderer (dev_t)
    /// 2. Query supported formats from renderer (Vec<Format>)
    /// 3. Create DmabufFeedbackBuilder with device and formats
    /// 4. Build DmabufFeedback
    /// 5. Create DmabufGlobal with default feedback
    pub fn init_dmabuf_global(&mut self, formats: FormatSet) -> Result<()> {
        debug!("Initializing DMA-BUF global");

        // Get the DRM device from the DMA-BUF allocator
        let main_device = self
            .dma_buf_allocator
            .as_ref()
            .and_then(|allocator| allocator.drm_node())
            .ok_or_else(|| CoreError::compositor("DMA-BUF allocator DRM node not initialized".to_string()))?
            .dev_id();

        debug!("Using DRM device for DMA-BUF: dev_id={}", main_device);

        // Create DmabufFeedbackBuilder with device and formats
        let feedback = DmabufFeedbackBuilder::new(main_device, formats)
            .build()
            .map_err(|e| CoreError::compositor(format!("Failed to build DMA-BUF feedback: {}", e)))?;

        debug!("DMA-BUF feedback created successfully");

        // Store the feedback for use in new_surface_feedback
        self.dma_buf_feedback = Some(feedback.clone());

        // Create DmabufGlobal with default feedback
        // The global is automatically registered in the display and doesn't need to be stored
        let _dmabuf_global = self
            .dma_buf_state
            .as_mut()
            .ok_or_else(|| CoreError::compositor("DMA-BUF state not initialized".to_string()))?
            .create_global_with_default_feedback::<SmearorCompositor>(&self.display_handle, &feedback);

        debug!("DMA-BUF global created successfully");

        Ok(())
    }

    /// Send pending frame callbacks after GTK has finished rendering
    /// This synchronizes Firefox with GTK's rendering cycle
    pub fn send_pending_frame_callbacks(&self) {
        if let Some(output) = &self.virtual_output {
            let mut pending_exists = false;
            if let Ok(mut pending) = self.pending_frame_callbacks.lock() {
                if !pending.is_empty() {
                    pending_exists = true;
                    pending.clear();
                }
            }

            if pending_exists {
                let elapsed = self.start_time.elapsed();
                for window in self.space.elements() {
                    window.send_frame(output, elapsed, Some(std::time::Duration::ZERO), |_, _| self.virtual_output.clone());
                    debug!("Sent pending frame callbacks for window {:?}", window);
                }
            }
        }
    }
}

/// Data passed to the event loop
pub struct CalloopData {
    pub state: Arc<Mutex<SmearorCompositor>>,
    pub display_handle: DisplayHandle,
}
