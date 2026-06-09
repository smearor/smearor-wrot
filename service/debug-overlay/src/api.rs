use smearor_wrot_model_geometry::Position;

pub trait DebugOverlayManager {
    /// Returns true, if the debug pointer overlay is enabled
    fn is_debug_pointer_enabled(&self) -> bool;

    /// Shows the debug pointer overlay
    fn show_debug_pointer(&self);

    /// Hides the debug pointer overlay
    fn hide_debug_pointer(&self);

    /// Updates the pointer point
    fn update_pointer_point(&self, gtk_position: Position<f32>, app_position: Position<f32>);

    /// Clears the pointer point
    fn clear_pointer_point(&self);

    /// Returns true, if the debug touch overlay is enabled
    fn is_debug_touch_enabled(&self) -> bool;

    /// Shows the debug touch overlay
    fn show_debug_touch_overlay(&self);

    /// Hides the debug touch overlay
    fn hide_debug_touch_overlay(&self);

    /// Updates a touch point with the given sequence id, GTK position and application position
    fn update_touch_point(&self, sequence: usize, gtk_position: Position<f32>, app_position: Position<f32>);

    /// Removes a touch point by sequence id
    fn remove_touch_point(&self, sequence: usize);

    /// Clears all touch points
    fn clear_touch_points(&self);
}
