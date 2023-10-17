use crate::{Equation, manipulation::VariableSubstitution};

use super::Evaluate;

impl Evaluate for Equation {
    fn evaluate(mut self) -> Self {
        self.left = self.left.evaluate();
        self.right = self.right.evaluate();
        self
    }

    fn evaluate_with_single_variable<SV,MV>(mut self, variable: &char, value: SV) -> Self 
        where Self: VariableSubstitution<SV,MV>, SV: Clone {
        
        self.replace_single_variable(variable, value); // avoid compile errors
        
        self.left = self.left.evaluate();
        self.right = self.right.evaluate();

        self
    }

    fn evaluate_with_variables<SV,MV>(mut self, variable_values:&mut MV) -> Self 
        where Self: VariableSubstitution<SV,MV>, SV: Clone {
        
            self.replace_variables(variable_values); // avoid compile errors
        
            self.left = self.left.evaluate();
            self.right = self.right.evaluate();
            
            self
    }
}