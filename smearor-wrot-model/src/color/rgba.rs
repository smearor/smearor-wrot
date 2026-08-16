use crate::color::RgbColor;
use crate::color::hex::ParseHexError;
use crate::color::hex::ToHex;
use crate::color::rgb::RgbColor24;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use typed_builder::TypedBuilder;

#[cfg(feature = "gtk4")]
use gtk4::gdk;

#[derive(Debug, Clone, Copy, PartialEq, TypedBuilder)]
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

impl From<RgbColor> for RgbaColor {
    fn from(color: RgbColor) -> Self {
        RgbaColor::new(color, 1.0)
    }
}

impl From<&str> for RgbaColor {
    fn from(hex: &str) -> Self {
        RgbaColor::parse_hex_with_optional_alpha(hex).unwrap_or_else(|_| {
            panic!("Invalid hex color string: '{hex}'")
        })
    }
}

impl From<String> for RgbaColor {
    fn from(hex: String) -> Self {
        RgbaColor::from(hex.as_str())
    }
}

impl FromStr for RgbaColor {
    type Err = ParseHexError;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        RgbaColor::parse_hex_with_optional_alpha(hex)
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
    fn test_rgba_color_from_rgb_color() {
        let rgb = RgbColor::new(0.5, 0.3, 0.7);
        let rgba: RgbaColor = rgb.into();
        assert_eq!(rgba.color.red, 0.5);
        assert_eq!(rgba.color.green, 0.3);
        assert_eq!(rgba.color.blue, 0.7);
        assert_eq!(rgba.alpha, 1.0);
    }

    #[test]
    fn test_rgba_color_from_str_hex() {
        let rgba: RgbaColor = "#FF0000".into();
        assert_eq!(rgba.color.red, 1.0);
        assert_eq!(rgba.color.green, 0.0);
        assert_eq!(rgba.color.blue, 0.0);
        assert_eq!(rgba.alpha, 1.0);
    }

    #[test]
    fn test_rgba_color_from_str_hex_with_alpha() {
        let rgba: RgbaColor = "#FF000077".into();
        assert_eq!(rgba.color.red, 1.0);
        assert_eq!(rgba.color.green, 0.0);
        assert_eq!(rgba.color.blue, 0.0);
        assert!((rgba.alpha - 0.4667).abs() < 0.01);
    }

    #[test]
    fn test_rgba_color_from_string() {
        let rgba: RgbaColor = String::from("#00FF00").into();
        assert_eq!(rgba.color.red, 0.0);
        assert_eq!(rgba.color.green, 1.0);
        assert_eq!(rgba.color.blue, 0.0);
    }

    #[test]
    fn test_rgba_color_from_str_trait() {
        let rgba = RgbaColor::from_str("#0000FF").unwrap();
        assert_eq!(rgba.color.red, 0.0);
        assert_eq!(rgba.color.green, 0.0);
        assert_eq!(rgba.color.blue, 1.0);
    }

    #[test]
    fn test_rgba_color_from_str_invalid() {
        let result = RgbaColor::from_str("invalid");
        assert!(result.is_err());
    }

    #[cfg(feature = "gtk4")]
    #[test]
    fn test_rgba_color_to_gdk_rgba() {
        let rgba = RgbaColor::new(RgbColor::new(1.0, 0.0, 0.0), 0.5);
        let gdk_rgba: gdk::RGBA = rgba.into();
        assert_eq!(gdk_rgba.red(), 1.0);
        assert_eq!(gdk_rgba.green(), 0.0);
        assert_eq!(gdk_rgba.blue(), 0.0);
        assert_eq!(gdk_rgba.alpha(), 0.5);
    }
}
