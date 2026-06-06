//! Window close handling

use smithay::desktop::Window;

use crate::compositor::SmearorCompositor;

/// Trait for window close handling
pub trait WindowClose {
    /// Request a window to close
    fn close_window(&mut self, window: &Window);
}

impl WindowClose for SmearorCompositor {
    fn close_window(&mut self, window: &Window) {
        if let Some(toplevel) = window.toplevel() {
            toplevel.send_close();
        }
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Window;

    #[test]
    fn test_window_close_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::windows::close::WindowClose;

        let _ = |c: &mut SmearorCompositor, w: &Window| {
            c.close_window(w);
        };
    }
}
