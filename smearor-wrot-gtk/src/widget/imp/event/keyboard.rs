use crate::CompositorWidget;
use crate::event_handler::keyboard::KeyboardInputEventHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::Propagation;
use glib::object::ObjectExt;
use gtk4::EventControllerKey;
use gtk4::prelude::WidgetExt;

impl CompositorWidgetImpl {
    pub(crate) fn setup_keyboard_events(&self, obj: &CompositorWidget) {
        obj.set_focusable(true);
        obj.set_can_target(true);
        obj.set_focus_on_click(true);
        obj.grab_focus();

        let key_controller = EventControllerKey::new();
        let widget_weak = obj.downgrade();
        key_controller.connect_key_pressed(move |_controller, keyval, keycode, _state| {
            if let Some(widget) = widget_weak.upgrade() {
                widget.handle_key_press(keyval, keycode);
            }
            Propagation::Stop
        });
        let widget_weak = obj.downgrade();
        key_controller.connect_key_released(move |_controller, keyval, keycode, _state| {
            if let Some(widget) = widget_weak.upgrade() {
                widget.handle_key_release(keyval, keycode);
            }
        });
        obj.add_controller(key_controller);
    }
}
