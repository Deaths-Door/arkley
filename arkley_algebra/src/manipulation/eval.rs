use crate::{Expression,Variables, ArithmeticOperation, ReplaceVariables};

impl ArithmeticOperation {
    fn operate_on(&self,left : Expression,right : Expression) -> Expression {
        match self {
            ArithmeticOperation::Plus => left + right,
            ArithmeticOperation::Minus => left - right,
            ArithmeticOperation::Mal => left * right,
            ArithmeticOperation::Durch => left / right,
        }
    }
}

impl Expression {
    /// Evaluates the expression.
    ///
    /// This function recursively evaluates the expression, taking into account
    /// the arithmetic operations specified in each binary node of the expression
    /// tree.
    ///
    /// # Returns
    ///
    /// The result of the expression evaluation.
    pub fn evalute(self) -> Self {
        match self {
            Expression::Binary { operation, left, right } => operation.operate_on(left.evalute(),right.evalute()),
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
    ///
    /// # Returns
    ///
    /// The result of the expression evaluation.
    pub fn evalute_with_values(mut self,variable_values : &mut Variables) -> Self {
        match self {
            Expression::Binary { operation, mut left, mut right } => {
                left.try_replace_variables_with_value(variable_values);
                right.try_replace_variables_with_value(variable_values);
                operation.operate_on(left.evalute(),right.evalute())
            }
            Expression::Nested(mut inner) => {
                inner.try_replace_variables_with_value(variable_values);
                inner.evalute()
            },
            Expression::Term(ref mut term) => {
                term.try_replace_variables_with_value(variable_values);
                self 
            }
        }
    }
}