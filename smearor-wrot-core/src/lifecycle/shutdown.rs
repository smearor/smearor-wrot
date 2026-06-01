use crate::SmearorCompositor;
use crate::message::compositor_message::CompositorMessage;
use crate::message::sender::CompositorMessageSender;
use tracing::debug;

pub trait ShutdownHandler {
    /// Check if there are no more toplevel surfaces (windows) and request shutdown if so
    fn check_and_request_shutdown(&mut self);
}

impl ShutdownHandler for SmearorCompositor {
    fn check_and_request_shutdown(&mut self) {
        // Check XdgShellState instead of Space, as Smithay manages surface lifecycle there
        let toplevel_count = self.xdg_shell_state.toplevel_surfaces().len();
        if toplevel_count == 0 {
            // Only shutdown if the compositor has ever had a surface
            if let Ok(flag) = self.has_had_surface.lock() {
                if *flag {
                    debug!("No more toplevels remaining, requesting compositor shutdown");
                    self.send_message(CompositorMessage::Shutdown);
                }
            }
        }
    }
}
