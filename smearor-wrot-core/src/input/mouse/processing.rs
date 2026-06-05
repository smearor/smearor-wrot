use crate::SmearorCompositor;
use crate::input::keyboard::processing::KeyboardInputProcessing;
use crate::input::mouse::convert::GtkToSmithayMouseEventConverter;
use crate::surface::SurfaceQuery;
use smithay::backend::input::Axis;
use smithay::backend::input::AxisSource;
use smithay::input::pointer::AxisFrame;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;
use tracing::error;
use tracing::trace;

pub trait MouseInputProcessing {
    fn process_gtk_mouse_press(&mut self, button: u32);
    fn process_gtk_mouse_release(&mut self, button: u32);
    fn process_gtk_mouse_motion(&mut self, x: f64, y: f64);
    fn process_gtk_mouse_wheel(&mut self, dx: f64, dy: f64);
}

impl MouseInputProcessing for SmearorCompositor {
    fn process_gtk_mouse_press(&mut self, button: u32) {
        debug!("Processing GTK mouse press: button {}", button);

        let Some(pointer) = self.seat.get_pointer() else {
            error!("Pointer not available for GTK mouse press");
            return;
        };
        debug!("Pointer available, current location: {:?}", pointer.current_location());

        let button_event = Self::convert_gtk_mouse_press(button);
        debug!("Converted to Smithay button event: {:?}", button_event);

        // Check if a popup grab is active
        // Smithay's PopupManager manages grabs automatically
        // When a grab is active, events are automatically forwarded to the popup
        let popup_grab_active = pointer.is_grabbed();

        debug!("Popup grab active: {}", popup_grab_active);

        let pointer_location = pointer.current_location();

        debug!("GTK mouse press: pointer_location={:?}", pointer_location);

        if !popup_grab_active {
            // Normal focus logic only when no popup grab is active
            if let Some((window, _loc)) = self.space.element_under(pointer_location).map(|(w, l)| (w.clone(), l)) {
                trace!("Window found under pointer: {:?}", window);
                self.focus_window(&window);
            } else {
                trace!("No window under pointer, clearing focus");
                self.clear_focus();
            }
        }

        // Send button event to pointer
        // Smithay automatically forwards to the popup when grab is active
        pointer.button(self, &button_event);
        pointer.frame(self);
    }

    fn process_gtk_mouse_release(&mut self, button: u32) {
        let Some(pointer) = self.seat.get_pointer() else {
            error!("Pointer not available for GTK mouse release");
            return;
        };
        let button_event = Self::convert_gtk_mouse_release(button);
        pointer.button(self, &button_event);
        pointer.frame(self);
    }

    fn process_gtk_mouse_motion(&mut self, x: f64, y: f64) {
        let Some(output) = self.space.outputs().next() else {
            error!("No output available for GTK mouse motion");
            return;
        };
        let Some(output_geometry) = self.space.output_geometry(output) else {
            error!("Failed to get output geometry");
            return;
        };
        let Some(pointer) = self.seat.get_pointer() else {
            error!("Pointer not available for GTK mouse motion");
            return;
        };
        let mut motion_event = Self::convert_gtk_mouse_motion(x, y);
        let position = motion_event.location + output_geometry.loc.to_f64();

        debug!("GTK mouse motion: x={}, y={}, final_position={:?}", x, y, position);

        // Check if a popup grab is active
        // Smithay's PopupManager manages grabs automatically
        // When a grab is active, events are automatically forwarded to the popup
        let popup_grab_active = pointer.is_grabbed();

        debug!("Popup grab active: {}", popup_grab_active);

        let under = self.surface_under(position);

        // Dialog logic only when no popup grab is active
        if !popup_grab_active {
            let mut has_dialog = false;
            let mut pointer_on_dialog = false;
            if let Ok(dialogs) = self.dialogs.lock() {
                if !dialogs.is_empty() {
                    has_dialog = true;
                    pointer_on_dialog = under.as_ref().map(|(s, _)| dialogs.iter().any(|d| d.wl_surface() == s)).unwrap_or(false);
                }
            }

            debug!("Pointer on dialog: {}", pointer_on_dialog);
            if has_dialog && !pointer_on_dialog {
                pointer.frame(self);
                debug!("Pointer is outside dialog bounds, don't send event to dialog");
                return;
            }
            if pointer_on_dialog {
                let dialog_margin = self.get_dialog_margin() as i32;
                motion_event.location.x = motion_event.location.x - output_geometry.loc.x as f64 - dialog_margin as f64;
                motion_event.location.y = motion_event.location.y - output_geometry.loc.y as f64 - dialog_margin as f64;
                debug!("motion event ({}, {})", motion_event.location.x, motion_event.location.y);
            }
        }

        // Send motion event to pointer
        // Smithay automatically forwards to the popup when grab is active
        pointer.motion(self, under, &motion_event);
        pointer.frame(self);
    }

    fn process_gtk_mouse_wheel(&mut self, dx: f64, dy: f64) {
        debug!("Processing GTK mouse wheel: dx={}, dy={}", dx, dy);

        let Some(pointer) = self.seat.get_pointer() else {
            error!("Pointer not available for GTK mouse wheel");
            return;
        };

        let time_msec = self.start_time.elapsed().as_millis() as u32;

        // Scroll factor to increase scroll speed
        let scroll_factor = 3.0;
        let dx = dx * scroll_factor;
        let dy = dy * scroll_factor;

        // Create axis frame with mouse wheel source
        let mut frame = AxisFrame::new(time_msec).source(AxisSource::Wheel);

        // Add horizontal scroll if dx is non-zero
        if dx != 0.0 {
            frame = frame.value(Axis::Horizontal, dx);
            frame = frame.v120(Axis::Horizontal, (dx * 120.0 / 15.0) as i32);
        }

        // Add vertical scroll if dy is non-zero
        if dy != 0.0 {
            frame = frame.value(Axis::Vertical, dy);
            frame = frame.v120(Axis::Vertical, (dy * 120.0 / 15.0) as i32);
        }

        debug!("Sending axis frame to pointer");
        pointer.axis(self, frame);
        pointer.frame(self);
    }
}

// TODO: Phase 6 - Dialog Management - Check if pointer is within dialog bounds
/// Check if the pointer position is within the active dialog bounds
impl SmearorCompositor {
    // fn is_pointer_within_dialog(&self, pos: Point<f64, Logical>) -> bool {
    //     let Ok(dialogs) = self.dialogs.lock() else {
    //         return false;
    //     };
    //     debug!("Checking if pointer {:?} is within dialog bounds", pos);
    //     for dialog in dialogs.iter() {
    //         // Get dialog size from surface attributes
    //         let dialog_size = self.dialog_size_for_surface(dialog.wl_surface());
    //
    //         if let Some(dialog_size) = dialog_size {
    //             let dialog_width = dialog_size.w as f64;
    //             let dialog_height = dialog_size.h as f64;
    //
    //             debug!("Dialog size: {}x{}", dialog_width, dialog_height);
    //
    //             // Calculate dialog position (centered)
    //             let output_size = if let Some(output) = &self.virtual_output {
    //                 output.current_mode().map(|mode| (mode.size.w, mode.size.h)).unwrap_or((1920, 1080))
    //             } else {
    //                 (1920, 1080)
    //             };
    //
    //             debug!("Output size: {:?}", output_size);
    //
    //             let margin_left = self.get_margin_left() as i32;
    //             let margin_right = self.get_margin_right() as i32;
    //             let margin_top = self.get_margin_top() as i32;
    //             let margin_bottom = self.get_margin_bottom() as i32;
    //
    //             let dialog_margin = self.get_dialog_margin() as i32;
    //
    //             debug!(
    //                 "Margins: left={}, right={}, top={}, bottom={}, dialog_margin={}",
    //                 margin_left, margin_right, margin_top, margin_bottom, dialog_margin
    //             );
    //
    //             let adjusted_width = output_size.0 - margin_left - margin_right - 2 * dialog_margin;
    //             let adjusted_height = output_size.1 - margin_top - margin_bottom - 2 * dialog_margin;
    //
    //             // Ensure adjusted size is positive
    //             let adjusted_width = adjusted_width.max(100);
    //             let adjusted_height = adjusted_height.max(100);
    //
    //             debug!("Adjusted size: {}x{}", adjusted_width, adjusted_height);
    //
    //             // Limit dialog size to adjusted size
    //             let dialog_width = dialog_width.min(adjusted_width as f64);
    //             let dialog_height = dialog_height.min(adjusted_height as f64);
    //
    //             debug!("Limited dialog size: {}x{}", dialog_width, dialog_height);
    //
    //             // Calculate dialog position (centered, like calculate_dialog_position)
    //             // Use output size without margins (same as calculate_dialog_position in snapshot.rs)
    //             let dialog_x = (output_size.0 as f64 - dialog_width) / 2.0;
    //             let dialog_y = (output_size.1 as f64 - dialog_height) / 2.0;
    //
    //             // // Apply geometry offset correction (like in snapshot.rs)
    //             // let (dialog_offset_x, dialog_offset_y) = {
    //             //     let mut offset_x = 0.0;
    //             //     let mut offset_y = 0.0;
    //             //     for window in self.space.elements() {
    //             //         let window_geometry = window.geometry();
    //             //         let window_location = self.space.element_location(window);
    //             //
    //             //         info!("Window geometry: {:?}", window_geometry);
    //             //         info!("Window location: {:?}", window_location);
    //             //
    //             //         if let Some(loc) = window_location {
    //             //             offset_x = (loc.x as f64) - (window_geometry.loc.x as f64);
    //             //             offset_y = (loc.y as f64) - (window_geometry.loc.y as f64);
    //             //         } else {
    //             //             offset_x = (window_geometry.loc.x as f64);
    //             //             offset_y = (window_geometry.loc.y as f64);
    //             //         }
    //             //
    //             //         info!("Geometry offset: ({}, {})", offset_x, offset_y);
    //             //         break;
    //             //     }
    //             //     (offset_x, offset_y)
    //             // };
    //             //
    //             // let dialog_x = dialog_x - dialog_offset_x;
    //             // let dialog_y = dialog_y - dialog_offset_y;
    //
    //             debug!("Dialog position: ({}, {})", dialog_x, dialog_y);
    //             debug!("Pointer position: ({}, {})", pos.x, pos.y);
    //
    //             // Check if pointer is within dialog bounds
    //             let is_within = pos.x >= dialog_x && pos.x < dialog_x + dialog_width && pos.y >= dialog_y && pos.y < dialog_y + dialog_height;
    //             debug!("Is within dialog bounds: {}", is_within);
    //             if is_within {
    //                 return true;
    //             }
    //         } else {
    //             debug!("Dialog size is None");
    //         }
    //     }
    //     debug!("Pointer is not within any dialog bounds");
    //     false
    // }

    // fn get_dialog_offset(&self) -> Option<Point<f64, Logical>> {
    //     if let Ok(dialogs) = self.dialogs.lock() {
    //         for dialog in dialogs.iter() {
    //             let dialog_size = self.dialog_size_for_surface(dialog.wl_surface());
    //             if let Some(dialog_size) = dialog_size {
    //                 let dialog_width = dialog_size.w as f64;
    //                 let dialog_height = dialog_size.h as f64;
    //
    //                 // Calculate dialog position (centered)
    //                 let output_size = if let Some(output) = &self.virtual_output {
    //                     output.current_mode().map(|mode| (mode.size.w, mode.size.h)).unwrap_or((1920, 1080))
    //                 } else {
    //                     (1920, 1080)
    //                 };
    //
    //                 let margin_left = self.get_margin_left() as i32;
    //                 let margin_right = self.get_margin_right() as i32;
    //                 let margin_top = self.get_margin_top() as i32;
    //                 let margin_bottom = self.get_margin_bottom() as i32;
    //
    //                 let dialog_margin = self.get_dialog_margin() as i32;
    //
    //                 let adjusted_width = output_size.0 - margin_left - margin_right - 2 * dialog_margin;
    //                 let adjusted_height = output_size.1 - margin_top - margin_bottom - 2 * dialog_margin;
    //
    //                 // Ensure adjusted size is positive
    //                 let adjusted_width = adjusted_width.max(100);
    //                 let adjusted_height = adjusted_height.max(100);
    //
    //                 // Limit dialog size to adjusted size
    //                 let dialog_width = dialog_width.min(adjusted_width as f64);
    //                 let dialog_height = dialog_height.min(adjusted_height as f64);
    //
    //                 let dialog_x = ((adjusted_width as f64 - dialog_width) / 2.0) + margin_left as f64 + dialog_margin as f64;
    //                 let dialog_y = ((adjusted_height as f64 - dialog_height) / 2.0) + margin_top as f64 + dialog_margin as f64;
    //
    //                 return Some(Point::from((dialog_x, dialog_y)));
    //             }
    //         }
    //     }
    //     None
    // }
}

impl MouseInputProcessing for Arc<Mutex<SmearorCompositor>> {
    fn process_gtk_mouse_press(&mut self, button: u32) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_mouse_press(&mut *guard, button);
        }
    }

    fn process_gtk_mouse_release(&mut self, button: u32) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_mouse_release(&mut *guard, button);
        }
    }

    fn process_gtk_mouse_motion(&mut self, x: f64, y: f64) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_mouse_motion(&mut *guard, x, y);
        }
    }

    fn process_gtk_mouse_wheel(&mut self, dx: f64, dy: f64) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_mouse_wheel(&mut *guard, dx, dy);
        }
    }
}
