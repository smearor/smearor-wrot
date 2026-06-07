use crate::ParseHexError;
use crate::ToHex;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct RgbColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl RgbColor {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn new_from_u8(red: u8, green: u8, blue: u8) -> Self {
        RgbColor24::new(red, green, blue).into()
    }

    pub fn clamp(&self) -> Self {
        Self {
            red: self.red.clamp(0.0, 1.0),
            green: self.green.clamp(0.0, 1.0),
            blue: self.blue.clamp(0.0, 1.0),
        }
    }

    pub fn from_rgb(value: (u8, u8, u8)) -> Self {
        RgbColor24::new(value.0, value.1, value.2).into()
    }

    pub fn from_bgr(value: (u8, u8, u8)) -> Self {
        RgbColor24::new(value.2, value.1, value.0).into()
    }
}

impl ToHex for RgbColor {
    fn to_hex(&self) -> String {
        RgbColor24::from(*self).to_hex()
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized,
    {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(ParseHexError::InvalidSize);
        }

        Ok(RgbColor::new_from_u8(
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseHexError::InvalidRed)?,
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseHexError::InvalidGreen)?,
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseHexError::InvalidBlue)?,
        ))
    }
}

impl Default for RgbColor {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Display for RgbColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RgbColor(r={}, g={}, b={})", self.red, self.green, self.blue)
    }
}

impl From<RgbColor24> for RgbColor {
    fn from(value: RgbColor24) -> Self {
        let convert = |v: u8| (v as f32 / 255.0).clamp(0.0, 1.0);
        Self {
            red: convert(value.red),
            green: convert(value.green),
            blue: convert(value.blue),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor24 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor24 {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn new_from_u8(red: f32, green: f32, blue: f32) -> Self {
        RgbColor::new(red, green, blue).into()
    }

    pub fn from_rgb(value: (u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2)
    }

    pub fn from_bgr(value: (u8, u8, u8)) -> Self {
        Self::new(value.2, value.1, value.0)
    }
}

impl ToHex for RgbColor24 {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    fn parse_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized,
    {
        RgbColor::parse_hex(hex).map(Self::from)
    }
}

impl From<RgbColor> for RgbColor24 {
    fn from(value: RgbColor) -> Self {
        let convert = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self {
            red: convert(value.red),
            green: convert(value.green),
            blue: convert(value.blue),
        }
    }
}

impl Display for RgbColor24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RgbColor24(r={}, g={}, b={})", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color_new() {
        let color = RgbColor::new(0.5, 0.25, 0.75);
        assert_eq!(color.red, 0.5);
        assert_eq!(color.green, 0.25);
        assert_eq!(color.blue, 0.75);
    }

    #[test]
    fn test_rgb_color_new_from_u8() {
        let color = RgbColor::new_from_u8(128, 64, 192);
        assert!((color.red - 0.502).abs() < 0.01);
        assert!((color.green - 0.251).abs() < 0.01);
        assert!((color.blue - 0.753).abs() < 0.01);
    }

    #[test]
    fn test_rgb_color_clamp() {
        let color = RgbColor::new(1.5, -0.5, 0.5);
        let clamped = color.clamp();
        assert_eq!(clamped.red, 1.0);
        assert_eq!(clamped.green, 0.0);
        assert_eq!(clamped.blue, 0.5);
    }

    #[test]
    fn test_rgb_color_from_rgb() {
        let color = RgbColor::from_rgb((255, 128, 64));
        assert!((color.red - 1.0).abs() < 0.01);
        assert!((color.green - 0.502).abs() < 0.01);
        assert!((color.blue - 0.251).abs() < 0.01);
    }

    #[test]
    fn test_rgb_color_from_bgr() {
        let color = RgbColor::from_bgr((64, 128, 255));
        assert!((color.red - 1.0).abs() < 0.01);
        assert!((color.green - 0.502).abs() < 0.01);
        assert!((color.blue - 0.251).abs() < 0.01);
    }

    #[test]
    fn test_rgb_color_to_hex() {
        let color = RgbColor::new(1.0, 0.5, 0.25);
        assert_eq!(color.to_hex(), "#FF8040");
    }

    #[test]
    fn test_rgb_color_parse_hex() {
        let color = RgbColor::parse_hex("#FF8040").unwrap();
        assert!((color.red - 1.0).abs() < 0.01);
        assert!((color.green - 0.502).abs() < 0.01);
        assert!((color.blue - 0.251).abs() < 0.01);
    }

    #[test]
    fn test_rgb_color_parse_hex_without_hash() {
        let color = RgbColor::parse_hex("FF8040").unwrap();
        assert!((color.red - 1.0).abs() < 0.01);
        assert!((color.green - 0.502).abs() < 0.01);
        assert!((color.blue - 0.251).abs() < 0.01);
    }

    #[test]
    fn test_rgb_color_parse_hex_invalid_size() {
        let result = RgbColor::parse_hex("#FF80");
        assert!(matches!(result, Err(ParseHexError::InvalidSize)));
    }

    #[test]
    fn test_rgb_color_parse_hex_invalid_characters() {
        let result = RgbColor::parse_hex("#GGGGGG");
        assert!(matches!(result, Err(ParseHexError::InvalidRed)));
    }

    #[test]
    fn test_rgb_color_default() {
        let color = RgbColor::default();
        assert_eq!(color.red, 0.0);
        assert_eq!(color.green, 0.0);
        assert_eq!(color.blue, 0.0);
    }

    #[test]
    fn test_rgb_color_display() {
        let color = RgbColor::new(0.5, 0.25, 0.75);
        assert_eq!(format!("{}", color), "RgbColor(r=0.5, g=0.25, b=0.75)");
    }

    #[test]
    fn test_rgb_color24_new() {
        let color = RgbColor24::new(128, 64, 192);
        assert_eq!(color.red, 128);
        assert_eq!(color.green, 64);
        assert_eq!(color.blue, 192);
    }

    #[test]
    fn test_rgb_color24_from_rgb() {
        let color = RgbColor24::from_rgb((255, 128, 64));
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_rgb_color24_from_bgr() {
        let color = RgbColor24::from_bgr((64, 128, 255));
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_rgb_color24_to_hex() {
        let color = RgbColor24::new(255, 128, 64);
        assert_eq!(color.to_hex(), "#FF8040");
    }

    #[test]
    fn test_rgb_color24_parse_hex() {
        let color = RgbColor24::parse_hex("#FF8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_rgb_color24_display() {
        let color = RgbColor24::new(128, 64, 192);
        assert_eq!(format!("{}", color), "RgbColor24(r=128, g=64, b=192)");
    }

    #[test]
    fn test_rgb_color_to_rgb_color24() {
        let color = RgbColor::new(1.0, 0.5, 0.25);
        let color24: RgbColor24 = color.into();
        assert_eq!(color24.red, 255);
        assert_eq!(color24.green, 128);
        assert_eq!(color24.blue, 64);
    }

    #[test]
    fn test_rgb_color24_to_rgb_color() {
        let color24 = RgbColor24::new(255, 128, 64);
        let color: RgbColor = color24.into();
        assert!((color.red - 1.0).abs() < 0.01);
        assert!((color.green - 0.502).abs() < 0.01);
        assert!((color.blue - 0.251).abs() < 0.01);
    }

    #[test]
    fn test_rgb_color_clamp_boundary() {
        let color = RgbColor::new(0.0, 1.0, 0.5);
        let clamped = color.clamp();
        assert_eq!(clamped.red, 0.0);
        assert_eq!(clamped.green, 1.0);
        assert_eq!(clamped.blue, 0.5);
    }
}
