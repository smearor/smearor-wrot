use crate::SocketManager;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct SmearorWrotApplication {
    socket_manager: Arc<SocketManager>,
}

impl SmearorWrotApplication {
    pub fn socket_manager(&self) -> Arc<SocketManager> {
        self.socket_manager.clone()
    }
}
