
// TODO : Use `thiserror` to make this an error type
/// An error type for `TryFrom` for [super::Quadratic]
#[derive(Debug,Clone)]
pub enum QuadraticError {
    /// An error indicating that the coefficient `a` is zero, making concavity undefined.
    UndefinedConcavity,

    /// An error indicating that there are multiple variables with exponents of 2 in the input , hence unable to 'decide' the coeffiecient
    MultipleVariablesToThePowerOf2,

    /// An error indicating that the input does not represent a valid quadratic equation.
    InvalidQuadratic
}
