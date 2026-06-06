use crate::SmearorCompositor;
use smithay::wayland::shm::ShmHandler;
use smithay::wayland::shm::ShmState;

impl ShmHandler for SmearorCompositor {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

smithay::delegate_shm!(SmearorCompositor);
