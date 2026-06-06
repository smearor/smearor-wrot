//! Wayland DMA-BUF protocol handler

use crate::DmaBuffer;
use crate::compositor::SmearorCompositor;
use smithay::backend::allocator::Buffer;
use smithay::backend::allocator::dmabuf as allocator_dmabuf;
use smithay::wayland::dmabuf;
use smithay::wayland::dmabuf::DmabufFeedback;
use smithay::wayland::dmabuf::DmabufGlobal;
use smithay::wayland::dmabuf::DmabufHandler;
use smithay::wayland::dmabuf::ImportNotifier;
use tracing::debug;
use tracing::error;

impl DmabufHandler for SmearorCompositor {
    fn dmabuf_state(&mut self) -> &mut dmabuf::DmabufState {
        self.dma_buf_state.as_mut().expect("DMA-BUF state not initialized")
    }

    fn dmabuf_imported(&mut self, _global: &DmabufGlobal, dmabuf: allocator_dmabuf::Dmabuf, notifier: ImportNotifier) {
        debug!("DMA-BUF buffer imported: {:?}", dmabuf);

        let format = dmabuf.format();
        let modifier: u64 = dmabuf.format().modifier.into();

        debug!("DMA-BUF format: {:?}, modifier: {:?}", format.code, modifier);

        // Always accept DMA-BUF import to avoid protocol errors
        // The rendering pipeline will handle DMA-BUF vs SHM based on is_dma_buf_available()
        match notifier.successful::<SmearorCompositor>() {
            Ok(_buffer) => {
                debug!("Successfully imported DMA-BUF buffer");
            }
            Err(e) => {
                error!("Failed to import DMA-BUF: {e}");
            }
        }
    }

    fn new_surface_feedback(
        &mut self,
        _surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
        _global: &DmabufGlobal,
    ) -> Option<DmabufFeedback> {
        if !self.is_dma_buf_available() {
            debug!("DMA-BUF is disabled, no surface feedback");
            return None;
        }

        debug!("Providing DMA-BUF surface feedback");

        self.dma_buf_feedback.clone()
    }
}

smithay::delegate_dmabuf!(SmearorCompositor);
