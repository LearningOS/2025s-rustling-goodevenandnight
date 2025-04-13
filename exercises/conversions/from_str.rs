//! This exercise focuses on implementing the FromStr trait for custom types.
//! Complete the implementation of the `from_str` method to parse a color string
//! in the format "r,g,b,a" where each component is an 8-bit unsigned integer.

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, PartialEq)]
pub enum ColorParseError {
    InvalidFormat,
    InvalidComponentValue,
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return Err(ColorParseError::InvalidFormat);
        }

        let parse_component = |s: &str| -> Result<u8, ColorParseError> {
            s.parse().map_err(|_| ColorParseError::InvalidComponentValue)
        };

        Ok(Color {
            r: parse_component(parts[0])?,
            g: parse_component(parts[1])?,
            b: parse_component(parts[2])?,
            a: parse_component(parts[3])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_color() {
        let c = Color::from_str("255,0,127,255").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 127);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_invalid_format() {
        assert!(matches!(
            Color::from_str("255,0,127"),
            Err(ColorParseError::InvalidFormat)
        ));
    }

    #[test]
    fn test_invalid_values() {
        assert!(matches!(
            Color::from_str("256,0,127,255"),
            Err(ColorParseError::InvalidComponentValue)
        ));
    }
}