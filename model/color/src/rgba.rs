use std::fmt::Display;
use std::fmt::Formatter;
use typed_builder::TypedBuilder;

use crate::ParseHexError;
use crate::RgbColor;
use crate::RgbColor24;
use crate::ToHex;
#[cfg(feature = "gtk4")]
use gtk4::gdk;

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct RgbaColor {
    pub color: RgbColor,
    pub alpha: f32,
}

impl RgbaColor {
    pub const fn new(color: RgbColor, alpha: f32) -> Self {
        Self { color, alpha }
    }

    pub const fn with_rgb(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            color: RgbColor::new(red, green, blue),
            alpha,
        }
    }

    pub fn transparent() -> Self {
        Self::new(RgbColor::default(), 0.0)
    }

    pub fn clamp(&self) -> Self {
        Self {
            color: self.color.clamp(),
            alpha: self.alpha.clamp(0.0, 1.0),
        }
    }

    pub fn parse_hex_with_optional_alpha(hex: &str) -> Result<Self, ParseHexError> {
        RgbaColor::parse_hex(hex).or_else(|_| RgbColor::parse_hex(hex).map(|rgb| RgbaColor::new(rgb, 1.0)))
    }
}

impl ToHex for RgbaColor {
    fn to_hex(&self) -> String {
        RgbaColor24::from(*self).to_hex()
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized,
    {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 8 {
            return Err(ParseHexError::InvalidSize);
        }

        let alpha = u8::from_str_radix(&hex[6..8], 16).map_err(|_| ParseHexError::InvalidAlpha)?;

        Ok(RgbaColor::new(
            RgbColor::new_from_u8(
                u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseHexError::InvalidRed)?,
                u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseHexError::InvalidGreen)?,
                u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseHexError::InvalidBlue)?,
            ),
            alpha as f32 / 255.0,
        ))
    }
}

impl Display for RgbaColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RgbaColor(r={}, g={}, b={} a={})", self.color.red, self.color.green, self.color.blue, self.alpha)
    }
}

#[cfg(feature = "gtk4")]
impl From<RgbaColor> for gdk::RGBA {
    fn from(color: RgbaColor) -> Self {
        gdk::RGBA::new(color.color.red, color.color.green, color.color.blue, color.alpha)
    }
}

#[cfg(feature = "gtk4")]
impl From<&RgbaColor> for gdk::RGBA {
    fn from(color: &RgbaColor) -> Self {
        gdk::RGBA::new(color.color.red, color.color.green, color.color.blue, color.alpha)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbaColor24 {
    pub color: RgbColor24,
    pub alpha: u8,
}
impl RgbaColor24 {
    pub fn new(color: RgbColor24, alpha: u8) -> Self {
        Self { color, alpha }
    }
}

impl ToHex for RgbaColor24 {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.color.red, self.color.green, self.color.blue, self.alpha)
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized,
    {
        RgbaColor::parse_hex(hex).map(Self::from)
    }
}

impl From<RgbaColor> for RgbaColor24 {
    fn from(value: RgbaColor) -> Self {
        let convert = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self {
            color: value.color.into(),
            alpha: convert(value.alpha),
        }
    }
}

impl Display for RgbaColor24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RgbaColor24(r={}, g={}, b={}, a={})", self.color.red, self.color.green, self.color.blue, self.alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_color_new() {
        let rgb = RgbColor::new(0.5, 0.25, 0.75);
        let color = RgbaColor::new(rgb, 0.5);
        assert_eq!(color.color.red, 0.5);
        assert_eq!(color.color.green, 0.25);
        assert_eq!(color.color.blue, 0.75);
        assert_eq!(color.alpha, 0.5);
    }

    #[test]
    fn test_rgba_color_with_rgb() {
        let color = RgbaColor::with_rgb(0.5, 0.25, 0.75, 0.5);
        assert_eq!(color.color.red, 0.5);
        assert_eq!(color.color.green, 0.25);
        assert_eq!(color.color.blue, 0.75);
        assert_eq!(color.alpha, 0.5);
    }

    #[test]
    fn test_rgba_color_transparent() {
        let color = RgbaColor::transparent();
        assert_eq!(color.color.red, 0.0);
        assert_eq!(color.color.green, 0.0);
        assert_eq!(color.color.blue, 0.0);
        assert_eq!(color.alpha, 0.0);
    }

    #[test]
    fn test_rgba_color_clamp() {
        let rgb = RgbColor::new(1.5, -0.5, 0.5);
        let color = RgbaColor::new(rgb, 1.5);
        let clamped = color.clamp();
        assert_eq!(clamped.color.red, 1.0);
        assert_eq!(clamped.color.green, 0.0);
        assert_eq!(clamped.color.blue, 0.5);
        assert_eq!(clamped.alpha, 1.0);
    }

    #[test]
    fn test_rgba_color_to_hex() {
        let rgb = RgbColor::new(1.0, 0.5, 0.25);
        let color = RgbaColor::new(rgb, 0.5);
        assert_eq!(color.to_hex(), "#FF804080");
    }

    #[test]
    fn test_rgba_color_parse_hex() {
        let color = RgbaColor::parse_hex("#FF804080").unwrap();
        assert!((color.color.red - 1.0).abs() < 0.01);
        assert!((color.color.green - 0.502).abs() < 0.01);
        assert!((color.color.blue - 0.251).abs() < 0.01);
        assert!((color.alpha - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_rgba_color_parse_hex_without_hash() {
        let color = RgbaColor::parse_hex("FF804080").unwrap();
        assert!((color.color.red - 1.0).abs() < 0.01);
        assert!((color.color.green - 0.502).abs() < 0.01);
        assert!((color.color.blue - 0.251).abs() < 0.01);
        assert!((color.alpha - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_rgba_color_parse_hex_invalid_size() {
        let result = RgbaColor::parse_hex("#FF8040");
        assert!(matches!(result, Err(ParseHexError::InvalidSize)));
    }

    #[test]
    fn test_rgba_color_parse_hex_with_optional_alpha_valid() {
        let color = RgbaColor::parse_hex_with_optional_alpha("#FF804080").unwrap();
        assert!((color.color.red - 1.0).abs() < 0.01);
        assert!((color.color.green - 0.502).abs() < 0.01);
        assert!((color.color.blue - 0.251).abs() < 0.01);
        assert!((color.alpha - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_rgba_color_parse_hex_with_optional_alpha_fallback() {
        let color = RgbaColor::parse_hex_with_optional_alpha("#FF8040").unwrap();
        assert!((color.color.red - 1.0).abs() < 0.01);
        assert!((color.color.green - 0.502).abs() < 0.01);
        assert!((color.color.blue - 0.251).abs() < 0.01);
        assert_eq!(color.alpha, 1.0);
    }

    #[test]
    fn test_rgba_color_display() {
        let rgb = RgbColor::new(0.5, 0.25, 0.75);
        let color = RgbaColor::new(rgb, 0.5);
        assert_eq!(format!("{}", color), "RgbaColor(r=0.5, g=0.25, b=0.75 a=0.5)");
    }

    #[test]
    fn test_rgba_color24_new() {
        let rgb = RgbColor24::new(128, 64, 192);
        let color = RgbaColor24::new(rgb, 128);
        assert_eq!(color.color.red, 128);
        assert_eq!(color.color.green, 64);
        assert_eq!(color.color.blue, 192);
        assert_eq!(color.alpha, 128);
    }

    #[test]
    fn test_rgba_color24_to_hex() {
        let rgb = RgbColor24::new(255, 128, 64);
        let color = RgbaColor24::new(rgb, 128);
        assert_eq!(color.to_hex(), "#FF804080");
    }

    #[test]
    fn test_rgba_color24_parse_hex() {
        let color = RgbaColor24::parse_hex("#FF804080").unwrap();
        assert_eq!(color.color.red, 255);
        assert_eq!(color.color.green, 128);
        assert_eq!(color.color.blue, 64);
        assert_eq!(color.alpha, 128);
    }

    #[test]
    fn test_rgba_color24_display() {
        let rgb = RgbColor24::new(128, 64, 192);
        let color = RgbaColor24::new(rgb, 128);
        assert_eq!(format!("{}", color), "RgbaColor24(r=128, g=64, b=192, a=128)");
    }

    #[test]
    fn test_rgba_color_to_rgba_color24() {
        let rgb = RgbColor::new(1.0, 0.5, 0.25);
        let color = RgbaColor::new(rgb, 0.5);
        let color24: RgbaColor24 = color.into();
        assert_eq!(color24.color.red, 255);
        assert_eq!(color24.color.green, 128);
        assert_eq!(color24.color.blue, 64);
        assert_eq!(color24.alpha, 128);
    }

    #[test]
    fn test_rgba_color_clamp_boundary() {
        let rgb = RgbColor::new(0.0, 1.0, 0.5);
        let color = RgbaColor::new(rgb, 0.0);
        let clamped = color.clamp();
        assert_eq!(clamped.color.red, 0.0);
        assert_eq!(clamped.color.green, 1.0);
        assert_eq!(clamped.color.blue, 0.5);
        assert_eq!(clamped.alpha, 0.0);
    }

    #[test]
    fn test_rgba_color_clamp_alpha() {
        let rgb = RgbColor::new(0.5, 0.5, 0.5);
        let color = RgbaColor::new(rgb, 1.5);
        let clamped = color.clamp();
        assert_eq!(clamped.alpha, 1.0);
    }
}
