use crate::CalloopData;
use crate::CoreError;
use crate::SmearorCompositor;
use smearor_wrot_model::Socket;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::wayland_server::ListeningSocket;
use std::ffi::OsString;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;

pub trait WaylandListener {
    fn init_wayland_listener(
        _display: Arc<Mutex<Display<SmearorCompositor>>>,
        _event_loop: &mut EventLoop<CalloopData>,
        socket: Option<Socket>,
    ) -> crate::error::Result<(OsString, ListeningSocket)>;
}

impl WaylandListener for SmearorCompositor {
    fn init_wayland_listener(
        _display: Arc<Mutex<Display<SmearorCompositor>>>,
        _event_loop: &mut EventLoop<CalloopData>,
        socket: Option<Socket>,
    ) -> crate::error::Result<(OsString, ListeningSocket)> {
        // Creates a new listening socket, using custom name if provided or auto-generating
        let listening_socket = if let Some(socket) = socket {
            debug!("Creating Wayland listening socket with custom name: {}", socket);
            ListeningSocket::bind(socket.deref()).map_err(|e| CoreError::compositor(format!("Failed to create listening socket: {}", e)))?
        } else {
            debug!("Creating Wayland listening socket with auto-generated name");
            ListeningSocket::bind_auto("wayland-0", 0..100).map_err(|e| CoreError::compositor(format!("Failed to create listening socket: {}", e)))?
        };

        // Get the name of the listening socket.
        // Clients will connect to this socket.
        let socket_name = listening_socket
            .socket_name()
            .map(|s| s.to_os_string())
            .ok_or_else(|| CoreError::compositor("Failed to get socket name"))?;
        debug!("Wayland listening socket created with name: {:?}", socket_name);
        debug!("Socket name as string: {:?}", socket_name.to_string_lossy());

        Ok((socket_name, listening_socket))
    }
}
