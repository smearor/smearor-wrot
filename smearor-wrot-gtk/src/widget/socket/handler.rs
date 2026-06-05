use crate::CompositorWidget;
use crate::widget::socket::error::SocketInitializationError;
use glib::subclass::prelude::ObjectSubclassIsExt;
use smearor_wrot_model::Socket;
use tracing::debug;

pub trait SocketHandler {
    /// Returns the socket used to connect to the compositor
    fn socket(&self) -> Option<Socket>;

    /// Sets and initializes the socket used to connect to the compositor
    fn initialize_socket(&self, socket: Socket) -> Result<(), SocketInitializationError>;
}

impl SocketHandler for CompositorWidget {
    fn socket(&self) -> Option<Socket> {
        self.imp().socket()
    }

    fn initialize_socket(&self, socket: Socket) -> Result<(), SocketInitializationError> {
        debug!("Initialize socket: {socket}");
        self.imp().initialize_socket(socket)
    }
}
