use num_notation::Number;

use crate::{
    Equation ,Expression, 
    manipulation::VariableExpressionAssociation
};

use super::Evaluate;

impl Evaluate for Equation {
    fn evaluate(mut self) -> Self {
        self.left = self.left.evaluate();
        self.right = self.right.evaluate();
        self
    }

    fn evaluate_single_variable_with_value(mut self,variable : &char,value : Number) -> Self {
        self.left = self.left.evaluate_single_variable_with_value(variable,value.clone());
        self.right = self.right.evaluate_single_variable_with_value(variable,value);
        self
    }

    fn evaluate_with_values(mut self, variable_values: &mut crate::Variables) -> Self {
        self.left = self.left.evaluate_with_values(variable_values);
        self.right = self.right.evaluate_with_values(variable_values);
        self
    }

    fn evaluate_single_variable_with_expr(mut self,variable : &char,value : Expression) -> Self {
        self.left = self.left.evaluate_single_variable_with_expr(variable,value.clone());
        self.right = self.right.evaluate_single_variable_with_expr(variable,value);
        self
    }

    fn evaluate_with_expr(mut self, variable_values: &mut VariableExpressionAssociation) -> Self {
        self.left = self.left.evaluate_with_expr(variable_values);
        self.right = self.right.evaluate_with_expr(variable_values);
        self
    }
}