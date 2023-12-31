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
    Durch
}

impl std::fmt::Debug for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}