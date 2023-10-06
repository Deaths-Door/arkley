mod expression;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;


use num_notation::Number;

use crate::{Variables, Expression};

use super::{VariableExpressionAssociation, VariableSubstitution};

/// A trait for evaluating expressions and values.
///
/// This trait provides methods for evaluating expressions and values in different contexts.
/// Implementations of this trait should define how an expression or value is evaluated
/// and return the result accordingly.
pub trait Evaluate<Output = Self> : VariableSubstitution {
    /// Evaluate the expression and return the result.
    fn evaluate(self) -> Output;

    /// Evaluate the expression with provided variable values and return the result.
    ///
    /// # Arguments
    ///
    /// * `variable_values`: A mutable reference to a collection of variable-value associations.
    ///
    /// # Returns
    ///
    /// The result of evaluating the expression with the provided variable values.
    fn evaluate_single_variable_with_value(self,variable : &char,value : Number) -> Output;

    /// Evaluate the expression with a single variable and its assigned value.
    ///
    /// This method evaluates the expression with a specific variable and its associated value.
    ///
    /// # Arguments
    ///
    /// * `variable_values`: A mutable reference to a collection of variable-value associations.
    ///
    /// # Returns
    ///
    /// The result of evaluating the expression with the single variable's value.
    fn evaluate_with_values(self, variable_values: &mut Variables) -> Output;

    /// Evaluate the expression with a single variable and an assigned expression as its value.
    ///
    /// This method evaluates the expression with a specific variable and its associated expression value.
    ///
    /// # Arguments
    ///
    /// * `variable`: A reference to the character representing the variable.
    /// * `value`: An expression that defines the value of the variable.
    ///
    /// # Returns
    ///
    /// The result of evaluating the expression with the variable's expression value.
    fn evaluate_single_variable_with_expr(self,_variable : &char,_value : Expression) -> Output;

    /// Evaluate the expression with provided variable expressions and return the result.
    ///
    /// # Arguments
    ///
    /// * `variable_values`: A mutable reference to a collection of variable-expression associations.
    ///
    /// # Returns
    ///
    /// The result of evaluating the expression with the provided variable expressions.
    fn evaluate_with_expr(self, variable_values: &mut VariableExpressionAssociation) -> Output;
}