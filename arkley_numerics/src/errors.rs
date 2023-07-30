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

/// Represents the possible parsing errors that can occur when converting a string to a `Number`.
/// 
/// The input string is in an invalid format for both `f64` and `StandardForm` parsing.
///
/// Contains two specific errors:
///
/// - `ParseFloatError`: Represents an error that occurred while parsing the string as `f64`.
/// - `ParsingStandardFormError`: Represents an error that occurred while parsing the string as `StandardForm`.
    
#[derive(Debug)]
pub struct ParsingNumberError(pub(super) ParseFloatError,pub(super) ParsingStandardFormError);

impl std::fmt::Display for ParsingNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let float_err = &self.0;
        let sf_err = &self.1;

        write!(
            f,
            "Invalid input format: Couldn't parse as f64 or StandardForm due to:\n\
             ------ Float Parsing Error ------\n\
             {float_err}\n\
             ------ StandardForm Parsing Error ------\n\
             {sf_err}",
        )
    }
}