use std::ops::Neg;

use crate::{Term,Expression,ArithmeticOperation};

impl Neg for Term {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.coefficient = -self.coefficient;
        self
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {     
        match self {
            Expression::Term(term) => Expression::new_term(-term),
            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Plus => Expression::new_binary(ArithmeticOperation::Minus ,-*left,*right),
            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Minus => Expression::new_binary(ArithmeticOperation::Plus,-*left,-*right),
            Expression::Binary { operation , left , right } => Expression::new_binary(operation,-*left,-*right),
            Expression::Function { ..  } => Expression::new_minus(0.into(), self),
        }
    }
}

#[cfg(feature="function")]
use crate::Function;

#[cfg(feature="function")]
impl Neg for Function  {
    type Output = Expression; 
    fn neg(self) -> Self::Output {
        Expression::new_minus(0.into(), self.into())
    }
}

#[cfg(test)]
mod expr {
    use super::*;

    use num_notation::Number;
    use crate::Variables;

    // Helper function to create a Term with a single variable.
    fn create_term_with_variable(coeff: f64, var: char, exp: f64) -> Term {
        let mut variables = Variables::new();
        variables.insert(var, Number::Decimal(exp));
        Term::new_with_variable(Number::Decimal(coeff), variables)
    }   

    fn check_expression_str(expression : Expression,_str : &str) {
        assert_eq!(&expression.to_string(),_str)
    }

    #[test]
    fn negate_term() {
        let expression : Expression = create_term_with_variable(5.0, 'x', 1.0).into();
        let negated = -expression;

        check_expression_str(negated, "-5x");
    }

    #[test]
    fn negate_nested() {
        let expression : Expression = create_term_with_variable(3.0, 'y', 1.0).into();
        let negated = -expression;

        check_expression_str(negated, "-3y");
    }

    #[test]
    fn negate_binary_expression() {
        let expression = Expression::new_plus(create_term_with_variable(2.0, 'a', 1.0).into(),create_term_with_variable(3.0, 'b', 1.0).into());
        let negated = -expression;

        check_expression_str(negated, "-2a - 3b");
    }
}