use crate::Equation;

use super::VariableAnalysis;

impl VariableAnalysis for Equation {
    fn get_unique_variables(&self) -> std::collections::BTreeSet<char> {
        let mut set = self.left.get_unique_variables();
        set.extend(self.right.get_unique_variables().into_iter());
        set
    }

    fn contains_any_variable(&self,variables : &[&char]) -> bool {
        self.left_contains_variable(variables) || self.right.contains_any_variable(variables) 
    }

    fn contains_all(&self,variables : &[&char]) -> bool {
        self.right_contains_variable(variables) && self.right.contains_all(variables)
    }
}

impl Equation {
    pub(in crate::manipulation) fn left_contains_variable(&self,variables : &[&char]) -> bool {
        self.left.contains_any_variable(variables)
    }

    pub(in crate::manipulation) fn right_contains_variable(&self,variables : &[&char]) -> bool {
        self.right.contains_any_variable(variables)
    }
}