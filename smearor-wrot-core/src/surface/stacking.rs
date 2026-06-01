//! Surface stacking operations

use smithay::desktop::Window;

use crate::compositor::SmearorCompositor;

/// Trait for surface stacking operations
pub trait SurfaceStacking {
    /// Raise a surface to the top of the stack
    fn raise_surface(&mut self, window: &Window);
}

impl SurfaceStacking for SmearorCompositor {
    fn raise_surface(&mut self, window: &Window) {
        self.space.raise_element(window, true);
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;

    #[test]
    fn test_surface_stacking_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::surface::stacking::SurfaceStacking;

        let _ = |c: &mut SmearorCompositor, w: &Window| {
            c.raise_surface(w);
        };
    }

    #[test]
    fn test_surface_stacking_window_reference_type() {
        let _ = |_: &Window| {};
    }
}
