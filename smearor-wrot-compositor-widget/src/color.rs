use gtk4::gdk;
use smearor_wrot_model::color::rgba::RgbaColor;

pub fn into_gdk_rgba(color: &RgbaColor) -> gdk::RGBA {
    gdk::RGBA::new(color.color.red, color.color.green, color.color.blue, color.alpha)
}
