use crate::CompositorWidget;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::object::ObjectExt;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_core::windows::WindowFocus;

impl CompositorWidgetImpl {
    pub(crate) fn setup_focus_synchronization(&self, obj: &CompositorWidget) {
        let widget_weak = obj.downgrade();
        obj.connect_notify_local(Some("is-focused"), move |widget, _param| {
            if let Some(widget) = widget_weak.upgrade() {
                let is_focused = widget.is_focus();
                if let Some(compositor_mutex) = widget.imp().compositor.borrow().as_ref() {
                    if let Ok(mut compositor) = compositor_mutex.lock() {
                        if is_focused {
                            compositor.set_focus_to_active_window();
                        } else {
                            compositor.clear_focus();
                        }
                    }
                }
            }
        });
    }
}
