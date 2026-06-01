use crate::event_handler::mouse::MouseInputEventHandler;
use crate::event_handler::touch::TouchInputEventHandler;
use crate::widget::event::handler::InputEventHandler;
use crate::widget::imp::CompositorWidgetImpl;
use glib::Propagation;
use glib::object::ObjectExt;
use gtk4::gdk::ButtonEvent;
use gtk4::gdk::EventType;
use gtk4::gdk::TouchEvent;
use gtk4::prelude::WidgetExt;
use tracing::debug;

pub trait TouchEventSetup {
    fn setup_touch_events(&self, obj: &crate::widget::CompositorWidget);
}

impl CompositorWidgetImpl {
    fn get_sequence_id(sequence: gtk4::gdk::EventSequence) -> usize {
        sequence.as_ptr() as usize
    }
}

impl TouchEventSetup for CompositorWidgetImpl {
    fn setup_touch_events(&self, obj: &crate::widget::CompositorWidget) {
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
                                if let Some((x, y)) = native.translate_coordinates(&widget, raw_x, raw_y) {
                                    debug!("Touch begin/down (corrected): id={id}, raw_x={raw_x}, raw_y={raw_y}, x={x}, y={y}");
                                    widget.handle_touch_down(id, x, y);
                                    return Propagation::Stop;
                                } else {
                                    debug!("Touch begin/down (raw): id={id}, raw_x={raw_x}, raw_y={raw_y}");
                                    widget.handle_touch_down(id, raw_x, raw_y);
                                    return Propagation::Stop;
                                }
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
                                    debug!("Touch update/motion (corrected): id={id}, raw_x={raw_x}, raw_y={raw_y}, x={x}, y={y}");
                                    widget.handle_touch_motion(id, x, y);
                                } else {
                                    debug!("Touch update/motion (raw): id={id}, raw_x={raw_x}, raw_y={raw_y}");
                                    widget.handle_touch_motion(id, raw_x, raw_y);
                                }
                            }
                        }
                    }
                }
                EventType::TouchEnd => {
                    if let Some(_touch_event) = event.downcast_ref::<TouchEvent>() {
                        let id = Self::get_sequence_id(event.event_sequence());
                        debug!("Touch end/up {id}");
                        widget.handle_touch_up(id);
                    }
                }
                EventType::TouchCancel => {
                    if let Some(_touch_event) = event.downcast_ref::<TouchEvent>() {
                        let id = Self::get_sequence_id(event.event_sequence());
                        debug!("Touch end/up {id}");
                        widget.handle_touch_up(id);
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
