//! Window configuration

use smithay::desktop::Window;

use crate::compositor::SmearorCompositor;

/// Trait for window configuration
pub trait WindowConfiguration {
    /// Configure a window with the given size
    fn configure_window(&mut self, window: &Window);

    /// Get the window geometry
    fn window_geometry(&self, window: &Window) -> Option<smithay::utils::Rectangle<i32, smithay::utils::Logical>>;

    /// Get the window location
    fn window_location(&self, window: &Window) -> Option<smithay::utils::Point<i32, smithay::utils::Logical>>;

    /// Set the window location
    fn set_window_location(&mut self, window: &Window, location: smithay::utils::Point<i32, smithay::utils::Logical>);

    /// Get the window size
    fn window_size(&self, window: &Window) -> Option<smithay::utils::Size<i32, smithay::utils::Logical>>;
}

impl WindowConfiguration for SmearorCompositor {
    fn configure_window(&mut self, window: &Window) {
        if let Some(toplevel) = window.toplevel() {
            toplevel.send_pending_configure();
        }
    }

    fn window_geometry(&self, window: &Window) -> Option<smithay::utils::Rectangle<i32, smithay::utils::Logical>> {
        self.space.element_geometry(window)
    }

    fn window_location(&self, window: &Window) -> Option<smithay::utils::Point<i32, smithay::utils::Logical>> {
        self.space.element_location(window)
    }

    fn set_window_location(&mut self, window: &Window, location: smithay::utils::Point<i32, smithay::utils::Logical>) {
        self.space.map_element(window.clone(), location, false);
    }

    fn window_size(&self, window: &Window) -> Option<smithay::utils::Size<i32, smithay::utils::Logical>> {
        self.window_geometry(window).map(|geo| geo.size)
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;
    use smithay::utils::Logical;

    #[test]
    fn test_window_configuration_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::configuration::WindowConfiguration;

        let _ = |c: &mut SmearorCompositor, w: &Window| {
            c.configure_window(w);
        };
    }

    #[test]
    fn test_window_geometry_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::configuration::WindowConfiguration;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.window_geometry(w);
        };
    }

    #[test]
    fn test_window_location_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::configuration::WindowConfiguration;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.window_location(w);
        };
    }

    #[test]
    fn test_set_window_location_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::configuration::WindowConfiguration;

        let _ = |c: &mut SmearorCompositor, w: &Window, l: smithay::utils::Point<i32, Logical>| {
            c.set_window_location(w, l);
        };
    }

    #[test]
    fn test_window_size_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::configuration::WindowConfiguration;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.window_size(w);
        };
    }
}
