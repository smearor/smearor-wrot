use gtk4::gdk;
use gtk4::glib::Bytes;
use gtk4::prelude::GtkWindowExt;
use tracing::error;

const ICON_DATA: &[u8] = include_bytes!("../assets/icon.svg");

pub fn set_program_icon(window: &gtk4::ApplicationWindow) {
    let _bytes = Bytes::from_static(ICON_DATA);
    let Some(display) = gdk::Display::default() else {
        error!("Failed to get default display");
        return;
    };
    let icon_theme = gtk4::IconTheme::for_display(&display);
    icon_theme.add_resource_path("/org/smearor/wrot/icons");
    window.set_icon_name(Some("io.smearor.wrot"));
}
