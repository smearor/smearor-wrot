//! Window activation and deactivation

use smithay::desktop::Window;

use crate::compositor::SmearorCompositor;

/// Trait for window activation and deactivation
pub trait WindowActivation {
    /// Activate a window
    fn activate_window(&mut self, window: &Window);

    /// Deactivate a window
    fn deactivate_window(&mut self, window: &Window);

    /// Deactivate all windows
    fn deactivate_all_windows(&mut self);
}

impl WindowActivation for SmearorCompositor {
    fn activate_window(&mut self, window: &Window) {
        window.set_activated(true);
        if let Some(toplevel) = window.toplevel() {
            toplevel.send_pending_configure();
        }
    }

    fn deactivate_window(&mut self, window: &Window) {
        window.set_activated(false);
        if let Some(toplevel) = window.toplevel() {
            toplevel.send_pending_configure();
        }
    }

    fn deactivate_all_windows(&mut self) {
        let windows: Vec<_> = self.space.elements().cloned().collect();
        for window in windows {
            self.deactivate_window(&window);
        }
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;

    #[test]
    fn test_window_activation_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::activation::WindowActivation;

        let _ = |c: &mut SmearorCompositor, w: &Window| {
            c.activate_window(w);
        };
    }

    #[test]
    fn test_window_deactivation_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::activation::WindowActivation;

        let _ = |c: &mut SmearorCompositor, w: &Window| {
            c.deactivate_window(w);
        };
    }

    #[test]
    fn test_window_deactivate_all_windows_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::activation::WindowActivation;

        let _ = |c: &mut SmearorCompositor| {
            c.deactivate_all_windows();
        };
    }
}
