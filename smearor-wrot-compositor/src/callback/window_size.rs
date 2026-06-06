use crate::SmearorCompositor;
use std::sync::Arc;

/// Callback for window size changes from application to compositor
pub type WindowSizeCallback = Arc<dyn Fn(i32, i32)>;

pub trait WindowSizeCallbackAware {
    /// Set the window size callback for application -> compositor size coupling
    fn set_window_size_callback(&self, callback: WindowSizeCallback);
}

impl WindowSizeCallbackAware for SmearorCompositor {
    fn set_window_size_callback(&self, callback: WindowSizeCallback) {
        if let Ok(mut cb) = self.window_size_callback.lock() {
            *cb = Some(callback);
        }
    }
}
