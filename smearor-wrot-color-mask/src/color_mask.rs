use crate::DEFAULT_COLOR_MASK_TOLERANCE;
use smearor_wrot_model_color::ParseHexError;
use smearor_wrot_model_color::RgbColor;
use smearor_wrot_model_color::ToHex;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub struct ColorMask {
    pub color: RgbColor,
    pub tolerance: f32,
}

impl ColorMask {
    pub fn new<R: Into<RgbColor>>(color: R, tolerance: f32) -> Self {
        Self {
            color: color.into(),
            tolerance,
        }
    }

    pub fn color(&self) -> RgbColor {
        self.color
    }

    pub fn tolerance(&self) -> f32 {
        self.tolerance
    }

    pub fn with_default_tolerance<R: Into<RgbColor>>(color: R) -> Self {
        Self {
            color: color.into(),
            tolerance: DEFAULT_COLOR_MASK_TOLERANCE,
        }
    }

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
