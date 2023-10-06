use strum::Display;
/// An enum representing basic arithmetic operations.
///
/// The `ArithmeticOperation` enum includes variants for common arithmetic operations
/// such as addition, subtraction, multiplication, and division.
#[derive(PartialEq,Clone,Display)]
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
    Durch
}

impl TryFrom<char> for ArithmeticOperation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use ArithmeticOperation::*;
        match value {
            '+' => Ok(Plus),
            '-' => Ok(Minus),
            '*' => Ok(Mal),
            '/' => Ok(Durch),
            _ => panic!()
        }
    }
}


impl ArithmeticOperation {
    pub(crate) const fn precedence(&self) -> i32 {
        match self {
            ArithmeticOperation::Plus | ArithmeticOperation::Minus => 1,
            ArithmeticOperation::Mal | ArithmeticOperation::Durch => 2,
        }
    }
}

impl std::fmt::Debug for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}