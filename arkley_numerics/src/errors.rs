use std::num::{ParseFloatError,ParseIntError};

/// Represents the possible errors that can occur during parsing of a `StandardForm` number.
#[derive(Debug)]
pub enum ParsingStandardFormError {
    /// Error that occurs while parsing the mantissa as a `ParseFloatError`.
    Mantissa(ParseFloatError),
    /// Error that occurs while parsing the exponent as a `ParseIntError`.
    Exponent(ParseIntError),
    /// Indicates an invalid format that doesn't match any valid `StandardForm` notation.
    InvalidFormat,
}

impl std::fmt::Display for ParsingStandardFormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingStandardFormError::Mantissa(err) => write!(f, "Error parsing mantissa: {}", err),
            ParsingStandardFormError::Exponent(err) => write!(f, "Error parsing exponent: {}", err),
            ParsingStandardFormError::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}