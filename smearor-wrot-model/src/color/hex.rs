use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ParseHexError {
    #[error("Invalid size")]
    InvalidSize,
    #[error("Invalid character")]
    InvalidCharacter,
    #[error("Invalid red component")]
    InvalidRed,
    #[error("Invalid green component")]
    InvalidGreen,
    #[error("Invalid blue component")]
    InvalidBlue,
    #[error("Invalid alpha component")]
    InvalidAlpha,
}

pub trait ToHex {
    fn to_hex(&self) -> String;

    fn parse_hex(hex: &str) -> Result<Self, ParseHexError>
    where
        Self: Sized;
}
