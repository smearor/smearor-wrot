use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::socket::error::SocketInitializationError;
use crate::widget::socket::handler::SocketHandler;
use smearor_wrot_model::Socket;
use tracing::debug;

impl SocketHandler for CompositorWidgetImpl {
    fn socket(&self) -> Option<Socket> {
        self.socket.borrow().clone()
    }

    fn initialize_socket(&self, socket: Socket) -> Result<(), SocketInitializationError> {
        debug!("initialize_socket {socket}");
        *self.socket.borrow_mut() = Some(socket.clone());
        Ok(())
    }
}
