use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

pub struct SubsurfaceData {
    /// The parent surface
    pub parent: WlSurface,

    /// The subsurface
    pub subsurface: WlSurface,
}

impl SubsurfaceData {
    pub fn new(parent: &WlSurface, subsurface: &WlSurface) -> Self {
        Self {
            parent: parent.clone(),
            subsurface: subsurface.clone(),
        }
    }
}
