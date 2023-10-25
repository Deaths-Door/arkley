use std::collections::BTreeSet;

use crate::Term;

use super::VariableAnalysis;

impl VariableAnalysis for Term{
    fn get_unique_variables(&self) -> BTreeSet<&char> {
        self.variables.keys().collect()   
    }

    fn contains_any_variable<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char> {
        variables.any(|key| self.variables.contains_key(key))
    }

    fn contains_all<'a,I>(&self,variables : &mut I) -> bool where I : Iterator<Item = &'a char>{
       variables.all(|c| self.variables.contains_key(c))
    }

    fn contains_variable(&self, variable: &char) -> bool {
        self.variables.contains_key(variable)
    }

    fn has_all<'a,I>(&self,iterator : &mut I) -> bool where I : Iterator<Item = &'a char> {
        self.contains_all(iterator)
    }
}