mod gen;
mod inner;

pub use inner::*;

#[cfg(feature="describe")]
mod describe;

use std::collections::HashMap;

use super::Find;

/// A trait for types that support variable replacement.
///
/// Types implementing this trait can perform variable substitution in various ways (this is done for optimzation reasons).
pub trait VariableSubstitution<T> : Sized {
    /// Attempts to replace a single variable with a specified value.
    fn replace_single_variable(self, variable: &char, value: T) -> SingleVariableReplacements<Self,T> where SingleVariableReplacements<Self,T> : Find {
        SingleVariableReplacements::new(self, *variable, value)
    }
    
    /// Attempts to replace multiple variables with specified values.
    fn replace_variables<'a>(self, variable_values: &'a HashMap<char,T>) -> MultipleVariableReplacements<'a,Self,T> where MultipleVariableReplacements<'a,Self,T> : Find  {
        MultipleVariableReplacements::new(self, variable_values)
    }
}