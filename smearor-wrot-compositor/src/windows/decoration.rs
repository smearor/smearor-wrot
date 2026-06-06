use crate::SmearorCompositor;
use smithay::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;
use std::sync::atomic::Ordering;
use tracing::debug;

pub trait ClientDecorationAware {
    /// Set whether client-side decorations are enabled for Wayland clients
    fn set_client_decorations_enabled(&self, enabled: bool);

    /// Check if client-side decorations are enabled
    fn is_client_decorations_enabled(&self) -> bool;

    /// Get the configured toplevel decoration mode
    fn get_configured_toplevel_decoration_mode(&self) -> Mode;
}

impl ClientDecorationAware for SmearorCompositor {
    fn set_client_decorations_enabled(&self, enabled: bool) {
        self.client_decorations_enabled.store(enabled, Ordering::Relaxed);
        debug!("Client-side decorations {}", if enabled { "enabled" } else { "disabled" });
    }

    fn is_client_decorations_enabled(&self) -> bool {
        self.client_decorations_enabled.load(Ordering::Relaxed)
    }

    fn get_configured_toplevel_decoration_mode(&self) -> Mode {
        if self.is_client_decorations_enabled() {
            Mode::ClientSide
        } else {
            Mode::ServerSide
        }
    }
}
