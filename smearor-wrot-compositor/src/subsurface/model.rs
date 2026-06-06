use smearor_wrot_model::geometry::position::Position;
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

pub struct SubsurfacePositionData {
    /// The parent surface
    pub parent: WlSurface,

    /// The subsurface
    pub subsurface: WlSurface,

    /// The position of the subsurface
    pub position: Position<i32>,
}

impl SubsurfacePositionData {
    pub fn new(parent: &WlSurface, subsurface: &WlSurface, position: &Position<i32>) -> Self {
        Self {
            parent: parent.clone(),
            subsurface: subsurface.clone(),
            position: *position,
        }
    }
}
