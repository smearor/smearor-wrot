use crate::SmearorCompositor;
use crate::windows::decoration::ClientDecorationAware;
use smithay::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;
use smithay::wayland::shell::xdg::ToplevelSurface;
use smithay::wayland::shell::xdg::decoration::XdgDecorationHandler;
use tracing::debug;

impl XdgDecorationHandler for SmearorCompositor {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        let mode = self.get_configured_toplevel_decoration_mode();

        debug!("new_decoration called - setting decoration mode to {:?}", mode);

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(mode);
        });

        toplevel.send_configure();

        debug!("Decoration mode set to {:?} for toplevel, configure event sent", mode);
    }

    fn request_mode(&mut self, toplevel: ToplevelSurface, requested_mode: Mode) {
        let configured_mode = self.get_configured_toplevel_decoration_mode();

        debug!("Client requested decoration mode {:?}, overriding with configured mode {:?}", requested_mode, configured_mode);

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(configured_mode);
        });

        toplevel.send_configure();

        debug!("Decoration mode set to {:?}, configure event sent", configured_mode);
    }

    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        let configured_mode = self.get_configured_toplevel_decoration_mode();

        debug!("Client requested to unset decoration mode, setting to configured mode {:?}", configured_mode);

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(configured_mode);
        });

        toplevel.send_configure();

        debug!("Decoration mode set to {:?}, configure event sent", configured_mode);
    }
}

smithay::delegate_xdg_decoration!(SmearorCompositor);
