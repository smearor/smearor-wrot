use crate::CompositorWidget;
use crate::widget::window_state::error::ChangeWindowStateError;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait WindowStateHandler {
    /// Toggles the maximized state of the first toplevel window.
    fn toggle_maximize_first_toplevel(&self) -> Result<(), ChangeWindowStateError>;

    /// Toggles the fullscreen state of the first toplevel window.
    fn toggle_fullscreen_first_toplevel(&self) -> Result<(), ChangeWindowStateError>;
}

impl WindowStateHandler for CompositorWidget {
    fn toggle_maximize_first_toplevel(&self) -> Result<(), ChangeWindowStateError> {
        self.imp().toggle_maximize_first_toplevel()
    }

    fn toggle_fullscreen_first_toplevel(&self) -> Result<(), ChangeWindowStateError> {
        self.imp().toggle_fullscreen_first_toplevel()
    }
}
