use crate::color::RgbColor;
use crate::color::hex::ParseHexError;
use crate::color::hex::ToHex;
use std::fmt::Display;
use std::fmt::Formatter;

/// Default tolerance for color matching (10% of the color range)
pub const DEFAULT_COLOR_MASK_TOLERANCE: f32 = 0.1;

/// A color mask for chroma-keying / background replacement.
///
/// Consists of an RGB color and a tolerance value (0.0-1.0) that controls
/// how closely a pixel must match the mask color to be affected.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorMask {
    /// The target color to mask
    pub color: RgbColor,
    /// Tolerance for color matching (0.0 = exact match, 1.0 = match all)
    pub tolerance: f32,
}

impl ColorMask {
    /// Creates a new color mask from a color and tolerance
    pub fn new<R: Into<RgbColor>>(color: R, tolerance: f32) -> Self {
        Self {
            color: color.into(),
            tolerance,
        }
    }

    /// Returns the mask color
    pub fn color(&self) -> RgbColor {
        self.color
    }

    /// Returns the tolerance
    pub fn tolerance(&self) -> f32 {
        self.tolerance
    }

    /// Creates a color mask with the default tolerance
    pub fn with_default_tolerance<R: Into<RgbColor>>(color: R) -> Self {
        Self {
            color: color.into(),
            tolerance: DEFAULT_COLOR_MASK_TOLERANCE,
        }
    }

    /// Clamps color components and tolerance to valid ranges
    pub fn clamp(&self) -> Self {
        Self {
            color: self.color.clamp(),
            tolerance: self.tolerance.clamp(0.0, 1.0),
        }
    }
}

impl ToHex for ColorMask {
    fn to_hex(&self) -> String {
        self.color.to_hex()
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized,
    {
        RgbColor::parse_hex(hex).map(|color| Self {
            color,
            tolerance: DEFAULT_COLOR_MASK_TOLERANCE,
        })
    }
}

impl Display for ColorMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColorMask({}, tolerance={})", self.color, self.tolerance)
    }
}

impl From<ColorMask> for RgbColor {
    fn from(mask: ColorMask) -> Self {
        mask.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_mask_new() {
        let mask = ColorMask::new(RgbColor::new(1.0, 0.0, 0.0), 0.2);
        assert_eq!(mask.color.red, 1.0);
        assert_eq!(mask.color.green, 0.0);
        assert_eq!(mask.color.blue, 0.0);
        assert_eq!(mask.tolerance, 0.2);
    }

    #[test]
    fn test_color_mask_with_default_tolerance() {
        let mask = ColorMask::with_default_tolerance(RgbColor::new(0.5, 0.5, 0.5));
        assert_eq!(mask.tolerance, DEFAULT_COLOR_MASK_TOLERANCE);
    }

    #[test]
    fn test_color_mask_clamp() {
        let mask = ColorMask::new(RgbColor::new(2.0, -1.0, 0.5), 1.5);
        let clamped = mask.clamp();
        assert_eq!(clamped.color.red, 1.0);
        assert_eq!(clamped.color.green, 0.0);
        assert_eq!(clamped.color.blue, 0.5);
        assert_eq!(clamped.tolerance, 1.0);
    }

    #[test]
    fn test_color_mask_to_hex() {
        let mask = ColorMask::new(RgbColor::new_from_u8(255, 0, 0), 0.1);
        assert_eq!(mask.to_hex(), "#FF0000");
    }

    #[test]
    fn test_color_mask_parse_hex() {
        let mask = ColorMask::parse_hex("#FF0000").unwrap();
        assert_eq!(mask.color.red, 1.0);
        assert_eq!(mask.color.green, 0.0);
        assert_eq!(mask.color.blue, 0.0);
        assert_eq!(mask.tolerance, DEFAULT_COLOR_MASK_TOLERANCE);
    }

    #[test]
    fn test_color_mask_display() {
        let mask = ColorMask::new(RgbColor::new(1.0, 0.0, 0.0), 0.1);
        assert!(format!("{}", mask).contains("ColorMask"));
        assert!(format!("{}", mask).contains("tolerance=0.1"));
    }

    #[test]
    fn test_color_mask_to_rgb_color() {
        let mask = ColorMask::new(RgbColor::new(0.5, 0.3, 0.7), 0.2);
        let color: RgbColor = mask.into();
        assert_eq!(color.red, 0.5);
        assert_eq!(color.green, 0.3);
        assert_eq!(color.blue, 0.7);
    }
}
