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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_error_display() {
        assert_eq!(format!("{}", ParseHexError::InvalidSize), "Invalid size");
        assert_eq!(format!("{}", ParseHexError::InvalidCharacter), "Invalid character");
        assert_eq!(format!("{}", ParseHexError::InvalidRed), "Invalid red component");
        assert_eq!(format!("{}", ParseHexError::InvalidGreen), "Invalid green component");
        assert_eq!(format!("{}", ParseHexError::InvalidBlue), "Invalid blue component");
        assert_eq!(format!("{}", ParseHexError::InvalidAlpha), "Invalid alpha component");
    }
}
