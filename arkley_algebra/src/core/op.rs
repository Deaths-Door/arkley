use strum::Display;
/// An enum representing basic arithmetic operations.
///
/// The `ArithmeticOperation` enum includes variants for common arithmetic operations
/// such as addition, subtraction, multiplication, and division.
#[derive(PartialEq,Clone,Display,Hash)]
pub enum ArithmeticOperation {
    /// Represents the addition operation (+).
    #[strum(serialize = "+")]
    Plus, 

    /// Represents the subtraction operation (-).
    #[strum(serialize = "-",serialize = "-+",serialize = "+-")]
    Minus,

    /// Represents the multiplication operation (*).
    #[strum(serialize = "*")]
    Mal,

    /// Represents the division operation (/).
    #[strum(serialize = "/")]
    Durch,

    /// Represents the exponentatal operation (^).
    #[strum(serialize = "^")]
    Pow,

    /// Represents the nth root operation
    #[strum(serialize = "√")]
    Root,
}

impl std::fmt::Debug for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl TryFrom<char> for ArithmeticOperation {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use ArithmeticOperation::*;
        match value {
            '+' => Ok(Plus),
            '-' => Ok(Minus),
            '*' => Ok(Mal),
            '/' => Ok(Durch),
            _ => Err(value)
        }
    }
}

impl ArithmeticOperation {
    pub(crate) const SQRT_SIGN : char = '√';
    pub(crate) const CBRT_SIGN : char = '∛';

    pub(crate) const SQRT_TEXT : &'static str = "sqrt";
    pub(crate) const CBRT_TEXT : &'static str = "cbrt";
}