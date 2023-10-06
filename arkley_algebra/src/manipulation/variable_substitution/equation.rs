use crate::Equation;

use super::VariableSubstitution;


impl VariableSubstitution for Equation {
    fn try_replace_single_variable_with_value(&mut self,variable : &char,value : num_notation::Number) -> Option<()> {
        let left = self.left.try_replace_single_variable_with_value(variable, value.clone());
        let right = self.right.try_replace_single_variable_with_value(variable, value.clone());

        match (left,right) {
            (None,None) => None,
            _ => Some(())
        }
    }

    fn try_replace_variables_with_value(&mut self,variable_values : &mut crate::Variables) {
        self.left.try_replace_variables_with_value(variable_values);
        self.right.try_replace_variables_with_value(variable_values);
    }
}