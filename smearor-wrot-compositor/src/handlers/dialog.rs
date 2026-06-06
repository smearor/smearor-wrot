use crate::SmearorCompositor;
use smithay::reexports::wayland_server::Resource;
use smithay::wayland::shell::xdg::ToplevelSurface;
use smithay::wayland::shell::xdg::dialog::XdgDialogHandler;
use tracing::debug;
use tracing::error;

impl XdgDialogHandler for SmearorCompositor {
    fn modal_changed(&mut self, toplevel: ToplevelSurface, is_modal: bool) {
        debug!(
            "XdgDialogHandler: modal_changed called for toplevel {:?}: is_modal = {}",
            toplevel.wl_surface().id(),
            is_modal
        );

        let Ok(mut dialogs) = self.dialogs.lock() else {
            error!("Failed to lock dialogs registry");
            return;
        };

        // Track all dialogs (both modal and non-modal)
        // This is necessary because Totem calls unset_modal() immediately after get_xdg_dialog
        if !dialogs.iter().any(|d| d.wl_surface() == toplevel.wl_surface()) {
            dialogs.push(toplevel.clone());
            debug!("XdgDialogHandler: Added dialog to registry (modal: {}), total dialogs: {}", is_modal, dialogs.len());
        } else {
            debug!("XdgDialogHandler: Dialog already in registry");
        }

        // Note: We don't remove dialogs when is_modal becomes false
        // Dialogs are removed when they are unmapped/closed
    }
}

smithay::delegate_xdg_dialog!(SmearorCompositor);
