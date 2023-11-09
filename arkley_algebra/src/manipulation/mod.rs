mod evaluation;

mod variable_analysis;
mod variable_substitution;

mod polynomial;

pub use evaluation::*;

pub use variable_analysis::*;
pub use variable_substitution::*;

pub use polynomial::*;


/// A trait for types that provide a `find` method to obtain a value of type `T`.
///
/// This trait is used as a helper method trait in various contexts, such as
/// calculating [Quadratic::discriminant], where one can get the value or describe it.
pub trait Find {
    /// Output Type
    type Output;

    /// Find and return a value of type `T`.
    ///
    /// # Returns
    ///
    /// A value of type `T` representing the result of the operation.
    #[must_use]
    fn find(self) -> Self::Output;
}
