use crate::SmearorCompositor;
use crate::surface::SurfaceQuery;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Physical;
use smithay::utils::Point;
use smithay::utils::Rectangle;
use smithay::utils::Size;

/// Trait for tracking damage on Wayland surfaces
///
/// Surface-level damage tracking allows the compositor to identify which
/// regions of individual surfaces have changed, enabling more granular
/// damage tracking than output-level tracking alone.
pub trait SurfaceDamage {
    /// Mark a region of a surface as damaged
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface to mark as damaged
    /// * `region` - The region to mark as damaged, or None to mark the entire surface
    fn mark_surface_damage(&mut self, surface: &WlSurface, region: Option<Rectangle<i32, Physical>>);

    /// Get the damage regions for a surface
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface to get damage regions for
    ///
    /// # Returns
    ///
    /// A vector of damage regions for the specified surface
    fn get_surface_damage(&self, surface: &WlSurface) -> Vec<Rectangle<i32, Physical>>;

    /// Resolve the surface to clear damage for and clear the surface damage
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface to clear damage for
    fn resolve_surface_and_clear_surface_damage(&mut self);

    /// Clear all damage regions for a surface
    ///
    /// # Arguments
    ///
    /// * `surface` - The surface to clear damage for
    fn clear_surface_damage(&mut self, surface: &WlSurface);

    /// Get all surface damage regions across all surfaces
    ///
    /// # Returns
    ///
    /// A vector of all damage regions from all surfaces
    fn get_all_surface_damage(&self) -> Vec<Rectangle<i32, Physical>>;

    /// Merge overlapping or adjacent damage regions
    ///
    /// This optimization combines overlapping or adjacent rectangles into larger
    /// rectangles to reduce the number of draw calls and improve rendering efficiency.
    ///
    /// # Arguments
    ///
    /// * `regions` - A vector of damage regions to merge
    ///
    /// # Returns
    ///
    /// A vector of merged damage regions
    fn merge_damage_regions(&self, regions: Vec<Rectangle<i32, Physical>>) -> Vec<Rectangle<i32, Physical>>;
}

impl SurfaceDamage for SmearorCompositor {
    fn mark_surface_damage(&mut self, surface: &WlSurface, region: Option<Rectangle<i32, Physical>>) {
        if let Some(damage_rect) = region {
            self.surface_damage.entry(surface.clone()).or_default().push(damage_rect);
        } else {
            // Mark entire surface as damaged by storing a large rectangle
            let entire_damage = Rectangle::new(Point::new(0, 0), Size::new(i32::MAX, i32::MAX));
            self.surface_damage.insert(surface.clone(), vec![entire_damage]);
        }
    }

    fn get_surface_damage(&self, surface: &WlSurface) -> Vec<Rectangle<i32, Physical>> {
        self.surface_damage.get(surface).map(|v| v.value().clone()).unwrap_or_default()
    }

    fn resolve_surface_and_clear_surface_damage(&mut self) {
        if let Some(surface) = self.get_first_toplevel_surface() {
            self.surface_damage.remove(surface);
        }
    }

    fn clear_surface_damage(&mut self, surface: &WlSurface) {
        self.surface_damage.remove(surface);
    }

    fn get_all_surface_damage(&self) -> Vec<Rectangle<i32, Physical>> {
        let mut all_damage = Vec::new();
        for entry in self.surface_damage.iter() {
            all_damage.extend(entry.value().clone());
        }
        all_damage
    }

    fn merge_damage_regions(&self, regions: Vec<Rectangle<i32, Physical>>) -> Vec<Rectangle<i32, Physical>> {
        if regions.is_empty() {
            return regions;
        }

        // Simple merging strategy: if regions are small and numerous, merge them into a bounding box
        // For now, we'll use a simple bounding box approach
        // TODO: Phase 5 - Implement more sophisticated merging algorithm (e.g., sweep line algorithm)

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for region in &regions {
            let loc = region.loc;
            let size = region.size;

            min_x = min_x.min(loc.x);
            min_y = min_y.min(loc.y);
            max_x = max_x.max(loc.x + size.w);
            max_y = max_y.max(loc.y + size.h);
        }

        if min_x == i32::MAX || min_y == i32::MAX {
            return regions;
        }

        let merged_width = max_x - min_x;
        let merged_height = max_y - min_y;

        vec![Rectangle::new(Point::new(min_x, min_y), Size::new(merged_width, merged_height))]
    }
}
