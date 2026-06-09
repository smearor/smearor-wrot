use crate::CompositorWidget;
use crate::event_handler::mouse::MouseInputEventHandler;
use crate::widget::event::handler::InputEventHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::Propagation;
use glib::object::ObjectExt;
use gtk4::EventControllerMotion;
use gtk4::EventControllerScroll;
use gtk4::EventControllerScrollFlags;
use gtk4::EventSequenceState;
use gtk4::GestureClick;
use gtk4::PropagationPhase;
use gtk4::prelude::EventControllerExt;
use gtk4::prelude::GestureExt;
use gtk4::prelude::GestureSingleExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_model_geometry::Position;
use tracing::debug;

impl CompositorWidgetImpl {
    pub(crate) fn setup_mouse_events(&self, obj: &CompositorWidget) {
        let gesture_click = GestureClick::new();
        // Use all buttons (without only the primary mouse button would be propagated)
        gesture_click.set_button(0);
        let widget_weak = obj.downgrade();
        gesture_click.connect_pressed(move |gesture, _n_press, x, y| {
            // Check if input is blocked (pie menu is open)
            let Some(widget) = widget_weak.upgrade() else {
                return;
            };
            if widget.is_input_blocked() {
                return;
            }

            debug!("Pointer pressed: x={x}, y={y}");
            let button = gesture.current_button();
            widget.handle_mouse_press(button);
        });
        let widget_weak = obj.downgrade();
        gesture_click.connect_released(move |gesture, _n_press, _x, _y| {
            // Check if input is blocked (pie menu is open)
            let Some(widget) = widget_weak.upgrade() else {
                return;
            };
            if widget.is_input_blocked() {
                return;
            }
            let button = gesture.current_button();
            widget.handle_mouse_release(button);
            gesture.set_state(EventSequenceState::Claimed);
        });
        obj.add_controller(gesture_click);

        let motion_controller = EventControllerMotion::new();
        motion_controller.set_propagation_phase(PropagationPhase::Capture);
        let widget_weak = obj.downgrade();
        motion_controller.connect_motion(move |_controller, x, y| {
            // Check if input is blocked (pie menu is open)
            let Some(widget) = widget_weak.upgrade() else {
                return;
            };
            if widget.is_input_blocked() {
                return;
            }
            widget.handle_mouse_motion(Position::new(x, y));
        });
        obj.add_controller(motion_controller);

        let scroll_controller = EventControllerScroll::new(EventControllerScrollFlags::BOTH_AXES);
        scroll_controller.set_propagation_phase(PropagationPhase::Capture);
        let widget_weak = obj.downgrade();
        scroll_controller.connect_scroll(move |_controller, dx, dy| {
            let Some(widget) = widget_weak.upgrade() else {
                return Propagation::Proceed;
            };
            if widget.is_input_blocked() {
                return Propagation::Stop;
            }

            widget.handle_mouse_wheel(dx, dy);
            Propagation::Proceed
        });
        obj.add_controller(scroll_controller);
    }
}
