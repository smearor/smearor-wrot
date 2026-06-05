use smearor_wrot_model::Socket;

pub struct SocketManager {
    socket: Socket,
}

impl SocketManager {
    pub fn new(socket: Socket) -> Self {
        Self { socket }
    }

    pub fn socket(&self) -> Socket {
        self.socket.clone()
    }
}
