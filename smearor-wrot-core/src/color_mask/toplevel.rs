use crate::SmearorCompositor;
use crate::color_mask::error::ColorMaskError;
use crate::color_mask::mask::ColorMask;
use std::sync::atomic::Ordering;
use tracing::debug;

pub const DEFAULT_COLOR_MASK_TOLERANCE: f32 = 0.1;

pub trait TopLevelColorMask {
    /// Get color mask
    fn get_color_mask(&self) -> Option<ColorMask>;

    /// Set color mask for chroma-keying (RGB + tolerance, 0.0-1.0 range)
    fn set_color_mask(&self, color_mask: ColorMask) -> Result<(), ColorMaskError>;

    /// Clear color mask (reset to None)
    fn clear_color_mask(&self) -> Result<(), ColorMaskError>;

    /// Get auto color mask detection status
    fn get_auto_color_mask(&self) -> bool;

    /// Set auto color mask detection enabled/disabled
    fn set_auto_color_mask(&self, enabled: bool);

    /// Check if color mask has been detected
    fn is_color_mask_detected(&self) -> bool;

    /// Set color mask detected flag
    fn set_color_mask_detected(&self, detected: bool);

    /// Get the color mask tolerance
    fn get_color_mask_tolerance(&self) -> f32;

    /// Set the color mask tolerance
    fn set_color_mask_tolerance(&self, tolerance: f32);
    /// Set the color mask shader flag
    fn set_color_mask_shader(&self, enabled: bool);
}

impl TopLevelColorMask for SmearorCompositor {
    fn get_color_mask(&self) -> Option<ColorMask> {
        self.color_mask.lock().ok().and_then(|color_mask| *color_mask)
    }

    fn set_color_mask(&self, color_mask: ColorMask) -> Result<(), ColorMaskError> {
        let clamped_color_mask = color_mask.clamp();
        let Ok(mut color_mask) = self.color_mask.lock() else {
            return Err(ColorMaskError::FailedToSetColorMask);
        };
        *color_mask = Some(clamped_color_mask);
        debug!("Set {clamped_color_mask}");
        Ok(())
    }

    fn clear_color_mask(&self) -> Result<(), ColorMaskError> {
        let Ok(mut color_mask) = self.color_mask.lock() else {
            return Err(ColorMaskError::FailedToClearColorMask);
        };
        *color_mask = None;
        debug!("Color mask cleared");
        Ok(())
    }

    fn get_auto_color_mask(&self) -> bool {
        self.auto_color_mask.load(Ordering::Relaxed)
    }

    fn set_auto_color_mask(&self, enabled: bool) {
        self.auto_color_mask.store(enabled, Ordering::Relaxed);
        debug!("Auto color mask detection set to {}", enabled);
    }

    fn is_color_mask_detected(&self) -> bool {
        self.color_mask_detected.load(Ordering::Relaxed)
    }

    fn set_color_mask_detected(&self, detected: bool) {
        self.color_mask_detected.store(detected, Ordering::Relaxed);
        debug!("Color mask detected flag set to {}", detected);
    }

    fn get_color_mask_tolerance(&self) -> f32 {
        if let Ok(tolerance) = self.color_mask_tolerance.lock() {
            *tolerance
        } else {
            DEFAULT_COLOR_MASK_TOLERANCE
        }
    }

    fn set_color_mask_tolerance(&self, tolerance: f32) {
        if let Ok(mut tolerance_guard) = self.color_mask_tolerance.lock() {
            *tolerance_guard = tolerance.clamp(0.0, 1.0);
        }
    }

    fn set_color_mask_shader(&self, enabled: bool) {
        self.color_mask_shader.store(enabled, Ordering::Relaxed);
    }
}
