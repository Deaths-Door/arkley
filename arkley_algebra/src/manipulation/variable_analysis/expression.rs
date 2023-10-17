use std::collections::BTreeSet;

use crate::Expression;

use super::VariableAnalysis;

impl VariableAnalysis for Expression {
    fn get_unique_variables(&self) -> BTreeSet<&char> {
        let mut unique_variables = BTreeSet::new();

        // Helper function to recursively traverse the Expression.
        fn extract_variables<'a>(expr: &'a Expression, unique_vars: &mut BTreeSet<&'a char>) {
            match expr {
                Expression::Term(term) => {
                    unique_vars.extend(term.variables.keys());
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

    fn contains_any_variable<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char>{
        match self {
            Expression::Term(term) => term.contains_any_variable(variables),
            Expression::Binary { left, right, .. } => left.contains_any_variable(variables) || right.contains_any_variable(variables)   ,
            Expression::Nested(inner) => inner.contains_any_variable(variables)
        }
    }

    fn contains_all<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char> {
        match self {
            Expression::Term(term) => term.contains_all(variables),
            Expression::Binary { left, right, .. } => left.contains_all(variables) || right.contains_all(variables)   ,
            Expression::Nested(inner) => inner.contains_all(variables)
        }
    }

    fn contains_variable(&self, variable: &char) -> bool {
        match self {
            Expression::Term(term) => term.contains_variable(variable),
            Expression::Binary { left, right, .. } => left.contains_variable(variable) || right.contains_variable(variable)   ,
            Expression::Nested(inner) => inner.contains_variable(variable)
        }
    }
}