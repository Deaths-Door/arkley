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
            Expression::Custom(value) => Self::Custom(value.negate())
        }
    }
}

#[cfg(test)] 
mod expr {
    use super::*;

    use test_case::test_case;
    
    #[test_case("5x","-5x")]
    #[test_case("3y","-3y")]
    #[test_case("2a + 3b","-2a - 3b")]

    fn test_negation(input : &str,output : &str) {
        assert_eq!((-Expression::try_from(input).unwrap()).to_string(),output)
    }
}