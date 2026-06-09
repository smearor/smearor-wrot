use crate::SmearorCompositor;
use crate::input::keyboard::processing::KeyboardInputProcessing;
use crate::input::time::get_time;
use crate::surface::SurfaceQuery;
use smearor_wrot_model_geometry::Position;
use smithay::input::touch::DownEvent;
use smithay::input::touch::MotionEvent;
use smithay::input::touch::UpEvent;
use smithay::utils::Logical;
use smithay::utils::Point;
use smithay::utils::SERIAL_COUNTER;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;

pub trait TouchInputProcessing {
    fn process_gtk_touch_down(&mut self, sequence: usize, position: Position<f64>);
    fn process_gtk_touch_up(&mut self, sequence: usize);
    fn process_gtk_touch_motion(&mut self, sequence: usize, position: Position<f64>);
}

impl TouchInputProcessing for SmearorCompositor {
    fn process_gtk_touch_down(&mut self, sequence: usize, position: Position<f64>) {
        debug!("Processing GTK touch down: sequence={sequence}, position={position}");

        let Some(output) = self.space.outputs().next() else {
            debug!("No output available for GTK touch down");
            return;
        };

        let Some(output_geo) = self.space.output_geometry(output) else {
            debug!("Failed to get output geometry for GTK touch down");
            return;
        };

        let Some(touch) = self.states.seat.get_touch() else {
            debug!("Touch not available for GTK touch down");
            return;
        };
        let Some(pointer) = self.states.seat.get_pointer() else {
            debug!("Pointer not available for GTK touch down");
            return;
        };

        let touch_slot = self.touch_slot_manager.get_slot_for_sequence(sequence);
        let location: Point<f64, Logical> = position.into();
        let final_position = location + output_geo.loc.to_f64();
        let focused_surface = self.surface_under(final_position);

        if !pointer.is_grabbed() {
            if let Some((surface, _loc)) = &focused_surface {
                if let Some(window) = self.window_for_surface(surface) {
                    self.focus_window(&window);
                }
            }
        }
        let down_event = DownEvent {
            slot: touch_slot,
            location: final_position,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        };

        touch.down(self, focused_surface, &down_event);
        touch.frame(self);
    }

    fn process_gtk_touch_up(&mut self, sequence: usize) {
        debug!("Processing GTK touch up: sequence={}", sequence);

        let Some(touch) = self.states.seat.get_touch() else {
            debug!("Touch not available for GTK touch up");
            return;
        };

        let touch_slot = self.touch_slot_manager.get_slot_for_sequence(sequence);

        let up_event = UpEvent {
            slot: touch_slot,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        };

        touch.up(self, &up_event);
        touch.frame(self);

        // Remove the slot when the touch sequence ends
        self.touch_slot_manager.remove_slot_for_sequence(sequence);
    }

    fn process_gtk_touch_motion(&mut self, sequence: usize, position: Position<f64>) {
        debug!("Processing GTK touch motion: sequence={sequence}, positionx={position}");

        let Some(output) = self.space.outputs().next() else {
            debug!("No output available for GTK touch motion");
            return;
        };

        let Some(output_geometry) = self.space.output_geometry(output) else {
            debug!("Failed to get output geometry for GTK touch motion");
            return;
        };

        let Some(touch) = self.states.seat.get_touch() else {
            debug!("Touch not available for GTK touch motion");
            return;
        };

        let touch_slot = self.touch_slot_manager.get_slot_for_sequence(sequence);
        let location: Point<f64, Logical> = position.into();
        let final_position = location + output_geometry.loc.to_f64();

        let motion_event = MotionEvent {
            slot: touch_slot,
            location: final_position,
            time: get_time(),
        };

        let focus = self.surface_under(final_position);

        touch.motion(self, focus, &motion_event);
        touch.frame(self);
    }
}

impl TouchInputProcessing for Arc<Mutex<SmearorCompositor>> {
    fn process_gtk_touch_down(&mut self, sequence: usize, position: Position<f64>) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_touch_down(&mut *guard, sequence, position);
        }
    }

    fn process_gtk_touch_up(&mut self, sequence: usize) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_touch_up(&mut *guard, sequence);
        }
    }

    fn process_gtk_touch_motion(&mut self, sequence: usize, position: Position<f64>) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_touch_motion(&mut *guard, sequence, position);
        }
    }
}
