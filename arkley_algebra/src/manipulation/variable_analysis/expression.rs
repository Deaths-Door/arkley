use std::collections::BTreeSet;

use crate::Expression;

use super::VariableAnalysis;

impl VariableAnalysis for Expression {
    fn get_unique_variables(&self) -> BTreeSet<char> {
        let mut unique_variables = BTreeSet::new();

        // Helper function to recursively traverse the Expression.
        fn extract_variables(expr: &Expression, unique_vars: &mut BTreeSet<char>) {
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