mod expression;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;

use super::VariableSubstitution;


/// A trait for evaluating expressions and values.
///
/// This trait provides methods for evaluating expressions and values in different contexts.
/// Implementations of this trait should define how an expression or value is evaluated
/// and return the result accordingly.
pub trait Evaluate<Output = Self> {
    /// Evaluate the expression and return the result.
    fn evaluate(self) -> Output;

    /// Evaluate the expression with provided variable values and return the result.
    fn evaluate_with_single_variable<SV,MV>(self, variable: &char, value: SV) -> Self where Self: VariableSubstitution<SV,MV>, SV: Clone ;

    /// Evaluate the expression with provided variable values and return the result.
    fn evaluate_with_variables<SV,MV>(self, variable_values:&mut MV) -> Self where Self: VariableSubstitution<SV,MV>, SV: Clone ;
}