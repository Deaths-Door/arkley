use std::collections::BTreeSet;

use crate::Term;

use super::VariableAnalysis;

impl VariableAnalysis for Term {
    fn get_unique_variables(&self) -> BTreeSet<char> {
        self.variables.keys().cloned().collect()   
    }

    fn contains_any_variable(&self,variables : &[&char]) -> bool {
        variables.iter().any(|&key| self.variables.contains_key(key))
    }
}