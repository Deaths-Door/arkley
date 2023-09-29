use std::collections::BTreeSet;

use crate::{Term, Expression};

impl Term {
    /// Checks if a variable is present in the term.
    ///
    /// # Arguments
    ///
    /// * `variable`: The variable (char) to check for in the term.
    ///
    /// # Returns
    ///
    /// `true` if the variable is present in the term, `false` otherwise.
    pub fn contains_variable(&self, variable: &char) -> bool {
        self.variables.contains_key(variable)
    }
}

impl Expression {
    /// Extracts unique variables from the expression.
    ///
    /// This function recursively traverses the expression and collects all unique variables
    /// found within it. The result is returned as a `BTreeSet<char>`, where each character
    /// represents a unique variable.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<char>` containing the unique variables present in the expression.
    pub fn get_unique_variables(&self) -> BTreeSet<char> {
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

    /// Checks if a variable is present in the expression.
    ///
    /// # Arguments
    ///
    /// * `variable`: The variable (char) to check for in the expression.
    ///
    /// # Returns
    ///
    /// `true` if the variable is present in the expression, `false` otherwise.
    pub fn contains_variable(&self, variable: &char) -> bool {
        // Helper function to recursively check for the variable.
        fn check_variable(expr: &Expression, target: &char) -> bool {
            match expr {
                Expression::Term(term) => term.variables.contains_key(target),
                Expression::Binary { left, right, .. } => check_variable(left, target) || check_variable(right, target),
                Expression::Nested(inner) => check_variable(inner, target),
            }
        }

        check_variable(self, variable)
    }

}