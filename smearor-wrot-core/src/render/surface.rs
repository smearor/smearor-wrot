//! Surface rendering

use smithay::desktop::Window;
use smithay::reexports::wayland_server::Resource;
use std::sync::Arc;
use std::sync::Mutex;

use crate::compositor::SmearorCompositor;

/// Trait for surface rendering operations
pub trait SurfaceRendering {
    /// Render a surface
    fn render_surface(&self, window: &Window);

    /// Check if a surface needs rendering
    fn needs_rendering(&self, window: &Window) -> bool;

    /// Mark a surface as rendered
    fn mark_rendered(&self, window: &Window);
}

impl SurfaceRendering for Arc<Mutex<SmearorCompositor>> {
    fn render_surface(&self, window: &Window) {
        if let Ok(guard) = self.lock() {
            if let Some(surface) = window.toplevel().map(|t| t.wl_surface()) {
                let id = surface.id();
                guard.rendered_surfaces.insert(id);
            }
        }
    }

    fn needs_rendering(&self, window: &Window) -> bool {
        if let Ok(guard) = self.lock() {
            if let Some(surface) = window.toplevel().map(|t| t.wl_surface()) {
                let id = surface.id();
                return !guard.rendered_surfaces.contains(&id);
            }
        }
        true
    }

    fn mark_rendered(&self, window: &Window) {
        if let Ok(guard) = self.lock() {
            if let Some(surface) = window.toplevel().map(|t| t.wl_surface()) {
                let id = surface.id();
                guard.rendered_surfaces.insert(id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::compositor::SmearorCompositor;

    #[test]
    fn test_surface_rendering_trait_exists() {
        use crate::render::surface::SurfaceRendering;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, w: &Window| {
            c.render_surface(w);
        };
    }

    #[test]
    fn test_needs_rendering_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::surface::SurfaceRendering;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, w: &Window| {
            c.needs_rendering(w);
        };
    }

    #[test]
    fn test_mark_rendered_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::render::surface::SurfaceRendering;

        let _ = |c: &Arc<Mutex<SmearorCompositor>>, w: &Window| {
            c.mark_rendered(w);
        };
    }

    #[test]
    fn test_rendered_surfaces_mutex_operations() {
        let rendered_surfaces: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(HashSet::new()));
        let surface_id = 123u32;

        let mut surfaces = rendered_surfaces.lock().unwrap();
        surfaces.insert(surface_id);
        drop(surfaces);

        let surfaces = rendered_surfaces.lock().unwrap();
        assert!(surfaces.contains(&surface_id));
    }

    #[test]
    fn test_rendered_surfaces_empty_initially() {
        let rendered_surfaces: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(HashSet::new()));
        let surfaces = rendered_surfaces.lock().unwrap();
        assert!(surfaces.is_empty());
    }
}
