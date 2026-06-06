use crate::SmearorCompositor;
use crate::color_mask::error::ColorMaskError;
use crate::color_mask::mask::ColorMask;
use std::sync::atomic::Ordering;
use tracing::debug;

pub trait SubSurfaceColorMask {
    /// Get subsurface color mask
    fn get_subsurface_color_mask(&self) -> Option<ColorMask>;

    /// Set subsurface color mask
    fn set_subsurface_color_mask(&self, subsurface_color_mask: ColorMask) -> Result<(), ColorMaskError>;

    /// Clear subsurface color mask (reset to None)
    fn clear_subsurface_color_mask(&self) -> Result<(), ColorMaskError>;

    /// Get auto subsurface color mask detection status
    fn get_auto_subsurface_color_mask(&self) -> bool;

    /// Set auto subsurface color mask detection status
    fn set_auto_subsurface_color_mask(&self, enabled: bool);

    /// Check if subsurface color mask has been detected
    fn is_subsurface_color_mask_detected(&self) -> bool;

    /// Set subsurface color mask detected flag
    fn set_subsurface_color_mask_detected(&self, detected: bool);
}

impl SubSurfaceColorMask for SmearorCompositor {
    fn get_subsurface_color_mask(&self) -> Option<ColorMask> {
        self.subsurface_color_mask.lock().ok().and_then(|subsurface_color_mask| *subsurface_color_mask)
    }

    fn set_subsurface_color_mask(&self, subsurface_color_mask: ColorMask) -> Result<(), ColorMaskError> {
        let clamped_subsurface_color_mask = subsurface_color_mask.clamp();
        let Ok(mut subsurface_color_mask) = self.subsurface_color_mask.lock() else {
            return Err(ColorMaskError::FailedToSetColorMask);
        };
        *subsurface_color_mask = Some(clamped_subsurface_color_mask);
        debug!("Set {clamped_subsurface_color_mask}");
        Ok(())
    }

    fn clear_subsurface_color_mask(&self) -> Result<(), ColorMaskError> {
        let Ok(mut subsurface_color_mask) = self.subsurface_color_mask.lock() else {
            return Err(ColorMaskError::FailedToClearColorMask);
        };
        *subsurface_color_mask = None;
        debug!("Subsurface color mask cleared");
        Ok(())
    }

    fn get_auto_subsurface_color_mask(&self) -> bool {
        self.auto_subsurface_color_mask.load(Ordering::Relaxed)
    }

    fn set_auto_subsurface_color_mask(&self, enabled: bool) {
        self.auto_subsurface_color_mask.store(enabled, Ordering::Relaxed);
        debug!("Auto subsurface color mask detection set to {}", enabled);
    }

    fn is_subsurface_color_mask_detected(&self) -> bool {
        self.subsurface_color_mask_detected.load(Ordering::Relaxed)
    }

    fn set_subsurface_color_mask_detected(&self, detected: bool) {
        self.subsurface_color_mask_detected.store(detected, Ordering::Relaxed);
        debug!("Subsurface color mask detected flag set to {}", detected);
    }
}
