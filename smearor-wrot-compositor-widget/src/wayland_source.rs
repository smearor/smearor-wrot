//! Wayland event source integration with GTK main loop

use glib::ControlFlow;
use glib::IOCondition;
use glib::MainContext;
use smearor_wrot_compositor::compositor::SmearorCompositor;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::wayland_server::ListeningSocket;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;
use tracing::error;

/// Wayland event source for integrating Wayland compositor with GTK main loop
///
/// This source monitors the Wayland display file descriptor and listening socket
/// to integrate Smithay's event processing with GTK's main loop.
pub struct WaylandSource {
    wayland_display: Arc<Mutex<Display<SmearorCompositor>>>,
    compositor: Arc<Mutex<SmearorCompositor>>,
    listening_socket: Arc<Mutex<ListeningSocket>>,
}

impl WaylandSource {
    /// Creates a new WaylandSource
    ///
    /// # Arguments
    ///
    /// * `wayland_display` - The Wayland display server instance
    /// * `compositor` - The compositor state
    /// * `listening_socket` - The listening socket for accepting client connections
    pub fn new(wayland_display: Arc<Mutex<Display<SmearorCompositor>>>, compositor: Arc<Mutex<SmearorCompositor>>, listening_socket: ListeningSocket) -> Self {
        debug!("Creating WaylandSource with display, compositor, and listening socket");
        Self {
            wayland_display,
            compositor,
            listening_socket: Arc::new(Mutex::new(listening_socket)),
        }
    }

    /// Attaches the WaylandSource to a GLib main context
    ///
    /// This sets up file descriptor watchers for Wayland events and client connections,
    /// integrating with the GTK event loop.
    ///
    /// # Arguments
    ///
    /// * `main_context` - The GLib main context to attach to
    pub fn attach(self, _main_context: &MainContext) {
        debug!("Attaching WaylandSource to main context");

        let mut display = match self.wayland_display.lock() {
            Ok(guard) => guard,
            Err(_) => {
                error!("Failed to lock wayland display");
                return;
            }
        };
        let display_poll_fd = display.backend().poll_fd();
        let display_poll_raw_fd = display_poll_fd.as_raw_fd();

        let display_reference = self.wayland_display.clone();
        let compositor_reference = self.compositor.clone();

        // Monitor the display file descriptor for client events
        glib_unix::unix_fd_add_local(display_poll_raw_fd, IOCondition::IN | IOCondition::ERR | IOCondition::HUP, move |_fd, condition| {
            if condition.contains(IOCondition::ERR) || condition.contains(IOCondition::HUP) {
                error!("Wayland display socket experienced an error or hang-up");
                return ControlFlow::Break;
            }

            let mut display_borrowed = match display_reference.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    error!("Failed to lock display reference");
                    return glib::ControlFlow::Continue;
                }
            };
            let mut compositor_borrowed = match compositor_reference.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    error!("Failed to lock compositor reference");
                    return glib::ControlFlow::Continue;
                }
            };

            debug!("Dispatching Wayland events");
            if let Err(dispatch_error) = display_borrowed.dispatch_clients(&mut *compositor_borrowed) {
                error!("Failed to dispatch Wayland clients: {:?}", dispatch_error);
            } else {
                debug!("Wayland events dispatched successfully");
            }

            if let Err(flush_error) = display_borrowed.flush_clients() {
                error!("Failed to flush Wayland clients: {:?}", flush_error);
            }

            ControlFlow::Continue
        });

        // Monitor the listening socket for new client connections
        let listening_socket_fd = match self.listening_socket.lock() {
            Ok(guard) => guard.as_raw_fd(),
            Err(_) => {
                error!("Failed to lock listening socket");
                return;
            }
        };
        debug!("Listening socket file descriptor: {}", listening_socket_fd);
        let display_reference_for_connections = self.wayland_display.clone();
        let listening_socket_reference = self.listening_socket.clone();

        glib_unix::unix_fd_add_local(listening_socket_fd, IOCondition::IN | IOCondition::ERR | IOCondition::HUP, move |_fd, condition| {
            debug!("Listening socket callback triggered, condition: {:?}", condition);
            if condition.contains(IOCondition::ERR) || condition.contains(IOCondition::HUP) {
                error!("Wayland listening socket experienced an error or hang-up");
                return ControlFlow::Break;
            }

            let display_borrowed = match display_reference_for_connections.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    error!("Failed to lock display reference for connections");
                    return ControlFlow::Continue;
                }
            };
            let mut display_handle = display_borrowed.handle();

            debug!("Attempting to accept new client connection");
            match listening_socket_reference.lock() {
                Ok(listening_socket) => match listening_socket.accept() {
                    Ok(Some(client_stream)) => {
                        debug!("Client connection accepted, inserting into display");
                        if let Err(e) = display_handle.insert_client(client_stream, Arc::new(smearor_wrot_compositor::state::client::ClientState::default())) {
                            error!("Failed to insert client: {}", e);
                        } else {
                            debug!("Client inserted successfully");
                        }
                    }
                    Ok(None) => {
                        debug!("No client connection available (accept returned None)");
                    }
                    Err(e) => {
                        error!("Failed to accept client connection: {}", e);
                    }
                },
                Err(_) => {
                    error!("Failed to lock listening socket reference");
                }
            }

            ControlFlow::Continue
        });

        debug!("WaylandSource attached successfully");
    }
}
