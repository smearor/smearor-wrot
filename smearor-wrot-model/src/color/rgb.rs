use crate::color::hex::ParseHexError;
use crate::color::hex::ToHex;
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
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
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
