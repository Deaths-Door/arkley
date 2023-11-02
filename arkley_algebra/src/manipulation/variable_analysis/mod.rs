mod term;
mod expression;

pub use term::*;
pub use expression::*;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;

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
    fn get_unique_variables(&self) -> BTreeSet<&char>;
   
    /// Checks if any of the specified variables are present in the expression.
    ///
    /// # Returns
    ///
    /// `true` if at least one of the specified variables is present in the expression, `false` otherwise.
    fn contains_any_variable<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char>;

    /// Checks if a variable is present in the expression.
    ///
    /// # Returns
    ///
    /// `true` if the variable is present in the expression, `false` otherwise.
    fn contains_variable(&self, variable: &char) -> bool;

    /// Checks if all of the specified variables are present in the expression.
    ///
    /// # Returns
    ///
    /// `true` if all of the specified variables are present in the expression, `false` otherwise.
    fn contains_all<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char>;    

    /// Checks if all elements in `self` contain all entries in `entries`.
    ///
    /// This function iterates through `self` and checks if each element has all the entries
    /// provided by the iterator `iterator`.
    /// # Returns
    ///
    /// `true` if all elements in `self` contain all the entries, `false` otherwise.   
    #[deprecated(note = "Not used at all so maybe just remove it")]
    fn has_all<'a,I>(&self,iterator : &mut I) -> bool where I : Iterator<Item = &'a char>;
}