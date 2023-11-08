mod inner;
mod gen;

pub use inner::*;
pub use gen::*;

#[cfg(feature="describe")]
mod describe;


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