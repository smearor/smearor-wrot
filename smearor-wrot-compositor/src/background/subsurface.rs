use crate::SmearorCompositor;
use crate::background::error::BackgroundColorError;
use smearor_wrot_color::RgbaColor;
use tracing::debug;

pub trait SubsurfaceBackground {
    /// Get subsurface background color (RGBA)
    fn get_subsurface_background_color(&self) -> Option<RgbaColor>;

    /// Set subsurface background color (RGBA, 0.0-1.0 range)
    fn set_subsurface_background_color(&self, subsurface_background_color: RgbaColor) -> Result<(), BackgroundColorError>;
}

impl SubsurfaceBackground for SmearorCompositor {
    fn get_subsurface_background_color(&self) -> Option<RgbaColor> {
        self.subsurface_background_color
            .lock()
            .ok()
            .and_then(|subsurface_background_color| *subsurface_background_color)
    }

    fn set_subsurface_background_color(&self, subsurface_background_color: RgbaColor) -> Result<(), BackgroundColorError> {
        let clamped_subsurface_background_color = subsurface_background_color.clamp();
        let Ok(mut subsurface_background_color) = self.subsurface_background_color.lock() else {
            return Err(BackgroundColorError::FailedToSetBackgroundColor);
        };
        *subsurface_background_color = Some(clamped_subsurface_background_color);
        debug!("Subsurface background color set to {clamped_subsurface_background_color}");
        Ok(())
    }
}
