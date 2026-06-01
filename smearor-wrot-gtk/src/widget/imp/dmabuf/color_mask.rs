use crate::extract_pixel_data_from_texture;
use gtk4::gdk::Texture;
use smearor_wrot_core::SmearorCompositor;
use smearor_wrot_core::color_mask::mask::ColorMask;
use smearor_wrot_core::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_model::color::hex::ToHex;
use tracing::debug;
use tracing::error;
use tracing::info;

pub trait DmaBufColorMask {
    fn detect_color_mask(&self, compositor: &SmearorCompositor, texture: &Texture);
}

impl DmaBufColorMask for crate::widget::imp::CompositorWidgetImpl {
    // color mask if enabled and not yet detected
    fn detect_color_mask(&self, compositor: &SmearorCompositor, texture: &Texture) {
        if !compositor.get_auto_color_mask() || compositor.is_color_mask_detected() {
            return;
        }
        debug!("DMA-BUF: Auto-detection enabled and not yet detected, extracting pixel data from DMA-BUF texture");
        let pixel_data = extract_pixel_data_from_texture(&texture);
        debug!("DMA-BUF: Successfully extracted {} bytes from texture", pixel_data.len());
        let Some(dominant_color) = pixel_data.get_dominant_color(8) else {
            debug!("DMA-BUF: Failed to detect dominant color from pixel data");
            return;
        };
        // Use default tolerance of 0.1 for auto-detection
        let Ok(_) = compositor.set_color_mask(ColorMask::with_default_tolerance(dominant_color)) else {
            error!("DMA-BUF: Failed to set color mask");
            return;
        };
        compositor.set_color_mask_detected(true);
        info!("DMA-BUF: Auto-detected dominant color from DMA buffer: {} with tolerance 0.1", dominant_color.to_hex());
    }
}
