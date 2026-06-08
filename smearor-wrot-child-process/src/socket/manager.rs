use crate::DEFAULT_SOCKET_PREFIX;
use crate::Socket;
use crate::socket::error::SocketBindError;
#[cfg(feature = "smithay")]
use smithay::reexports::wayland_server::ListeningSocket;
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Arc;
use tracing::debug;
use tracing::info;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct SocketManager {
    /// The socket to use.
    socket: Arc<Option<Socket>>,

    /// Wayland listening socket for accepting client connections
    #[cfg(feature = "smithay")]
    pub listening_socket: RefCell<Option<ListeningSocket>>,
}

impl SocketManager {
    pub fn new(socket: Option<Socket>) -> Self {
        Self {
            socket: Arc::new(socket),
            #[cfg(feature = "smithay")]
            listening_socket: RefCell::new(None),
        }
    }

    pub fn socket(&self) -> Arc<Option<Socket>> {
        self.socket.clone()
    }

    #[cfg(feature = "smithay")]
    pub fn is_bind(&self) -> bool {
        self.listening_socket.borrow().is_some()
    }

    #[cfg(feature = "smithay")]
    pub fn socket_name(&self) -> Option<String> {
        self.listening_socket
            .borrow()
            .as_ref()
            .and_then(|listening_socket| listening_socket.socket_name().map(|s| s.to_os_string().to_string_lossy().to_string()))
    }

    #[cfg(feature = "smithay")]
    pub fn bind(&self) -> Result<(), SocketBindError> {
        // Creates a new listening socket, using custom name if provided or auto-generating
        let listening_socket = if let Some(socket) = self.socket.deref() {
            debug!("Creating Wayland listening socket with custom name: {socket}");
            ListeningSocket::bind(socket).map_err(|e| SocketBindError::BindFailed(e.to_string()))?
        } else {
            debug!("Creating Wayland listening socket with auto-generated name");
            ListeningSocket::bind_auto(&format!("{DEFAULT_SOCKET_PREFIX}-0"), 0..100).map_err(|e| SocketBindError::BindFailed(e.to_string()))?
        };
        self.listening_socket.replace(Some(listening_socket));
        if let Some(socket_name) = self.socket_name() {
            info!("Successfully created Wayland listening socket: {socket_name}");
        }
        Ok(())
    }
}
