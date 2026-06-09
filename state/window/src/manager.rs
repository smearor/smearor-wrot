use crate::WindowState;
use gtk4::ApplicationWindow;
use std::cell::RefCell;
use std::sync::Arc;
use typed_builder::TypedBuilder;

/// Manages the (outer) application window and layer shell.
#[derive(Debug, TypedBuilder)]
pub struct WindowManager {
    /// The (outer) application window
    parent_window: RefCell<Option<Arc<ApplicationWindow>>>,

    /// The window config
    state: Arc<WindowState>,
}

impl WindowManager {
    pub fn parent_window(&self) -> Option<Arc<ApplicationWindow>> {
        self.parent_window.borrow().clone()
    }

    pub fn update_header_bar_title(&self, title: &str) {}
}
