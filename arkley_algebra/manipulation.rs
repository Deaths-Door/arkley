use std::collections::BTreeSet;
use num_notation::Number;

use crate::{Expression,Term,Variables, ArithmeticOperation};

impl Expression {
    pub fn evalute(self) -> Self {
        match self {
            Expression::Binary { operation, left, right } => match operation {
                ArithmeticOperation::Plus => left.evalute() + right.evalute(),
                ArithmeticOperation::Minus => left.evalute() - right.evalute(),
                ArithmeticOperation::Mal => left.evalute() * right.evalute(),
                ArithmeticOperation::Durch => left.evalute() / right.evalute(),
            },
        
            Expression::Nested(inner) => inner.evalute(),
            _ => self
        }
    }

    /// Evaluates the expression with variable values.
    ///
    /// This function recursively evaluates the expression, taking into account
    /// the arithmetic operations specified in each binary node of the expression
    /// tree. It also replaces any variables in the expression with their values
    /// from the provided `variable_values` map.
    ///
    /// # Arguments
    ///
    /// * `variable_values`: A mutable reference to a map containing variable
    ///   names and their corresponding values.
    pub fn evalute_with_values(mut self,variable_values : &mut Variables) -> Self {
        match self {
            Expression::Binary { operation, mut left, mut right } => {
                left.try_replace_variables(variable_values);
                right.try_replace_variables(variable_values);
                match operation {
                    ArithmeticOperation::Plus => left.evalute() + right.evalute(),
                    ArithmeticOperation::Minus => left.evalute() - right.evalute(),
                    ArithmeticOperation::Mal => left.evalute() * right.evalute(),
                    ArithmeticOperation::Durch => left.evalute() / right.evalute(),
                }
            }
            Expression::Nested(mut inner) => {
                inner.try_replace_variables(variable_values);
                inner.evalute()
            },
            Expression::Term(ref mut term) => {
                term.try_replace_variables(variable_values);
                self 
            }
        }
    }
}
