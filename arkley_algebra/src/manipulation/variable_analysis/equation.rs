use crate::Equation;

use super::VariableAnalysis;

impl VariableAnalysis for Equation {
    fn get_unique_variables(&self) -> std::collections::BTreeSet<&char> {
        let mut set = self.left.get_unique_variables();
        set.extend(self.right.get_unique_variables().into_iter());
        set
    }

    fn contains_any_variable<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char> {
        self.left.contains_any_variable(variables) || self.right.contains_any_variable(variables)
    }

    fn contains_all<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char> {
        self.left.contains_all(variables) && self.right.contains_all(variables) 
    }

    fn contains_variable(&self, variable: &char) -> bool {
        self.left.contains_variable(variable) || self.right.contains_variable(variable)
    }
}