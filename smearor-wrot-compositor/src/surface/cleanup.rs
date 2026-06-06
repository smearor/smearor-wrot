use crate::SmearorCompositor;
use crate::lifecycle::shutdown::ShutdownHandler;
use smithay::reexports::wayland_server::Resource;
use tracing::debug;
use tracing::info;

pub trait DeadSurfaceCleanup {
    /// Remove surfaces that have no buffers attached
    fn cleanup_dead_surfaces(&mut self);
}

impl DeadSurfaceCleanup for SmearorCompositor {
    fn cleanup_dead_surfaces(&mut self) {
        debug!("cleanup_dead_surfaces");
        // Remove dialogs that are no longer in the Smithay space
        if let Ok(mut dialogs) = self.dialogs.lock() {
            let mut dialogs_to_remove = Vec::new();
            for dialog in dialogs.iter() {
                let dialog_surface = dialog.wl_surface();
                // Check if the dialog is still in the Smithay space
                let is_still_mapped = self
                    .space
                    .elements()
                    .any(|window| window.toplevel().map(|t| t.wl_surface() == dialog_surface).unwrap_or(false));
                if !is_still_mapped {
                    dialogs_to_remove.push(dialog.clone());
                    info!("Dialog {:?} is no longer mapped, removing from registry", dialog_surface.id());
                }
            }
            for dialog_to_remove in dialogs_to_remove {
                dialogs.retain(|d| d.wl_surface() != dialog_to_remove.wl_surface());
            }
        }

        // Smithay's Space doesn't have an unmap_element method
        // Instead, we need to rebuild the space without dead surfaces
        // For now, we just check for shutdown after buffer destruction
        // The actual surface removal is handled by Smithay internally
        self.check_and_request_shutdown();
    }
}
