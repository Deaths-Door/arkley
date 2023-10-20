use crate::{Function, manipulation::VariableSubstitution, Expression};

use super::Evaluate;


impl Evaluate<Expression> for Function {
    fn evaluate(self) -> Expression {
        (self.closure)(self)
    }

    fn evaluate_with_single_variable<SV,MV>(mut self, variable: &char, value: SV) -> Expression where Self: VariableSubstitution<SV,MV>, SV: Clone  {
        self.replace_single_variable(variable, value);
        (self.closure)(self)
    }

    fn evaluate_with_variables<SV,MV>(mut self, variable_values:&mut MV) -> Expression where Self: VariableSubstitution<SV,MV>, SV: Clone  {
        self.replace_variables(variable_values);
        (self.closure)(self)
    }
}