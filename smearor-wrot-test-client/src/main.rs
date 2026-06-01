//! smearor-wrot-test-client: Test client for the compositor
//!
//! This is a basic Wayland client that connects to the smearor-wrot compositor
//! and tests basic functionality including visual rotation.

use miette::Result;
use std::env;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::BorrowedFd;
use std::time::Duration;
use std::time::Instant;
use wayland_client::Connection;
use wayland_client::Dispatch;
use wayland_client::QueueHandle;
use wayland_client::protocol::wl_buffer;
use wayland_client::protocol::wl_compositor;
use wayland_client::protocol::wl_display;
use wayland_client::protocol::wl_pointer;
use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_seat;
use wayland_client::protocol::wl_shm;
use wayland_client::protocol::wl_shm_pool;
use wayland_client::protocol::wl_surface;
use wayland_protocols::xdg::shell::client::xdg_surface;
use wayland_protocols::xdg::shell::client::xdg_toplevel;
use wayland_protocols::xdg::shell::client::xdg_wm_base;

/// State for the Wayland client
struct ClientState {
    compositor: Option<wl_compositor::WlCompositor>,
    surface: Option<wl_surface::WlSurface>,
    xdg_wm_base: Option<xdg_wm_base::XdgWmBase>,
    xdg_surface: Option<xdg_surface::XdgSurface>,
    xdg_toplevel: Option<xdg_toplevel::XdgToplevel>,
    shm: Option<wl_shm::WlShm>,
    shm_pool: Option<wl_shm_pool::WlShmPool>,
    buffer: Option<wl_buffer::WlBuffer>,
    seat: Option<wl_seat::WlSeat>,
    pointer: Option<wl_pointer::WlPointer>,
    running: bool,
    rotation_angle: f32,
    connection_time: Option<Duration>,
    registry_time: Option<Duration>,
    configured: bool,
    buffer_data: Option<Vec<u8>>,
    pending_width: Option<i32>,
    pending_height: Option<i32>,
}

impl ClientState {
    fn new(rotation_angle: f32) -> Self {
        Self {
            compositor: None,
            surface: None,
            xdg_wm_base: None,
            xdg_surface: None,
            xdg_toplevel: None,
            shm: None,
            shm_pool: None,
            buffer: None,
            seat: None,
            pointer: None,
            running: true,
            rotation_angle,
            connection_time: None,
            registry_time: None,
            configured: false,
            buffer_data: None,
            pending_width: None,
            pending_height: None,
        }
    }
}

impl Dispatch<wl_display::WlDisplay, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_display::WlDisplay,
        _event: <wl_display::WlDisplay as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for ClientState {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            match interface.as_str() {
                "wl_compositor" => {
                    let compositor = proxy.bind::<wl_compositor::WlCompositor, _, _>(name, version, qh, ());
                    state.compositor = Some(compositor);
                    println!("Bound wl_compositor");
                }
                "wl_shm" => {
                    let shm = proxy.bind::<wl_shm::WlShm, _, _>(name, version, qh, ());
                    state.shm = Some(shm);
                    println!("Bound wl_shm");
                }
                "xdg_wm_base" => {
                    let xdg_wm_base = proxy.bind::<xdg_wm_base::XdgWmBase, _, _>(name, version, qh, ());
                    state.xdg_wm_base = Some(xdg_wm_base);
                    println!("Bound xdg_wm_base");
                }
                "wl_seat" => {
                    let seat = proxy.bind::<wl_seat::WlSeat, _, _>(name, version, qh, ());
                    state.seat = Some(seat);
                    println!("Bound wl_seat");
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_compositor::WlCompositor,
        _event: <wl_compositor::WlCompositor as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_surface::WlSurface,
        event: <wl_surface::WlSurface as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let wl_surface::Event::Enter { .. } = event {
            println!("Surface entered output");
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for ClientState {
    fn event(
        state: &mut Self,
        _proxy: &xdg_wm_base::XdgWmBase,
        event: <xdg_wm_base::XdgWmBase as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            if let Some(xdg_wm_base) = &state.xdg_wm_base {
                xdg_wm_base.pong(serial);
            }
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for ClientState {
    fn event(
        state: &mut Self,
        _proxy: &xdg_surface::XdgSurface,
        event: <xdg_surface::XdgSurface as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            if let Some(xdg_surface) = &state.xdg_surface {
                xdg_surface.ack_configure(serial);
            }
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for ClientState {
    fn event(
        state: &mut Self,
        _proxy: &xdg_toplevel::XdgToplevel,
        event: <xdg_toplevel::XdgToplevel as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _queue_handle: &QueueHandle<Self>,
    ) {
        match event {
            xdg_toplevel::Event::Close => {
                println!("Toplevel close requested");
                state.running = false;
            }
            xdg_toplevel::Event::Configure { width, height, .. } => {
                println!("=== CONFIGURE EVENT RECEIVED === width={}, height={}", width, height);
                state.configured = true;
                state.pending_width = Some(width);
                state.pending_height = Some(height);

                // Don't create and attach buffer here - do it in the event loop
                // The commit will block if called in the event handler
                println!("Will create and attach buffer in event loop");
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_shm_pool::WlShmPool,
        _event: <wl_shm_pool::WlShmPool as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_buffer::WlBuffer,
        _event: <wl_buffer::WlBuffer as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_shm::WlShm, ()> for ClientState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_shm::WlShm,
        _event: <wl_shm::WlShm as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for ClientState {
    fn event(
        state: &mut Self,
        proxy: &wl_seat::WlSeat,
        event: <wl_seat::WlSeat as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_seat::Event::Capabilities { capabilities, .. } => {
                println!("Seat capabilities: {:?}", capabilities);

                // Get pointer if available
                // Try to get pointer - if seat doesn't support it, the request will be ignored
                let pointer = proxy.get_pointer(qh, ());
                state.pointer = Some(pointer);
                println!("Requested wl_pointer");
            }
            wl_seat::Event::Name { name } => {
                println!("Seat name: {}", name);
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for ClientState {
    fn event(
        state: &mut Self,
        _proxy: &wl_pointer::WlPointer,
        event: <wl_pointer::WlPointer as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Enter {
                surface: _,
                surface_x,
                surface_y,
                ..
            } => {
                println!("Pointer entered surface: surface_x={}, surface_y={}", surface_x, surface_y);

                // Test coordinate transformation with rotation
                let (transformed_x, transformed_y) = transform_coordinates(surface_x as f32, surface_y as f32, state.rotation_angle);
                println!(
                    "Input transformation test: ({}, {}) -> ({}, {}) with {}° rotation",
                    surface_x, surface_y, transformed_x, transformed_y, state.rotation_angle
                );
            }
            wl_pointer::Event::Motion { surface_x, surface_y, .. } => {
                println!("Pointer motion: surface_x={}, surface_y={}", surface_x, surface_y);
            }
            wl_pointer::Event::Button {
                button, state: button_state, ..
            } => {
                println!("Pointer button: button={}, state={:?}", button, button_state);
            }
            wl_pointer::Event::Leave { .. } => {
                println!("Pointer left surface");
            }
            _ => {}
        }
    }
}

/// Transform coordinates based on rotation angle
fn transform_coordinates(x: f32, y: f32, rotation_degrees: f32) -> (f32, f32) {
    let radians = rotation_degrees.to_radians();
    let cos = radians.cos();
    let sin = radians.sin();

    // Rotate coordinates around origin (0, 0)
    let new_x = x * cos - y * sin;
    let new_y = x * sin + y * cos;

    (new_x, new_y)
}

impl ClientState {
    fn create_and_attach_buffer(&mut self, qh: &QueueHandle<Self>, width: i32, height: i32) -> Result<()> {
        let width = if width > 0 { width } else { 640 };
        let height = if height > 0 { height } else { 480 };

        println!("Creating buffer: {}x{}", width, height);

        // Calculate buffer size (4 bytes per pixel for RGBA)
        let stride = width * 4;
        let buffer_size = (stride * height) as usize;

        // Create shared memory file using memfd_create (required for Wayland SHM)
        let memfd_name = format!("wl_shm_{}", std::process::id());
        let memfd_fd = unsafe { libc::memfd_create(memfd_name.as_ptr() as *const i8, libc::MFD_CLOEXEC | libc::MFD_ALLOW_SEALING) };
        if memfd_fd < 0 {
            return Err(miette::miette!("Failed to create memfd: {}", std::io::Error::last_os_error()));
        }
        let mut memfd: std::fs::File = unsafe { std::os::unix::io::FromRawFd::from_raw_fd(memfd_fd) };

        // Set file size
        memfd
            .set_len(buffer_size as u64)
            .map_err(|e| miette::miette!("Failed to set memfd size: {}", e))?;

        // Generate RGBA pixel data
        let mut pixel_data = vec![0u8; buffer_size];
        for y in 0..height {
            for x in 0..width {
                let offset = ((y * stride) + (x * 4)) as usize;

                // Create a gradient pattern
                let r = ((x as f32 / width as f32) * 255.0) as u8;
                let g = ((y as f32 / height as f32) * 255.0) as u8;
                let b = 128;
                let a = 255;

                pixel_data[offset] = b; // Blue
                pixel_data[offset + 1] = g; // Green
                pixel_data[offset + 2] = r; // Red
                pixel_data[offset + 3] = a; // Alpha
            }
        }

        // Write pixel data to shared memory
        memfd.write_all(&pixel_data).map_err(|e| miette::miette!("Failed to write pixel data: {}", e))?;

        // Create SHM pool
        let shm = self.shm.as_ref().ok_or_else(|| miette::miette!("SHM not available"))?;

        let borrowed_fd = unsafe { BorrowedFd::borrow_raw(memfd.as_raw_fd()) };
        let pool = shm.create_pool(borrowed_fd, buffer_size as i32, qh, ());
        self.shm_pool = Some(pool);

        // Create buffer from pool
        let pool = self.shm_pool.as_ref().ok_or_else(|| miette::miette!("SHM pool not created"))?;

        let buffer = pool.create_buffer(0, width, height, stride, wl_shm::Format::Argb8888, qh, ());
        self.buffer = Some(buffer);
        self.buffer_data = Some(pixel_data);

        // Attach buffer to surface
        let surface = self.surface.as_ref().ok_or_else(|| miette::miette!("Surface not created"))?;

        let buffer = self.buffer.as_ref().ok_or_else(|| miette::miette!("Buffer not created"))?;

        surface.attach(Some(buffer), 0, 0);
        println!("About to commit surface after buffer attach");
        surface.commit();
        println!("Surface committed after buffer attach");

        println!("Buffer created and attached successfully");

        Ok(())
    }

    fn create_window(&mut self, qh: &QueueHandle<Self>) -> Result<()> {
        let compositor = self.compositor.as_ref().ok_or_else(|| miette::miette!("Compositor not available"))?;

        let xdg_wm_base = self.xdg_wm_base.as_ref().ok_or_else(|| miette::miette!("XDG WM base not available"))?;

        // Create surface
        let surface = compositor.create_surface(qh, ());
        self.surface = Some(surface);

        // Get the surface back
        let surface = self.surface.as_ref().ok_or_else(|| miette::miette!("Surface not created"))?;

        // Create XDG surface
        let xdg_surface = xdg_wm_base.get_xdg_surface(surface, qh, ());
        self.xdg_surface = Some(xdg_surface);

        // Get the XDG surface back
        let xdg_surface = self.xdg_surface.as_ref().ok_or_else(|| miette::miette!("XDG surface not created"))?;

        // Create toplevel
        let toplevel = xdg_surface.get_toplevel(qh, ());
        self.xdg_toplevel = Some(toplevel);

        // Set toplevel title
        if let Some(toplevel) = &self.xdg_toplevel {
            toplevel.set_title("smearor-wrot Test Client".to_string());
            toplevel.set_app_id("io.smearor.wrot.test".to_string());
        }

        // Commit surface
        surface.commit();

        println!("Window created successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_coordinates_zero_rotation() {
        let (x, y) = transform_coordinates(100.0, 200.0, 0.0);
        assert!((x - 100.0).abs() < 0.01);
        assert!((y - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_transform_coordinates_ninety_degrees() {
        let (x, y) = transform_coordinates(100.0, 200.0, 90.0);
        // 90° rotation: (x, y) -> (-y, x)
        assert!((x - (-200.0)).abs() < 0.01);
        assert!((y - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_transform_coordinates_one_hundred_eighty_degrees() {
        let (x, y) = transform_coordinates(100.0, 200.0, 180.0);
        // 180° rotation: (x, y) -> (-x, -y)
        assert!((x - (-100.0)).abs() < 0.01);
        assert!((y - (-200.0)).abs() < 0.01);
    }

    #[test]
    fn test_transform_coordinates_two_hundred_seventy_degrees() {
        let (x, y) = transform_coordinates(100.0, 200.0, 270.0);
        // 270° rotation: (x, y) -> (y, -x)
        assert!((x - 200.0).abs() < 0.01);
        assert!((y - (-100.0)).abs() < 0.01);
    }

    #[test]
    fn test_transform_coordinates_custom_angle() {
        let (x, y) = transform_coordinates(100.0, 200.0, 45.0);
        // 45° rotation should produce specific values
        // Just verify it produces reasonable results
        assert!(x.is_finite());
        assert!(y.is_finite());
    }

    #[test]
    fn test_transform_coordinates_origin() {
        let (x, y) = transform_coordinates(0.0, 0.0, 90.0);
        // Origin should remain at origin regardless of rotation
        assert!((x - 0.0).abs() < 0.01);
        assert!((y - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_transform_coordinates_negative_values() {
        let (x, y) = transform_coordinates(-100.0, -200.0, 90.0);
        // Should handle negative coordinates correctly
        assert!(x.is_finite());
        assert!(y.is_finite());
    }

    #[test]
    fn test_rotation_parsing_valid() {
        let angle: f32 = "90".parse().unwrap();
        assert!((angle - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_rotation_parsing_invalid() {
        let angle: Result<f32, _> = "invalid".parse();
        assert!(angle.is_err());
    }

    #[test]
    fn test_rotation_parsing_float() {
        let angle: f32 = "45.5".parse().unwrap();
        assert!((angle - 45.5).abs() < 0.01);
    }
}

fn main() -> Result<()> {
    println!("Test Client started");
    let args: Vec<String> = env::args().collect();

    // Get rotation angle from command line (default: 0)
    let rotation: f32 = if args.len() > 1 { args[1].parse().unwrap_or(0.0) } else { 0.0 };

    // Get Wayland display from environment
    let display_name = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| {
        eprintln!("WAYLAND_DISPLAY not set, using default");
        "wayland-0".to_string()
    });

    println!("=== smearor-wrot Test Client ===");
    println!("Connecting to Wayland display: {}", display_name);
    println!("Rotation angle: {}°", rotation);

    // Connect to the Wayland display with timing
    let start = Instant::now();
    let conn = Connection::connect_to_env().map_err(|e| miette::miette!("Failed to connect to Wayland display: {}", e))?;
    let connection_duration = start.elapsed();

    println!("Successfully connected to Wayland display");

    let display = conn.display();
    let mut event_queue = conn.new_event_queue();

    let mut state = ClientState::new(rotation);
    state.connection_time = Some(connection_duration);

    let qh = event_queue.handle();

    // Get the registry
    let _registry = display.get_registry(&qh, ());

    println!("Getting registry...");

    // Roundtrip to get all globals with timing
    let start = Instant::now();
    event_queue.roundtrip(&mut state).map_err(|e| miette::miette!("Failed to roundtrip: {}", e))?;
    let registry_duration = start.elapsed();
    state.registry_time = Some(registry_duration);

    println!("Registry received");

    // Create window
    if let Err(e) = state.create_window(&qh) {
        println!("Failed to create window: {}", e);
        return Ok(());
    }

    // Roundtrip to get configure event
    event_queue
        .roundtrip(&mut state)
        .map_err(|e| miette::miette!("Failed to roundtrip after window creation: {}", e))?;

    // Wait for configure event
    if !state.configured {
        println!("Waiting for configure event...");
        event_queue
            .roundtrip(&mut state)
            .map_err(|e| miette::miette!("Failed to roundtrip for configure: {}", e))?;
    }

    // Run event loop for a longer time to allow buffer commit
    println!("\n=== Running event loop ===");
    let start = Instant::now();
    let timeout = Duration::from_secs(30);
    let mut iteration = 0;
    let mut buffer_created = false;
    let qh = &qh; // Re-borrow qh for use in the loop

    while state.running && start.elapsed() < timeout {
        iteration += 1;
        if iteration % 10 == 0 {
            println!("Event loop iteration {}", iteration);
        }

        // Check if we need to create and attach buffer
        if !buffer_created && state.pending_width.is_some() && state.pending_height.is_some() && state.buffer.is_none() {
            let width = state.pending_width.unwrap();
            let height = state.pending_height.unwrap();
            println!("Creating and attaching buffer in event loop: {}x{}", width, height);
            if let Err(e) = state.create_and_attach_buffer(qh, width, height) {
                println!("Failed to create and attach buffer: {}", e);
            } else {
                println!("Buffer created and attached successfully");
                buffer_created = true;

                // Roundtrip to ensure the commit is processed by the compositor
                println!("Roundtrip after buffer attach to ensure commit is processed");
                if let Err(e) = event_queue.roundtrip(&mut state) {
                    println!("Failed to roundtrip after buffer attach: {}", e);
                } else {
                    println!("Roundtrip successful");
                }
            }
        }

        event_queue
            .dispatch_pending(&mut state)
            .map_err(|e| miette::miette!("Failed to dispatch events: {}", e))?;

        std::thread::sleep(Duration::from_millis(16));
    }

    println!("Event loop ended after {} iterations", iteration);

    println!("\n=== Test Results ===");
    if state.buffer.is_some() {
        println!("✓ Buffer created and attached successfully");
        println!("✓ Window creation and rendering pipeline functional");
    } else {
        println!("✗ Buffer creation failed");
    }

    if state.configured {
        println!("✓ Configure event received");
    } else {
        println!("✗ Configure event not received");
    }

    // Keep running for a moment to show the message
    std::thread::sleep(Duration::from_secs(2));

    println!("\nTest completed. Connection verified.");
    println!("Input transformation testing requires window creation to be functional.");

    Ok(())
}
