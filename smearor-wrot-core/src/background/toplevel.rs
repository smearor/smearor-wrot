use crate::SmearorCompositor;
use crate::background::error::BackgroundColorError;
use smearor_wrot_model::color::rgba::RgbaColor;
use tracing::debug;

pub trait ToplevelBackground {
    /// Get toplevel background color (RGBA, 0.0-1.0 range)
    fn get_background_color(&self) -> Option<RgbaColor>;

    /// Set toplevel background color (RGBA, 0.0-1.0 range)
    fn set_background_color(&self, toplevel_background_color: RgbaColor) -> Result<(), BackgroundColorError>;
}

impl ToplevelBackground for SmearorCompositor {
    fn get_background_color(&self) -> Option<RgbaColor> {
        self.background_color.lock().ok().and_then(|background_color| *background_color)
    }

    fn set_background_color(&self, toplevel_background_color: RgbaColor) -> Result<(), BackgroundColorError> {
        let clamped_toplevel_background_color = toplevel_background_color.clamp();
        let Ok(mut toplevel_background_color) = self.background_color.lock() else {
            return Err(BackgroundColorError::FailedToSetBackgroundColor);
        };
        *toplevel_background_color = Some(clamped_toplevel_background_color);
        debug!("Toplevel background color set to {clamped_toplevel_background_color}");
        Ok(())
    }
}
