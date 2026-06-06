//! GTK4 compositor widget

use crate::clipboard::error::CompositorClipboardError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::clipboard::handler::ClipboardHandler;
use crate::widget::imp::widget::ApplicationError;
use crate::widget::imp::widget::CompositorWidgetImpl;
use crate::widget::socket::handler::SocketHandler;
use glib::Object;
use gtk4::Accessible;
use gtk4::Buildable;
use gtk4::ConstraintTarget;
use gtk4::Widget;
use gtk4::glib;
use gtk4::prelude::WidgetExt;
use gtk4::subclass::prelude::*;
use smearor_wrot_model::Position;
use smearor_wrot_model::Socket;
use smearor_wrot_model::geometry::size::Size;

glib::wrapper! {
    pub struct CompositorWidget(ObjectSubclass<CompositorWidgetImpl>)
        @extends Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl CompositorWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_socket(socket: Option<Socket>) -> Self {
        let obj = Self::new();
        if let Some(socket) = socket {
            obj.initialize_socket(socket);
            obj.initialize_compositor();
        }
        obj
    }

    pub fn set_clipboard_content(&self, content: Option<String>) -> Result<(), CompositorClipboardError> {
        self.imp().set_clipboard_content(content)
    }

    pub fn get_clipboard_content(&self) -> Result<Option<String>, CompositorClipboardError> {
        self.imp().get_clipboard_content()
    }

    pub fn set_selection_from_host(&self, mime_types: Vec<String>) -> Result<(), CompositorClipboardError> {
        self.imp().set_selection_from_host(mime_types)
    }

    pub fn notify_window_mapped(&self) {
        // self.queue_draw();
    }

    pub fn request_render(&self) {
        self.imp().request_render();
    }

    pub fn request_render_force(&self) {
        self.imp().request_render_force();
    }

    pub fn set_header_bar_title(&self, title: &str) {
        self.imp().update_header_bar_title(title);
    }

    pub fn set_auto_resize_handling(&self, enabled: bool) {
        self.imp().set_auto_resize_handling(enabled);
    }

    pub fn set_touch_transform_callback<F>(&self, callback: F)
    where
        F: Fn(usize, Position<f64>) -> Position<f64> + 'static,
    {
        self.imp().set_touch_transform_callback(callback);
    }

    pub fn set_pointer_transform_callback<F>(&self, callback: F)
    where
        F: Fn(Position<f64>) -> Position<f64> + 'static,
    {
        self.imp().set_pointer_transform_callback(callback);
    }

    pub fn apply_touch_transform(&self, sequence: usize, position: Position<f64>) -> Position<f64> {
        self.imp().apply_touch_transform(sequence, position)
    }

    pub fn apply_pointer_transform(&self, position: Position<f64>) -> Position<f64> {
        self.imp().apply_pointer_transform(position)
    }

    pub fn set_application_error(&self, program_name: Option<String>) {
        let error = program_name.map(|name| ApplicationError::NotFound(name));
        self.imp().set_application_error(error);
    }

    pub fn set_application_not_specified(&self) {
        self.imp().set_application_error(Some(ApplicationError::NotSpecified));
    }
}

impl From<&CompositorWidget> for Size<i32> {
    fn from(widget: &CompositorWidget) -> Self {
        Size::new(widget.width(), widget.height())
    }
}
