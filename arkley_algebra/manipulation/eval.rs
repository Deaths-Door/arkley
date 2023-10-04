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
    pub fn evaluate(self) -> Self {
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
    pub fn evaluate_with_values(mut self,variable_values : &mut Variables) -> Self {
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

    pub fn evaluate_with_expr(mut self,variable_values : &mut Variables) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_evaluate_addition() {
        // Create an expression: 2 + 3
        let expression = Expression::Binary {
            operation: ArithmeticOperation::Add,
            left: Box::new(Expression::Number(2.0)),
            right: Box::new(Expression::Number(3.0)),
        };

        // Evaluate the expression
        let result = expression.evaluate();

        // Expected result: 2 + 3 = 5
        assert_eq!(result, Expression::Number(5.0));
    }

    #[test]
    fn test_evaluate_multiplication() {
        // Create an expression: 4 * 5
        let expression = Expression::Binary {
            operation: ArithmeticOperation::Multiply,
            left: Box::new(Expression::Number(4.0)),
            right: Box::new(Expression::Number(5.0)),
        };

        // Evaluate the expression
        let result = expression.evaluate();

        // Expected result: 4 * 5 = 20
        assert_eq!(result, Expression::Number(20.0));
    }

    #[test]
    fn test_evaluate_nested_expression() {
        // Create a nested expression: (2 + 3) * 4
        let expression = Expression::Binary {
            operation: ArithmeticOperation::Multiply,
            left: Box::new(Expression::Binary {
                operation: ArithmeticOperation::Add,
                left: Box::new(Expression::Number(2.0)),
                right: Box::new(Expression::Number(3.0)),
            }),
            right: Box::new(Expression::Number(4.0)),
        };

        // Evaluate the expression
        let result = expression.evaluate();

        // Expected result: (2 + 3) * 4 = 20
        assert_eq!(result, Expression::Number(20.0));
    }

    #[test]
    fn test_evaluate_with_variables() {
        // Create an expression with a variable: x + y
        let expression = Expression::Binary {
            operation: ArithmeticOperation::Add,
            left: Box::new(Expression::Term(Term::Variable("x".to_string()))),
            right: Box::new(Expression::Term(Term::Variable("y".to_string()))),
        };

        // Create a map of variable values
        let mut variable_values = HashMap::new();
        variable_values.insert("x".to_string(), 7.0);
        variable_values.insert("y".to_string(), 3.0);

        // Evaluate the expression with variable values
        let result = expression.evaluate_with_values(&mut variable_values);

        // Expected result: 7 + 3 = 10
        assert_eq!(result, Expression::Number(10.0));
    }
}
