
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

impl ArithmeticOperation {
    pub(crate) const fn negate_if_plus_or_minus(self) -> Self {
        use ArithmeticOperation::*;
        match self {
            Plus => Minus,
            Minus => Plus,
            _ => self
        }
    }
}