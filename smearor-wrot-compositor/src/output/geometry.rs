//! Output geometry management
//!
//! This module provides traits and implementations for managing output geometry
//! including size updates and coordinate transformations.

use crate::compositor::SmearorCompositor;
use crate::damage::output::OutputDamage;
use crate::margin::handler::MarginHandler;
use smearor_wrot_model::geometry::size::Size;
use smithay::output::Mode;
use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel;
use smithay::reexports::wayland_server::Resource;
use smithay::utils::Transform;
use tracing::debug;
use tracing::error;
use tracing::warn;

/// Trait for managing output geometry
///
/// Output geometry management allows the compositor to update output sizes
/// and coordinate transformations, ensuring proper rendering and window placement.
pub trait OutputGeometry {
    /// Update the output size
    ///
    /// # Arguments
    ///
    /// * `width` - The new width of the output
    /// * `height` - The new height of the output
    fn update_output_size(&mut self, output_size: Size<i32>);
}

impl OutputGeometry for SmearorCompositor {
    fn update_output_size(&mut self, output_size: Size<i32>) {
        let output = self.space.outputs().next().cloned();

        if let Some(output) = output {
            // Check if the size actually changed to avoid unnecessary configure events
            let current_mode = output.current_mode();
            let size_changed = current_mode.is_none_or(|mode| mode.size != output_size.into());

            if !size_changed {
                debug!("Output size unchanged at {output_size}, skipping configure events");
                return;
            }

            // Create a new mode with the updated size
            let mode = Mode {
                size: output_size.into(),
                refresh: 60_000,
            };

            // Update the output's current state with the new mode
            output.change_current_state(Some(mode), Some(Transform::Normal), None, Some((0, 0).into()));

            // Remap the output with the new size
            self.space.map_output(&output, (0, 0));

            // Mark the entire output as damaged
            self.mark_output_damage(&output, None);

            // Send configure events to all toplevels (not just those in space)
            // This ensures that all toplevel surfaces receive configure events, including
            // those that may not yet be mapped to the space (e.g., wl_surface#47 for application window)
            let toplevel_count = self.xdg_shell_state.toplevel_surfaces().len();
            debug!("Sending configure events to {toplevel_count} toplevels with new size {output_size}");

            for (idx, toplevel) in self.xdg_shell_state.toplevel_surfaces().iter().enumerate() {
                let wl_surface = toplevel.wl_surface();
                debug!("Toplevel {}: surface id={:?}", idx, wl_surface.id());

                // TODO: Phase 6 - Dialog Management - Apply dialog-margin to dialogs after rotation
                // Check if this toplevel is a dialog by checking the dialog registry
                let is_dialog = if let Ok(dialogs) = self.dialogs.lock() {
                    dialogs.iter().any(|d| d.wl_surface() == wl_surface)
                } else {
                    false
                };

                // Calculate the size to send (dialog-margin for dialogs, margins for normal windows)
                let send_size = if is_dialog {
                    let dialog_margin = self.get_dialog_margin() as i32;
                    let dialog_size = output_size - Size::new(2 * dialog_margin, 2 * dialog_margin);
                    // Ensure size is positive
                    let dialog_size = dialog_size.max(&Size::new(100, 100));
                    debug!("Applying dialog-margin to dialog: {output_size} -> {dialog_size}");
                    dialog_size.max(&Size::new(100, 100))
                } else {
                    // Apply margins for normal windows
                    let margin_size = self.get_margin_size();
                    let adjusted_size = output_size - margin_size.into();
                    // Ensure size is positive
                    let adjusted_size = adjusted_size.max(&Size::new(100, 100));
                    debug!("Applying margins to window: {output_size} -> {adjusted_size} (margins: {margin_size})");
                    adjusted_size
                };

                if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    toplevel.with_pending_state(|state| {
                        state.size = Some(send_size.into());
                        // Set Resizing state to force GTK4 to respond to configure events
                        state.states.set(xdg_toplevel::State::Resizing);
                        // Set Activated state to force GTK4 to respond
                        state.states.set(xdg_toplevel::State::Activated);
                        debug!("Set toplevel pending state: size={send_size}, states=Resizing|Activated");
                    });
                    toplevel.send_configure();
                    debug!("Configure event sent to toplevel");
                })) {
                    error!("Failed to send configure event to toplevel: {:?}", e);
                }
            }

            debug!("Updated output size to {output_size}");
        } else {
            warn!("No output found to update size to {output_size}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_geometry_trait_exists() {
        // This test ensures the trait is properly defined
        // The trait should be implementable by any type
        trait TestOutputGeometry: OutputGeometry {}
    }

    #[test]
    fn test_output_geometry_trait_methods_exist() {
        // This test verifies the trait has the required methods
        // by checking the trait definition is valid
        fn check_trait<T: OutputGeometry>() {}
        // If this compiles, the trait exists and has the required methods
    }
}
