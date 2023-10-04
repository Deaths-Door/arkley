use std::collections::BTreeSet;

use crate::{Term, Expression};


/// A trait for operations related to variables within expressions.
pub trait VariableOperations {
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

impl VariableOperations for Term {
    fn get_unique_variables(&self) -> BTreeSet<char> {
        self.variables.keys().cloned().collect()   
    }

    fn contains_any_variable(&self,variables : &[&char]) -> bool {
        variables.iter().any(|&key| self.variables.contains_key(key))
    }
}

impl VariableOperations for Expression {
    fn get_unique_variables(&self) -> BTreeSet<char> {
        let mut unique_variables = BTreeSet::new();

        // Helper function to recursively traverse the Expression.
        fn extract_variables(expr: &Expression, unique_vars: &mut BTreeSet<char>) {
            match expr {
                Expression::Term(term) => {
                    unique_vars.extend(term.variables.keys().cloned());
                },
                Expression::Binary { left, right, .. } => {
                    extract_variables(left, unique_vars);
                    extract_variables(right, unique_vars);
                },
                Expression::Nested(inner) => {
                    extract_variables(inner, unique_vars);
                },
            }
        }

        extract_variables(self, &mut unique_variables);
        unique_variables
    }


    fn contains_any_variable(&self,variables : &[&char]) -> bool {
        match self {
            Expression::Term(term) => term.contains_any_variable(variables),
            Expression::Binary { left, right, .. } => left.contains_any_variable(variables) || right.contains_any_variable(variables)   ,
            Expression::Nested(inner) => inner.contains_any_variable(variables)
        }
    }
}