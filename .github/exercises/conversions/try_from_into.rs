//! Implement the TryFrom trait for a custom type to handle conversions with potential errors.
//! Complete the implementation of `try_from` to convert a tuple of three integers into a Color struct
//! while validating that each component is within the 0-255 range.

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq)]
pub enum ColorError {
    InvalidRed,
    InvalidGreen,
    InvalidBlue,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = ColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;

        if red < 0 || red > 255 {
            return Err(ColorError::InvalidRed);
        }

        if green < 0 || green > 255 {
            return Err(ColorError::InvalidGreen);
        }

        if blue < 0 || blue > 255 {
            return Err(ColorError::InvalidBlue);
        }

        Ok(Color {
            r: red as u8,
            g: green as u8,
            b: blue as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_color() {
        let c = Color::try_from((0, 128, 255)).unwrap();
        assert_eq!(c.r, 0);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 255);
    }

    #[test]
    fn test_invalid_red() {
        assert!(matches!(
            Color::try_from((-1, 128, 255)),
            Err(ColorError::InvalidRed)
        ));
    }

    #[test]
    fn test_invalid_green() {
        assert!(matches!(
            Color::try_from((0, 256, 255)),
            Err(ColorError::InvalidGreen)
        ));
    }

    #[test]
    fn test_invalid_blue() {
        assert!(matches!(
            Color::try_from((0, 128, 512)),
            Err(ColorError::InvalidBlue)
        ));
    }
}