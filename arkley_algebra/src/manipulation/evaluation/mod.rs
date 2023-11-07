mod inner;
mod gen;

pub use inner::*;
pub use gen::*;

use std::collections::HashMap;
use super::{VariableSubstitution, Find, SingleVariableReplacements, MultipleVariableReplacements};

/// This trait provides method for evaluating expressions
pub trait Evaluate : Sized {
    /// Evaluates the expression 
    fn evaluate(self) -> EvaluateNoValues<Self> where EvaluateNoValues<Self> : Find {
        EvaluateNoValues(self)
    }
}

/// This trait provides method for evaluating expressions and values in different contexts.
pub trait EvaluteWithValues<T> : Evaluate + VariableSubstitution<T> + Sized {   
    /// Evaluates the expression with a single variable replacement
    fn evaluate_with_single_value(self, variable: &char, value: T) -> EvaluateWithSingleValue<Self,T> 
        where EvaluateWithSingleValue<Self,T> : Find ,
        SingleVariableReplacements<Self, T>: Find
    {
        EvaluateWithSingleValue(self.replace_single_variable(variable, value))
    }

    /// Evaluates the expression with multiple variable replacements
    fn evaluate_with_multiple_values<'a>(self, variable_values: &'a HashMap<char, T>) -> EvaluateWithMultipleValues<'a,Self,T> 
        where EvaluateWithSingleValue<Self,T> : Find ,
        MultipleVariableReplacements<'a, Self, T>: Find
    {
        EvaluateWithMultipleValues(self.replace_variables(variable_values))
    }
}
/*
mod expression;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;

#[cfg(feature="function")]
mod function;

#[cfg(feature="function")]
pub use function::*;
*/
/*
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
    fn evaluate_with_single_variable<SV,MV>(self, variable: &char, value: SV) -> Output where Self: VariableSubstitution<SV,MV>, SV: Clone ;

    /// Evaluate the expression with provided variable values and return the result.
    fn evaluate_with_variables<SV,MV>(self, variable_values:&mut MV) -> Output where Self: VariableSubstitution<SV,MV>, SV: Clone ;
}*/