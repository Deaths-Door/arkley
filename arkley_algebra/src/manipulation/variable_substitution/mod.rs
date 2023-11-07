mod gen;
mod replace_variables;
pub use replace_variables::*;

use std::collections::HashMap;

/// A trait for types that support variable replacement.
///
/// Types implementing this trait can perform variable substitution in various ways (this is done for optimzation reasons).
pub trait VariableSubstitution<T> : Sized {
    /// Attempts to replace a single variable with a specified value.
    fn replace_single_variable(self, variable: &char, value: T) -> SingleVariableReplacements<Self,T> {
        SingleVariableReplacements::new(self, *variable, value)
    }
    
    /// Attempts to replace multiple variables with specified values.
    fn replace_variables<'a>(self, variable_values: &'a HashMap<char,T>) -> MultipleVariableReplacements<'a,Self,T> {
        MultipleVariableReplacements::new(self, variable_values)
    }
}