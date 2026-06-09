//! Input handling for the compositor

pub mod keyboard;
pub mod mouse;
pub mod time;
pub mod touch;

use smithay::backend::input::AbsolutePositionEvent;
use smithay::backend::input::Axis;
use smithay::backend::input::AxisSource;
use smithay::backend::input::ButtonState;
use smithay::backend::input::Event;
use smithay::backend::input::InputBackend;
use smithay::backend::input::InputEvent;
use smithay::backend::input::KeyboardKeyEvent;
use smithay::backend::input::PointerAxisEvent;
use smithay::backend::input::PointerButtonEvent;
use smithay::input::keyboard::FilterResult;
use smithay::input::pointer::AxisFrame;
use smithay::input::pointer::ButtonEvent;
use smithay::input::pointer::MotionEvent;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::SERIAL_COUNTER;
use std::sync::Arc;
use std::sync::Mutex;

use crate::compositor::SmearorCompositor;
use crate::surface::query::SurfaceQuery;

use tracing::error;

/// Trait for input processing methods
pub trait InputProcessing {
    /// Process a generic Smithay input event
    fn process_input_event<I: InputBackend>(&mut self, event: InputEvent<I>);
}

impl InputProcessing for SmearorCompositor {
    fn process_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => {
                let serial = SERIAL_COUNTER.next_serial();
                let time = Event::time_msec(&event);

                if let Some(keyboard) = self.states.seat.get_keyboard() {
                    keyboard.input::<Self, _>(self, event.key_code(), event.state(), serial, time, |_, _, _| FilterResult::Forward);
                } else {
                    error!("Keyboard not available for input event");
                }
            }
            InputEvent::PointerMotion { .. } => {}
            InputEvent::PointerMotionAbsolute { event, .. } => {
                let Some(output) = self.space.outputs().next() else {
                    error!("No output available for pointer motion");
                    return;
                };

                let Some(output_geo) = self.space.output_geometry(output) else {
                    error!("Failed to get output geometry");
                    return;
                };

                let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();

                let serial = SERIAL_COUNTER.next_serial();

                let Some(pointer) = self.states.seat.get_pointer() else {
                    error!("Pointer not available for input event");
                    return;
                };

                let under = self.surface_under(pos);

                pointer.motion(
                    self,
                    under,
                    &MotionEvent {
                        location: pos,
                        serial,
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerButton { event, .. } => {
                let Some(pointer) = self.states.seat.get_pointer() else {
                    error!("Pointer not available for button event");
                    return;
                };

                let Some(keyboard) = self.states.seat.get_keyboard() else {
                    error!("Keyboard not available for button event");
                    return;
                };

                let serial = SERIAL_COUNTER.next_serial();

                let button = event.button_code();

                let button_state = event.state();

                if ButtonState::Pressed == button_state && !pointer.is_grabbed() {
                    if let Some((window, _loc)) = self.space.element_under(pointer.current_location()).map(|(w, l)| (w.clone(), l)) {
                        self.space.raise_element(&window, true);
                        if let Some(toplevel) = window.toplevel() {
                            keyboard.set_focus(self, Some(toplevel.wl_surface().clone()), serial);
                        }
                        self.space.elements().for_each(|window| {
                            if let Some(toplevel) = window.toplevel() {
                                toplevel.send_pending_configure();
                            }
                        });
                    } else {
                        self.space.elements().for_each(|window| {
                            window.set_activated(false);
                            if let Some(toplevel) = window.toplevel() {
                                toplevel.send_pending_configure();
                            }
                        });
                        keyboard.set_focus(self, Option::<WlSurface>::None, serial);
                    }
                };

                pointer.button(
                    self,
                    &ButtonEvent {
                        button,
                        state: button_state,
                        serial,
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerAxis { event, .. } => {
                let Some(pointer) = self.states.seat.get_pointer() else {
                    error!("Pointer not available for axis event");
                    return;
                };

                let source = event.source();

                let horizontal_amount = event
                    .amount(Axis::Horizontal)
                    .unwrap_or_else(|| event.amount_v120(Axis::Horizontal).unwrap_or(0.0) * 15.0 / 120.);
                let vertical_amount = event
                    .amount(Axis::Vertical)
                    .unwrap_or_else(|| event.amount_v120(Axis::Vertical).unwrap_or(0.0) * 15.0 / 120.);
                let horizontal_amount_discrete = event.amount_v120(Axis::Horizontal);
                let vertical_amount_discrete = event.amount_v120(Axis::Vertical);

                let mut frame = AxisFrame::new(event.time_msec()).source(source);
                if horizontal_amount != 0.0 {
                    frame = frame.value(Axis::Horizontal, horizontal_amount);
                    if let Some(discrete) = horizontal_amount_discrete {
                        frame = frame.v120(Axis::Horizontal, discrete as i32);
                    }
                }
                if vertical_amount != 0.0 {
                    frame = frame.value(Axis::Vertical, vertical_amount);
                    if let Some(discrete) = vertical_amount_discrete {
                        frame = frame.v120(Axis::Vertical, discrete as i32);
                    }
                }

                if source == AxisSource::Finger {
                    if event.amount(Axis::Horizontal) == Some(0.0) {
                        frame = frame.stop(Axis::Horizontal);
                    }
                    if event.amount(Axis::Vertical) == Some(0.0) {
                        frame = frame.stop(Axis::Vertical);
                    }
                }

                pointer.axis(self, frame);
                pointer.frame(self);
            }
            _ => {}
        }
    }
}

impl InputProcessing for Arc<Mutex<SmearorCompositor>> {
    fn process_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        if let Ok(mut guard) = self.lock() {
            SmearorCompositor::process_input_event(&mut *guard, event);
        }
    }
}
