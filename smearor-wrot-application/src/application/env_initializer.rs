use crate::config::env_vars::EnvironmentVariablesState;
use smearor_wrot_child_process::GDK_BACKEND;
use smearor_wrot_child_process::GDK_BACKEND_WAYLAND;
use smearor_wrot_child_process::GSK_RENDERER;
use smearor_wrot_child_process::GSK_RENDERER_NGL;
use smearor_wrot_child_process::WAYLAND_DEBUG;
use smearor_wrot_child_process::WAYLAND_DISPLAY;
use tracing::debug;

pub struct EnvInitializer {}
impl EnvInitializer {
    pub fn init_env_vars(config: &EnvironmentVariablesState) {
        debug!("Initializing environment variables for the parent process");
        unsafe {
            std::env::set_var(GDK_BACKEND, GDK_BACKEND_WAYLAND);
            std::env::set_var(GSK_RENDERER, GSK_RENDERER_NGL);
            if config.env_override_wayland_debug {
                debug!("Enabling WAYLAND_DEBUG");
                std::env::set_var(WAYLAND_DEBUG, "1");
            }
            if let Some(override_wayland_display) = config.env_override_wayland_display {
                debug!("Overriding WAYLAND_DISPLAY to: {override_wayland_display}");
                std::env::set_var(WAYLAND_DISPLAY, override_wayland_display);
            }
        }
    }
}
