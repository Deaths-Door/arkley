mod term;
mod expression;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;

#[cfg(feature="function")]
mod function;

#[cfg(feature="equation")]
pub use function::*;

use std::collections::HashMap;

use num_notation::Number;

/// A trait for types that support variable replacement.
///
/// Types implementing this trait can perform variable substitution in various ways (this is done for optimzation reasons).
pub trait VariableSubstitution<SV = Number,MV = HashMap<char,SV>> {
    /// Attempts to replace a single variable with a specified value.
    ///
    /// # Arguments
    ///
    /// - `variable`: A reference to the variable (char) to be replaced.
    /// - `value`: The value (Number) to replace the variable with.
    ///
    /// # Returns
    ///
    /// An `Option<()>` where:
    /// - `Some(())` indicates the variable was found and successfully replaced.
    /// - `None` indicates the variable did not exist, and no replacement occurred.
    fn replace_single_variable(&mut self, variable: &char, value: SV) -> Option<()>;

    /// Attempts to replace multiple variables with specified values.
    ///
    /// # Arguments
    ///
    /// - `variable_values`: A reference to a `Variables` map containing variables and their values.
    ///
    /// # Returns
    ///
    /// The updated term with the specified variables replaced. Variables that do not exist in the term
    /// are left unchanged in the `variable_values` map given.
    // TODO : THIS MAY NOT WORK LIKE EXCEpTED eg for 2x + 2x the first x is replaced but not the second
    fn replace_variables(&mut self, variable_values:&mut MV);
}
