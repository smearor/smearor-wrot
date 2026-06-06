//! Surface mapping operations

use smithay::desktop::Window;
use smithay::utils::Logical;

use crate::compositor::SmearorCompositor;

/// Trait for surface mapping operations
pub trait SurfaceMapping {
    /// Map a surface to the space at a specific location
    fn map_surface(&mut self, window: Window, location: smithay::utils::Point<i32, Logical>);
}

impl SurfaceMapping for SmearorCompositor {
    fn map_surface(&mut self, window: Window, location: smithay::utils::Point<i32, Logical>) {
        self.space.map_element(window, location, false);
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;
    use smithay::utils::Logical;

    #[test]
    fn test_surface_mapping_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::surface::mapping::SurfaceMapping;

        let _ = |c: &mut SmearorCompositor, w: Window, l: smithay::utils::Point<i32, Logical>| {
            c.map_surface(w, l);
        };
    }

    #[test]
    fn test_surface_mapping_location_point_creation() {
        let location = smithay::utils::Point::<i32, Logical>::new(100, 200);
        assert_eq!(location.x, 100);
        assert_eq!(location.y, 200);
    }
}
