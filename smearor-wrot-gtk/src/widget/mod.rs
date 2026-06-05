//! GTK4 compositor widget with OpenGL rendering

use crate::clipboard::error::CompositorClipboardError;
use crate::config::CompositorWidgetConfig;
use crate::widget::imp::ApplicationError;
use crate::widget::imp::CompositorWidgetImpl;
use crate::widget::imp::clipboard::handler::ClipboardHandler;
use crate::widget::socket::handler::SocketHandler;
use gtk4::glib;
use gtk4::prelude::WidgetExt;
use gtk4::subclass::prelude::*;
use smearor_wrot_core::SmearorCompositor;
use smearor_wrot_model::geometry::size::Size;

pub mod buffer;
pub mod color_mask;
pub mod commit;
pub mod compositor;
pub mod config;
pub mod dmabuf;
pub mod event;
pub mod imp;
pub mod resize;
pub mod shm;
pub mod shutdown;
pub mod size;
pub mod socket;
pub mod window_state;

glib::wrapper! {
    pub struct CompositorWidget(ObjectSubclass<CompositorWidgetImpl>)
        @extends gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl CompositorWidget {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn with_socket(socket: Option<String>) -> Self {
        let obj: Self = glib::Object::builder().build();
        if let Some(socket_path) = socket {
            obj.set_socket_path(socket_path);
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
        F: Fn(usize, f64, f64) -> (f64, f64) + 'static,
    {
        self.imp().set_touch_transform_callback(callback);
    }

    pub fn set_pointer_transform_callback<F>(&self, callback: F)
    where
        F: Fn(f64, f64) -> (f64, f64) + 'static,
    {
        self.imp().set_pointer_transform_callback(callback);
    }

    pub fn apply_touch_transform(&self, sequence: usize, x: f64, y: f64) -> (f64, f64) {
        self.imp().apply_touch_transform(sequence, x, y)
    }

    pub fn apply_pointer_transform(&self, x: f64, y: f64) -> (f64, f64) {
        self.imp().apply_pointer_transform(x, y)
    }

    pub fn show_touch_overlay(&self) {
        self.imp().show_touch_overlay();
    }

    pub fn hide_touch_overlay(&self) {
        self.imp().hide_touch_overlay();
    }

    pub fn update_touch_point(&self, sequence: usize, gtk_x: f64, gtk_y: f64, app_x: f64, app_y: f64) {
        self.imp().update_touch_point(sequence, gtk_x, gtk_y, app_x, app_y);
    }

    pub fn remove_touch_point(&self, sequence: usize) {
        self.imp().remove_touch_point(sequence);
    }

    pub fn update_pointer_point(&self, gtk_x: f64, gtk_y: f64, app_x: f64, app_y: f64) {
        self.imp().update_pointer_point(gtk_x, gtk_y, app_x, app_y);
    }

    pub fn clear_pointer_point(&self) {
        self.imp().clear_pointer_point();
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
