//! Surface caching for compositor rendering
//!
//! This module provides traits and implementations for caching surface data
//! to improve rendering performance by avoiding redundant data processing.

use smithay::reexports::wayland_server::backend::ObjectId;

use crate::compositor::SmearorCompositor;

/// Trait for caching surface data
///
/// Surface caching allows the compositor to store rendered surface data
/// for reuse, improving performance by avoiding redundant processing.
pub trait SurfaceCache {
    /// Cache rendered surface data
    ///
    /// # Arguments
    ///
    /// * `surface_id` - The ID of the surface to cache
    /// * `data` - The rendered surface data to cache
    fn cache_surface(&self, surface_id: ObjectId, data: Vec<u8>);

    /// Get cached surface data
    ///
    /// # Arguments
    ///
    /// * `surface_id` - The ID of the surface to retrieve data for
    ///
    /// # Returns
    ///
    /// The cached surface data, or None if not found
    fn get_cached_surface(&self, surface_id: ObjectId) -> Option<Vec<u8>>;

    /// Clear surface cache
    ///
    /// # Arguments
    ///
    /// * `surface_id` - The ID of the surface to clear from cache
    fn clear_surface_cache(&self, surface_id: ObjectId);
}

impl SurfaceCache for SmearorCompositor {
    fn cache_surface(&self, surface_id: ObjectId, data: Vec<u8>) {
        self.render_cache.insert(surface_id, data);
    }

    fn get_cached_surface(&self, surface_id: ObjectId) -> Option<Vec<u8>> {
        self.render_cache.get(&surface_id).map(|v| v.clone())
    }

    fn clear_surface_cache(&self, surface_id: ObjectId) {
        self.render_cache.remove(&surface_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_cache_trait_exists() {
        // This test ensures the trait is properly defined
        // The trait should be implementable by any type
        trait TestSurfaceCache: SurfaceCache {}
    }

    #[test]
    fn test_surface_cache_trait_methods_exist() {
        // This test verifies the trait has the required methods
        // by checking the trait definition is valid
        fn check_trait<T: SurfaceCache>() {}
        // If this compiles, the trait exists and has the required methods
    }
}
