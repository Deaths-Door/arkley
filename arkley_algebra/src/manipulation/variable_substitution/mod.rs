mod gen;
mod replace_variables;
pub use replace_variables::*;

use std::collections::HashMap;

/// A trait for types that support variable replacement.
///
/// Types implementing this trait can perform variable substitution in various ways (this is done for optimzation reasons).
pub trait VariableSubstitution {
    // Type of value
    type Input;

    /// Attempts to replace a single variable with a specified value.
    fn replace_single_variable(self, variable: &char, value: Self::Input) -> SingleVariableReplacements<Self,Self::Input> {
        SingleVariableReplacements::new(self, variable, value);
    }
    
    /// Attempts to replace multiple variables with specified values.
    fn replace_variables<'a>(self, variable_values: &'a HashMap<char,Self::Input>) -> MultipleVariableReplacements<'a,Self,Self::Input> {
        MultipleVariableReplacements::new(self, variable_values);
    }
}