mod term;

pub use term::*;

use std::collections::BTreeSet;

/// A trait for operations related to variables within expressions.
pub trait VariableAnalysis {
    /// Extracts unique variables from the expression.
    ///
    /// This function recursively traverses the expression and collects all unique variables
    /// found within it. The result is returned as a `BTreeSet<char>`, where each character
    /// represents a unique variable.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<char>` containing the unique variables present in the expression.
    fn get_unique_variables(&self) -> BTreeSet<char>;
   
    /// Checks if any of the specified variables are present in the expression.
    ///
    /// # Arguments
    ///
    /// * `variables`: A slice containing references to variables (char) to check for in the expression.
    ///
    /// # Returns
    ///
    /// `true` if at least one of the specified variables is present in the expression, `false` otherwise.
    fn contains_any_variable(&self,variables : &[&char]) -> bool;

    /// Checks if a variable is present in the expression.
    ///
    /// # Arguments
    ///
    /// * `variable`: The variable (char) to check for in the expression.
    ///
    /// # Returns
    ///
    /// `true` if the variable is present in the expression, `false` otherwise.
    fn contains_variable(&self, variable: &char) -> bool {
        self.contains_any_variable(&[variable])
    }
}
