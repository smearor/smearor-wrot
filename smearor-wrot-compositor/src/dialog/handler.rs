use crate::SmearorCompositor;
use smithay::wayland::shell::xdg::ToplevelSurface;
use tracing::error;

pub trait DialogHandler {
    /// Returns all modal dialogs for rendering
    fn get_all_dialogs(&self) -> Vec<ToplevelSurface>;

    /// Returns true if there are any active dialogs
    fn has_active_dialogs(&self) -> bool;

    /// Calculates the dialog size based on the application window size
    /// Returns (width, height) for the dialog
    /// Limits the dialog size by subtracting dialog-margin from the application window dimensions
    fn calculate_dialog_size(&self, app_width: i32, app_height: i32, dialog_aspect_ratio: f32) -> (i32, i32);

    /// Calculates the dialog position to center it in the application window
    /// Returns (x, y) for the dialog
    fn calculate_dialog_position(&self, app_width: i32, app_height: i32, dialog_width: i32, dialog_height: i32) -> (i32, i32);
}

impl DialogHandler for SmearorCompositor {
    fn get_all_dialogs(&self) -> Vec<ToplevelSurface> {
        let Ok(dialogs) = self.dialogs.lock() else {
            error!("Failed to lock dialogs registry");
            return Vec::new();
        };
        dialogs.clone()
    }

    fn has_active_dialogs(&self) -> bool {
        !self.get_all_dialogs().is_empty()
    }

    fn calculate_dialog_size(&self, app_width: i32, app_height: i32, dialog_aspect_ratio: f32) -> (i32, i32) {
        let dialog_margin = self.get_dialog_margin() as i32;

        // Subtract dialog-margin from both sides
        let max_width = app_width - 2 * dialog_margin;
        let max_height = app_height - 2 * dialog_margin;

        // Ensure size is positive
        let max_width = max_width.max(100);
        let max_height = max_height.max(100);

        // Calculate dialog size based on original aspect ratio, limited by dialog-margin
        // The dialog keeps its original aspect ratio but is constrained to fit within the margin
        let dialog_aspect_ratio = dialog_aspect_ratio.max(0.1); // Avoid division by zero

        // Calculate size that fits within the margin while maintaining aspect ratio
        let dialog_width = (max_height as f32 * dialog_aspect_ratio) as i32;
        let dialog_height = max_height;

        // If width exceeds the margin, scale down by width instead
        if dialog_width > max_width {
            let dialog_width = max_width;
            let dialog_height = (max_width as f32 / dialog_aspect_ratio) as i32;
            (dialog_width, dialog_height)
        } else {
            (dialog_width, dialog_height)
        }
    }

    fn calculate_dialog_position(&self, app_width: i32, app_height: i32, dialog_width: i32, dialog_height: i32) -> (i32, i32) {
        let dialog_x = (app_width - dialog_width) / 2;
        let dialog_y = (app_height - dialog_height) / 2;
        (dialog_x, dialog_y)
    }
}
