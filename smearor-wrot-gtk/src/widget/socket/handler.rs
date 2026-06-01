use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;
use tracing::debug;

pub trait SocketHandler {
    fn set_socket_path(&self, socket_path: String);
}

impl SocketHandler for CompositorWidget {
    fn set_socket_path(&self, socket_path: String) {
        debug!("Setting socket path: {socket_path}");
        self.imp().initialize_socket_with_path(socket_path);
    }
}
