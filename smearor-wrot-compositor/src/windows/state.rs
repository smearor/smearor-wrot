//! Window state queries

use smithay::desktop::Window;
use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State;

use crate::compositor::SmearorCompositor;

/// Trait for window state queries
pub trait WindowState {
    /// Check if a window is maximized
    fn is_window_maximized(&self, window: &Window) -> bool;

    /// Check if a window is fullscreen
    fn is_window_fullscreen(&self, window: &Window) -> bool;

    /// Check if a window is resizing
    fn is_window_resizing(&self, window: &Window) -> bool;

    /// Check if a window is activated
    fn is_window_activated(&self, window: &Window) -> bool;

    /// Check if a window is suspended
    fn is_window_suspended(&self, window: &Window) -> bool;

    /// Toggle maximize state of the first toplevel window
    fn toggle_maximize_first_toplevel(&mut self);

    /// Toggle fullscreen state of the first toplevel window
    fn toggle_fullscreen_first_toplevel(&mut self);
}

impl WindowState for SmearorCompositor {
    fn is_window_maximized(&self, window: &Window) -> bool {
        window
            .toplevel()
            .map(|toplevel| {
                let state = toplevel.current_state();
                state.states.contains(State::Maximized)
            })
            .unwrap_or(false)
    }

    fn is_window_fullscreen(&self, window: &Window) -> bool {
        window
            .toplevel()
            .map(|toplevel| {
                let state = toplevel.current_state();
                state.states.contains(State::Fullscreen)
            })
            .unwrap_or(false)
    }

    fn is_window_resizing(&self, window: &Window) -> bool {
        window
            .toplevel()
            .map(|toplevel| {
                let state = toplevel.current_state();
                state.states.contains(State::Resizing)
            })
            .unwrap_or(false)
    }

    fn is_window_activated(&self, window: &Window) -> bool {
        window
            .toplevel()
            .map(|toplevel| {
                let state = toplevel.current_state();
                state.states.contains(State::Activated)
            })
            .unwrap_or(false)
    }

    fn is_window_suspended(&self, window: &Window) -> bool {
        window
            .toplevel()
            .map(|toplevel| {
                let state = toplevel.current_state();
                state.states.contains(State::Suspended)
            })
            .unwrap_or(false)
    }

    fn toggle_maximize_first_toplevel(&mut self) {
        if let Some(window) = self.space.elements().next() {
            if let Some(toplevel) = window.toplevel() {
                let is_maximized = self.is_window_maximized(window);
                if is_maximized {
                    toplevel.with_pending_state(|state| {
                        state.states.unset(State::Maximized);
                    });
                } else {
                    toplevel.with_pending_state(|state| {
                        state.states.set(State::Maximized);
                    });
                }
                toplevel.send_configure();
            }
        }
    }

    fn toggle_fullscreen_first_toplevel(&mut self) {
        if let Some(window) = self.space.elements().next() {
            if let Some(toplevel) = window.toplevel() {
                let is_fullscreen = self.is_window_fullscreen(window);
                if is_fullscreen {
                    toplevel.with_pending_state(|state| {
                        state.states.unset(State::Fullscreen);
                    });
                } else {
                    toplevel.with_pending_state(|state| {
                        state.states.set(State::Fullscreen);
                    });
                }
                toplevel.send_configure();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;

    #[test]
    fn test_window_state_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::state::WindowState;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.is_window_maximized(w);
        };
    }

    #[test]
    fn test_window_is_window_fullscreen_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::state::WindowState;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.is_window_fullscreen(w);
        };
    }

    #[test]
    fn test_window_is_window_resizing_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::state::WindowState;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.is_window_resizing(w);
        };
    }

    #[test]
    fn test_window_is_window_activated_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::state::WindowState;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.is_window_activated(w);
        };
    }

    #[test]
    fn test_window_is_window_suspended_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::state::WindowState;

        let _ = |c: &SmearorCompositor, w: &Window| {
            c.is_window_suspended(w);
        };
    }
}
