//! Buffer damage tracking

use crate::compositor::SmearorCompositor;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Logical;
use smithay::utils::Point;
use smithay::utils::Rectangle;
use smithay::utils::Size;

/// Trait for buffer damage tracking
pub trait BufferTracking {
    /// Mark a surface as damaged in a specific region
    fn damage_surface(&mut self, surface: &WlSurface, damage: Rectangle<i32, Logical>);

    /// Mark entire surface as damaged
    fn damage_entire_surface(&mut self, surface: &WlSurface);

    /// Get the damage region for a surface
    fn get_damage_region(&self, surface: &WlSurface) -> Option<Rectangle<i32, Logical>>;

    /// Clear damage for a surface
    fn clear_damage(&mut self, surface: &WlSurface);
}

impl BufferTracking for SmearorCompositor {
    fn damage_surface(&mut self, surface: &WlSurface, damage: Rectangle<i32, Logical>) {
        let id = surface.id();
        self.damage_regions.insert(id, damage);
    }

    fn damage_entire_surface(&mut self, surface: &WlSurface) {
        let id = surface.id();
        let damage: Rectangle<i32, Logical> = Rectangle::new(Point::new(0, 0), Size::new(i32::MAX, i32::MAX));
        self.damage_regions.insert(id, damage);
    }

    fn get_damage_region(&self, surface: &WlSurface) -> Option<Rectangle<i32, Logical>> {
        let id = surface.id();
        self.damage_regions.get(&id).map(|v| *v)
    }

    fn clear_damage(&mut self, surface: &WlSurface) {
        let id = surface.id();
        self.damage_regions.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
    use smithay::utils::Point;
    use smithay::utils::Rectangle;
    use smithay::utils::Size;

    #[test]
    fn test_buffer_tracking_trait_exists() {
        use crate::buffer::tracking::BufferTracking;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, s: &WlSurface| {
            c.damage_surface(s, Rectangle::new(Point::new(0, 0), Size::new(100, 100)));
        };
    }

    #[test]
    fn test_damage_entire_surface_trait_exists() {
        use crate::buffer::tracking::BufferTracking;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, s: &WlSurface| {
            c.damage_entire_surface(s);
        };
    }

    #[test]
    fn test_get_damage_region_trait_exists() {
        use crate::buffer::tracking::BufferTracking;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &SmearorCompositor, s: &WlSurface| {
            c.get_damage_region(s);
        };
    }

    #[test]
    fn test_clear_damage_trait_exists() {
        use crate::buffer::tracking::BufferTracking;
        use crate::compositor::SmearorCompositor;

        let _ = |c: &mut SmearorCompositor, s: &WlSurface| {
            c.clear_damage(s);
        };
    }
}
