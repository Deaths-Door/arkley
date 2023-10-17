use crate::{Equation, Expression};

use super::VariableSubstitution;


impl<SV,MV> VariableSubstitution<SV,MV> for Equation where Expression : VariableSubstitution<SV,MV> , SV : Clone {
    fn replace_single_variable(&mut self, variable: &char, value: SV) -> Option<()> {
        let left = self.left.replace_single_variable(variable, value.clone());
        let right = self.right.replace_single_variable(variable, value);

        match (left,right) {
            (None,None) => None,
            _ => Some(())
        }
    }

    fn replace_variables(&mut self, variable_values:&mut MV) {
        self.left.replace_variables(variable_values);
        self.right.replace_variables(variable_values);
    }
}
/*
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
}*/