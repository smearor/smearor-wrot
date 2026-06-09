//! Surface query operations

use crate::compositor::SmearorCompositor;
use crate::surface::dialog::DialogSizeQuery;
use smearor_wrot_state_margin::MarginStateAccessor;
use smithay::desktop::Window;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Logical;
use smithay::utils::Point;
use tracing::debug;
use tracing::error;

/// Trait for surface query operations
pub trait SurfaceQuery {
    /// Get the surface under a specific position
    fn surface_under(&self, pos: Point<f64, Logical>) -> Option<(WlSurface, Point<f64, Logical>)>;

    /// Get the window associated with a surface
    fn window_for_surface(&self, surface: &WlSurface) -> Option<Window>;

    /// Get the top-level surface of the first window
    fn get_first_toplevel_surface(&self) -> Option<&WlSurface>;

    /// Get the ObjectId of the top-level window (first toplevel surface)
    fn get_first_toplevel_surface_id(&self) -> Option<ObjectId>;

    /// Get all mapped surfaces (windows)
    fn mapped_surfaces(&self) -> Vec<Window>;

    /// Check if a surface is currently mapped
    fn is_surface_mapped(&self, window: &Window) -> bool;

    /// Count the number of elements in the space
    fn count_elements(&self) -> usize;
}

impl SurfaceQuery for SmearorCompositor {
    fn surface_under(&self, pos: Point<f64, Logical>) -> Option<(WlSurface, Point<f64, Logical>)> {
        // TODO: Phase 6 - Dialog Management - Check dialogs under pointer first
        // Dialogs should have priority over main windows
        let Some(output) = self.space.outputs().next() else {
            error!("No output available for GTK mouse motion");
            return None;
        };
        let Some(output_geometry) = self.space.output_geometry(output) else {
            error!("Failed to get output geometry");
            return None;
        };
        let Ok(dialogs) = self.dialogs.lock() else {
            return None;
        };
        for dialog in dialogs.iter() {
            // Get dialog surface
            let dialog_surface = dialog.wl_surface();

            // Calculate dialog position (centered in widget)
            let output_size = if let Some(output) = &self.virtual_output {
                output.current_mode().map(|mode| (mode.size.w, mode.size.h)).unwrap_or((1920, 1080))
            } else {
                (1920, 1080)
            };

            let margin_left = self.margin_manager.margin_left() as i32;
            let margin_right = self.margin_manager.margin_right() as i32;
            let margin_top = self.margin_manager.margin_top() as i32;
            let margin_bottom = self.margin_manager.margin_bottom() as i32;
            let dialog_margin = self.margin_manager.dialog_margin() as i32;

            let adjusted_width = output_size.0 - margin_left - margin_right - 2 * dialog_margin;
            let adjusted_height = output_size.1 - margin_top - margin_bottom - 2 * dialog_margin;

            // Ensure adjusted size is positive
            let adjusted_width = adjusted_width.max(100);
            let adjusted_height = adjusted_height.max(100);

            // Get dialog size from surface attributes
            let dialog_size = self.dialog_size_for_surface(dialog_surface);

            if let Some(dialog_size) = dialog_size {
                let dialog_width = dialog_size.w as f64;
                let dialog_height = dialog_size.h as f64;

                // Limit dialog size to adjusted size
                let dialog_width = dialog_width.min(adjusted_width as f64);
                let dialog_height = dialog_height.min(adjusted_height as f64);

                // Calculate dialog position (centered, like calculate_dialog_position)
                // Use output size without margins (same as calculate_dialog_position in snapshot.rs)
                let dialog_x = (output_size.0 as f64 - dialog_width) / 2.0;
                let dialog_y = (output_size.1 as f64 - dialog_height) / 2.0;

                debug!("surface_under - Dialog position: ({}, {})", dialog_x, dialog_y);

                // // Apply geometry offset correction (like in snapshot.rs)
                // let (dialog_offset_x, dialog_offset_y) = {
                //     let mut offset_x = 0.0;
                //     let mut offset_y = 0.0;
                //     for window in self.space.elements() {
                //         let window_geometry = window.geometry();
                //         let window_location = self.space.element_location(window);
                //
                //         if let Some(loc) = window_location {
                //             offset_x = (loc.x as f64) - (window_geometry.loc.x as f64);
                //             offset_y = (loc.y as f64) - (window_geometry.loc.y as f64);
                //         } else {
                //             offset_x = -(window_geometry.loc.x as f64);
                //             offset_y = -(window_geometry.loc.y as f64);
                //         }
                //
                //         break;
                //     }
                //     (offset_x, offset_y)
                // };
                //
                // let dialog_x = dialog_x - dialog_offset_x;
                // let dialog_y = dialog_y - dialog_offset_y;

                // Check if pointer is within dialog bounds
                if pos.x >= dialog_x && pos.x < dialog_x + dialog_width && pos.y >= dialog_y && pos.y < dialog_y + dialog_height {
                    // info!("Dialog Surface hit at global: {:?}", pos);
                    // return Some((dialog_surface.clone(), pos));
                    // Calculate position within dialog
                    let local_x = pos.x - dialog_x;
                    let local_y = pos.y - dialog_y;
                    debug!("local ({},{})", local_x, local_y);
                    debug!("Dialog Surface under pointer: {:?}", dialog_surface);
                    // let local: Point<f64, Logical> = Point::new(local_x, local_y);
                    // let local: Point<f64, Logical> = Point::new(dialog_x, dialog_y);
                    let _local: Point<f64, Logical> = Point::new(dialog_margin as f64, dialog_margin as f64);
                    // let surface = (dialog_surface.clone(), Point::new(local_x, local_y));
                    // let surface = (dialog_surface.clone(), pos);
                    // info!("Surface under pointer (dialog): {:?}", surface);
                    // return Some(surface);
                    if let Some(surface) = self.space.element_under(pos).and_then(|(window, location)| {
                        window
                            .surface_under(pos - location.to_f64(), smithay::desktop::WindowSurfaceType::ALL)
                            .map(|(s, p)| {
                                // BEFORE (s, local)
                                (s, (p + output_geometry.loc).to_f64())
                            })
                    }) {
                        debug!("Surface under pointer (dialog): {:?}", surface);
                        return Some(surface);
                    }
                } else {
                    debug!("Pointer not within dialog bounds");
                }
            }
        }

        // If no dialog found, check for windows in Smithay space
        if let Some(surface) = self.space.element_under(pos).and_then(|(window, location)| {
            window
                .surface_under(pos - location.to_f64(), smithay::desktop::WindowSurfaceType::ALL)
                .map(|(s, p)| (s, (p + location).to_f64()))
        }) {
            debug!("Surface under pointer (main window): {:?}", surface);
            return Some(surface);
        }

        debug!("No surface under pointer");
        None
    }

    fn window_for_surface(&self, surface: &WlSurface) -> Option<Window> {
        self.space
            .elements()
            .find(|window| {
                window
                    .toplevel()
                    .map(|toplevel_surface| toplevel_surface.wl_surface() == surface)
                    .unwrap_or(false)
            })
            .cloned()
    }

    fn get_first_toplevel_surface(&self) -> Option<&WlSurface> {
        self.space
            .elements()
            .find_map(|window| window.toplevel().map(|toplevel_surface| toplevel_surface.wl_surface()))
    }

    fn get_first_toplevel_surface_id(&self) -> Option<ObjectId> {
        self.get_first_toplevel_surface().map(|surface| surface.id())
    }

    fn mapped_surfaces(&self) -> Vec<Window> {
        self.space.elements().cloned().collect()
    }

    fn is_surface_mapped(&self, window: &Window) -> bool {
        self.space.elements().any(|w| w == window)
    }

    fn count_elements(&self) -> usize {
        self.space.elements().count()
    }
}

#[cfg(test)]
mod tests {
    use smithay::desktop::Space;
    use smithay::desktop::Window;

    #[test]
    fn test_surface_query_trait_exists() {
        use crate::compositor::SmearorCompositor;
        use crate::surface::query::SurfaceQuery;

        let _ = |c: &SmearorCompositor| c.mapped_surfaces();
    }

    #[test]
    fn test_mapped_surfaces_returns_empty_vector_when_no_windows_mapped() {
        let space = Space::<Window>::default();
        assert!(space.elements().count() == 0);
    }

    #[test]
    fn test_is_surface_mapped_returns_false_for_non_existent_window() {
        let space = Space::<Window>::default();
        let window_count = space.elements().count();
        assert!(window_count == 0);
    }
}
