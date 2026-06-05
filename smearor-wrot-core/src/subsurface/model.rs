use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Logical;
use smithay::utils::Point;

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
    pub position: Point<i32, Logical>,
}

impl SubsurfacePositionData {
    pub fn new(parent: &WlSurface, subsurface: &WlSurface, position: &Point<i32, Logical>) -> Self {
        Self {
            parent: parent.clone(),
            subsurface: subsurface.clone(),
            position: *position,
        }
    }
}
