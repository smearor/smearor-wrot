use crate::CompositorWidget;
use crate::event_handler::mouse::MouseInputEventHandler;
use crate::event_handler::touch::TouchInputEventHandler;
use crate::widget::event::handler::InputEventHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::Propagation;
use glib::object::ObjectExt;
use gtk4::gdk::ButtonEvent;
use gtk4::gdk::EventType;
use gtk4::gdk::TouchEvent;
use gtk4::prelude::WidgetExt;
use smearor_wrot_model::Position;
use tracing::debug;

pub trait TouchEventSetup {
    fn setup_touch_events(&self, obj: &CompositorWidget);
}

impl CompositorWidgetImpl {
    fn get_sequence_id(sequence: gtk4::gdk::EventSequence) -> usize {
        sequence.as_ptr() as usize
    }
}

impl TouchEventSetup for CompositorWidgetImpl {
    fn setup_touch_events(&self, obj: &CompositorWidget) {
        let touch_controller = gtk4::EventControllerLegacy::new();
        let widget_weak = obj.downgrade();
        touch_controller.connect_event(move |_controller, event| {
            // Check if input is blocked (pie menu is open)
            let Some(widget) = widget_weak.upgrade() else {
                return Propagation::Proceed;
            };
            if widget.is_input_blocked() {
                return Propagation::Stop;
            }

            match event.event_type() {
                EventType::TouchBegin => {
                    if let Some(touch_event) = event.downcast_ref::<TouchEvent>() {
                        let id = Self::get_sequence_id(event.event_sequence());
                        if let Some((raw_x, raw_y)) = touch_event.position() {
                            if let Some(native) = widget.native() {
                                return if let Some((x, y)) = native.translate_coordinates(&widget, raw_x, raw_y) {
                                    debug!("Touch begin/down (corrected): id={id}, raw_x={raw_x}, raw_y={raw_y}, x={x}, y={y}");
                                    match widget.handle_touch_down(id, Position::new(x, y)) {
                                        Ok(_) => Propagation::Stop,
                                        Err(e) => {
                                            debug!("Failed to handle touch down event {e}");
                                            Propagation::Proceed
                                        }
                                    }
                                } else {
                                    debug!("Touch begin/down (raw): id={id}, raw_x={raw_x}, raw_y={raw_y}");
                                    match widget.handle_touch_down(id, Position::new(raw_x, raw_y)) {
                                        Ok(_) => Propagation::Stop,
                                        Err(e) => {
                                            debug!("Failed to handle raw touch down event {e}");
                                            Propagation::Proceed
                                        }
                                    }
                                };
                            }
                        }
                    }
                }
                EventType::TouchUpdate => {
                    if let Some(touch_event) = event.downcast_ref::<TouchEvent>() {
                        let id = Self::get_sequence_id(event.event_sequence());
                        if let Some((raw_x, raw_y)) = touch_event.position() {
                            if let Some(native) = widget.native() {
                                if let Some((x, y)) = native.translate_coordinates(&widget, raw_x, raw_y) {
                                    match widget.handle_touch_motion(id, Position::new(x, y)) {
                                        Ok(_) => {
                                            debug!("Corrected touch update/motion (corrected): id={id}, raw_x={raw_x}, raw_y={raw_y}, x={x}, y={y}");
                                        }
                                        Err(e) => {
                                            debug!("Failed to handle corrected touch update/motion: id={id}, raw_x={raw_x}, raw_y={raw_y}, x={x}, y={y}: {e}");
                                        }
                                    }
                                } else {
                                    match widget.handle_touch_motion(id, Position::new(raw_x, raw_y)) {
                                        Ok(_) => {
                                            debug!("Raw touch update/motion: id={id}, raw_x={raw_x}, raw_y={raw_y}");
                                        }
                                        Err(e) => {
                                            debug!("Failed to handle raw Touch update/motion: id={id}, raw_x={raw_x}, raw_y={raw_y}: {e}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                EventType::TouchEnd => {
                    if let Some(_touch_event) = event.downcast_ref::<TouchEvent>() {
                        let id = Self::get_sequence_id(event.event_sequence());
                        match widget.handle_touch_up(id) {
                            Ok(_) => {
                                debug!("Touch end/up {id}");
                            }
                            Err(e) => {
                                debug!("Failed to handle touch end event {id}: {e}");
                            }
                        }
                    }
                }
                EventType::TouchCancel => {
                    if let Some(_touch_event) = event.downcast_ref::<TouchEvent>() {
                        let id = Self::get_sequence_id(event.event_sequence());

                        match widget.handle_touch_up(id) {
                            Ok(_) => {
                                debug!("Canceled touch event {id}");
                            }
                            Err(e) => {
                                debug!("Failed to cancel touch event {id}: {e}");
                            }
                        }
                    }
                }
                EventType::ButtonPress => {
                    if let Some(button_event) = event.downcast_ref::<ButtonEvent>() {
                        let button = button_event.button();
                        if button_event.is_pointer_emulated() {
                            if let Some((raw_x, raw_y)) = button_event.position() {
                                if let Some(native) = widget.native() {
                                    if let Some((x, y)) = native.translate_coordinates(&widget, raw_x, raw_y) {
                                        debug!("Emulated button press (corrected): raw_x={raw_x}, raw_y={raw_y}, x={x}, y={y}");
                                        widget.handle_mouse_press(button);
                                    } else {
                                        debug!("Emulated button press (raw): raw_x={raw_x}, raw_y={raw_y}");
                                        widget.handle_mouse_press(button);
                                    }
                                }
                            }
                            return Propagation::Stop;
                        }
                    }
                }
                _ => {
                    // Ignore other events
                }
            }
            Propagation::Proceed
        });
        obj.add_controller(touch_controller);
    }
}
