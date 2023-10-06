use crate::{
    Expression, Variables, ArithmeticOperation, 
    manipulation::{VariableExpressionAssociation, VariableSubstitution}
};

use super::Evaluate;

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

impl Evaluate for Expression {
    fn evaluate(self) -> Self {
        match self {
            Expression::Binary { operation, left, right } => operation.operate_on(left.evaluate(), right.evaluate()),
            Expression::Nested(inner) => inner.evaluate(),
            _ => self
        }      
    }

    fn evaluate_single_variable_with_value(mut self,variable : &char,value : num_notation::Number) -> Self {
        match self {
            Expression::Binary { operation,mut left, mut right } => {
                left.try_replace_single_variable_with_value(variable,value.clone());
                right.try_replace_single_variable_with_value(variable,value.clone());
                operation.operate_on(left.evaluate_single_variable_with_value(variable, value.clone()), right.evaluate_single_variable_with_value(variable, value))
            }
            Expression::Nested(mut inner) => {
                inner.try_replace_single_variable_with_value(variable,value.clone());
                inner.evaluate_single_variable_with_value(variable,value)
            }
            Expression::Term(ref mut term) => {
                term.try_replace_single_variable_with_value(variable, value);
                self
            }
        }        
    }

    
    fn evaluate_with_values(mut self, variable_values: &mut Variables) -> Self {
        match self {
            Expression::Binary { operation,mut left, mut right } => {
                left.try_replace_variables_with_value(variable_values);
                right.try_replace_variables_with_value(variable_values);
                operation.operate_on(left.evaluate_with_values(variable_values), right.evaluate_with_values(variable_values))
            }
            Expression::Nested(mut inner) => {
                inner.try_replace_variables_with_value(variable_values);
                inner.evaluate_with_values(variable_values)
            }
            Expression::Term(ref mut term) => {
                term.try_replace_variables_with_value(variable_values);
                self
            }
        }        
    }
    fn evaluate_single_variable_with_expr(mut self,variable : &char,value : Expression) -> Self {
        match self {
            Expression::Binary { operation,mut left, mut right } => {
                left.try_replace_single_variable_with_expr(variable,value.clone());
                right.try_replace_single_variable_with_expr(variable,value.clone());
                operation.operate_on(left.evaluate_single_variable_with_expr(variable, value.clone()), right.evaluate_single_variable_with_expr(variable, value))
            }
            Expression::Nested(mut inner) => {
                inner.try_replace_single_variable_with_expr(variable,value.clone());
                inner.evaluate_single_variable_with_expr(variable,value)
            }
            Expression::Term(ref mut term) => {
                term.try_replace_single_variable_with_expr(variable, value);
                self
            }
        }    
    }

    fn evaluate_with_expr(mut self, variable_values: &mut VariableExpressionAssociation) -> Self {
        match self {
            Expression::Binary { operation,mut left, mut right } => {
                left.try_replace_variables_with_expr(variable_values);
                right.try_replace_variables_with_expr(variable_values);
                operation.operate_on(left.evaluate_with_expr(variable_values), right.evaluate_with_expr(variable_values))
            }
            Expression::Nested(mut inner) => {
                inner.try_replace_variables_with_expr(variable_values);
                inner.evaluate_with_expr(variable_values)
            }
            Expression::Term(ref mut term) => {
                term.try_replace_variables_with_expr(variable_values);
                self
            }
        }       
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::parse_expression;

    fn from_str(input :&str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    fn check_expression_str(result : Expression,_str : &str) {
        assert_eq!(&result.to_string(),_str)
    }

    #[test]
    fn test_evaluate_addition() {
        // Create an expression: 2 + 3
        let expression = from_str("2 + 3");
        
        // Evaluate the expression
        let result = expression.evaluate();

        // Expected result: 2 + 3 = 5
        check_expression_str(result, "5");
    }

    #[test]
    fn test_evaluate_multiplication() {
        // Create an expression: 4 * 5
        let expression = from_str("20");

        // Evaluate the expression
        let result = expression.evaluate();

        // Expected result: 4 * 5 = 20
        check_expression_str(result, "20");
    }

    #[test]
    fn test_evaluate_nested_expression() {
        // Create a nested expression: (2 + 3) * 4
        let expression = Expression::new_mal(from_str("2 + 3"), from_str("4"));

        // Evaluate the expression
        let result = expression.evaluate();

        // Expected result: (2 + 3) * 4 = 20
        check_expression_str(result, "20");
    }

    #[test]
    fn test_evaluate_with_variables() {
        // Create an expression with a variable: x + y
        let expression = from_str("x + y");

        // Create a map of variable values
        let mut variable_values = Variables::new();
        variable_values.insert('x', 7.0.into());
        variable_values.insert('y', 3.0.into());

        let mut __expr = expression.clone();
        __expr.try_replace_variables_with_value(&mut variable_values.clone());

        check_expression_str(__expr, "7 + 3");

        // Evaluate the expression with variable values
        let result = expression.evaluate_with_values(&mut variable_values);

        // Expected result: 7 + 3 = 10
        check_expression_str(result,"10");
    }
}
