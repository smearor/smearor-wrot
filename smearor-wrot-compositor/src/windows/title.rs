use crate::SmearorCompositor;
use smithay::wayland::compositor::with_states;
use smithay::wayland::shell::xdg::XdgToplevelSurfaceData;
use tracing::debug;

pub trait WindowTitle {
    /// Get the title of the active client window
    fn get_active_window_title(&self) -> Option<String>;
}

impl WindowTitle for SmearorCompositor {
    fn get_active_window_title(&self) -> Option<String> {
        self.space.elements().next().and_then(|window| {
            window.toplevel().and_then(|toplevel| {
                with_states(toplevel.wl_surface(), |states| {
                    states
                        .data_map
                        .get::<XdgToplevelSurfaceData>()
                        .and_then(|toplevel_surface_data| toplevel_surface_data.lock().ok())
                        .and_then(|surface_role_attributes| {
                            let title = surface_role_attributes.title.clone();
                            debug!("Extracted window title: {:?}", title);
                            title
                        })
                })
            })
        })
    }
}
