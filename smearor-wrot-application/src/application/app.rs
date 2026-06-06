use crate::SocketManager;
use crate::application::config::CompositorApplicationConfig;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct CompositorApplication {
    pub config: CompositorApplicationConfig,
}

#[derive(TypedBuilder)]
pub struct SmearorWrotApplication {
    socket_manager: Arc<SocketManager>,
}

impl SmearorWrotApplication {
    pub fn socket_manager(&self) -> Arc<SocketManager> {
        self.socket_manager.clone()
    }
}
