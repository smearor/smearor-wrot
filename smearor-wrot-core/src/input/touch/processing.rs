use crate::SmearorCompositor;
use crate::input::time::get_time;
use crate::surface::SurfaceQuery;
use smithay::input::touch::DownEvent;
use smithay::input::touch::MotionEvent;
use smithay::input::touch::UpEvent;
use smithay::utils::SERIAL_COUNTER;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;

pub trait TouchInputProcessing {
    fn process_gtk_touch_down(&mut self, sequence: usize, x: f64, y: f64);
    fn process_gtk_touch_up(&mut self, sequence: usize);
    fn process_gtk_touch_motion(&mut self, sequence: usize, x: f64, y: f64);
}

impl TouchInputProcessing for SmearorCompositor {
    fn process_gtk_touch_down(&mut self, sequence: usize, x: f64, y: f64) {
        debug!("Processing GTK touch down: sequence={}, x={}, y={}", sequence, x, y);

        let Some(output) = self.space.outputs().next() else {
            debug!("No output available for GTK touch down");
            return;
        };

        let Some(output_geo) = self.space.output_geometry(output) else {
            debug!("Failed to get output geometry for GTK touch down");
            return;
        };

        let Some(touch) = self.seat.get_touch() else {
            debug!("Touch not available for GTK touch down");
            return;
        };

        let touch_slot = self.touch_slot_manager.get_slot_for_sequence(sequence);
        let location: smithay::utils::Point<f64, smithay::utils::Logical> = (x, y).into();
        let pos = location + output_geo.loc.to_f64();
        let focus = self.surface_under(pos);

        let down_event = DownEvent {
            slot: touch_slot,
            location: pos,
            serial: SERIAL_COUNTER.next_serial(),
            time: get_time(),
        };

        touch.down(self, focus, &down_event);
        touch.frame(self);
    }

    fn process_gtk_touch_up(&mut self, sequence: usize) {
        debug!("Processing GTK touch up: sequence={}", sequence);

        let Some(touch) = self.seat.get_touch() else {
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

    fn process_gtk_touch_motion(&mut self, sequence: usize, x: f64, y: f64) {
        debug!("Processing GTK touch motion: sequence={}, x={}, y={}", sequence, x, y);

        let Some(output) = self.space.outputs().next() else {
            debug!("No output available for GTK touch motion");
            return;
        };

        let Some(output_geometry) = self.space.output_geometry(output) else {
            debug!("Failed to get output geometry for GTK touch motion");
            return;
        };

        let Some(touch) = self.seat.get_touch() else {
            debug!("Touch not available for GTK touch motion");
            return;
        };

        let touch_slot = self.touch_slot_manager.get_slot_for_sequence(sequence);
        let location: smithay::utils::Point<f64, smithay::utils::Logical> = (x, y).into();
        let pos = location + output_geometry.loc.to_f64();

        let motion_event = MotionEvent {
            slot: touch_slot,
            location: pos,
            time: get_time(),
        };

        let focus = self.surface_under(pos);

        touch.motion(self, focus, &motion_event);
        touch.frame(self);
    }
}

impl TouchInputProcessing for Arc<Mutex<SmearorCompositor>> {
    fn process_gtk_touch_down(&mut self, sequence: usize, x: f64, y: f64) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_touch_down(&mut *guard, sequence, x, y);
        }
    }

    fn process_gtk_touch_up(&mut self, sequence: usize) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_touch_up(&mut *guard, sequence);
        }
    }

    fn process_gtk_touch_motion(&mut self, sequence: usize, x: f64, y: f64) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_gtk_touch_motion(&mut *guard, sequence, x, y);
        }
    }
}
