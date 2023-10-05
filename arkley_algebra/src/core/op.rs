
/// An enum representing basic arithmetic operations.
///
/// The `ArithmeticOperation` enum includes variants for common arithmetic operations
/// such as addition, subtraction, multiplication, and division.
#[derive(PartialEq,Clone)]
pub enum ArithmeticOperation {
    /// Represents the addition operation (+).
    Plus, 

    /// Represents the subtraction operation (-).
    Minus,

    /// Represents the multiplication operation (*).
    Mal,

    /// Represents the division operation (/).
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

impl std::fmt::Debug for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ArithmeticOperation::*;
        write!(f,"{}",match self {
            Plus => "+",
            Minus => "-",
            Mal => "*",
            Durch => "/",
        })
    }
}