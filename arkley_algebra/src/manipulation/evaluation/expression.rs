use crate::{
    Expression, ArithmeticOperation, 
    manipulation::VariableSubstitution
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
            Self::Binary { operation, left, right } => operation.operate_on(left.evaluate(), right.evaluate()),
            Self::Nested(inner) => inner.evaluate(),
            Self::Function(func) => func.evaluate(),
            Self::Term(_) => self,
        }      
    }

    fn evaluate_with_single_variable<SV,MV>(mut self, variable: &char, value: SV) -> Self 
        where Self: VariableSubstitution<SV,MV>, SV: Clone {
            self.replace_single_variable(variable, value);
            self.evaluate()
    }

    fn evaluate_with_variables<SV,MV>(mut self, variable_values:&mut MV) -> Self 
        where Self: VariableSubstitution<SV,MV>, SV: Clone {
            self.replace_variables(variable_values);
            self.evaluate()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use num_notation::Number;
    use super::*;

    fn from_str(input :&str) -> Expression {
        Expression::try_from((input,&Default::default())).unwrap()
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
        let mut variable_values : HashMap<char,Number> = HashMap::new();
        variable_values.insert('x', 7.0.into());
        variable_values.insert('y', 3.0.into());

        // Evaluate the expression with variable values
        let result = expression.evaluate_with_variables(&mut variable_values);

        // Expected result: 7 + 3 = 10
        check_expression_str(result,"10");
    }
}
