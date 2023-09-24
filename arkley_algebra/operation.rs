impl ArithmeticOperation {
    pub(crate) const fn negate_if_plus_or_minus(self) -> Self {
        use ArithmeticOperation::*;
        match self {
            Plus => Minus,
            Minus => Plus,
            _ => self
        }
        }
    }

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {     
        match self {
            Expression::Term(term) => Expression::new_term(-term),
            Expression::Nested(inner) => Expression::new_nested(-*inner),
            Expression::Binary { operation , left , right } => Expression::new_binary(operation.negate_if_plus_or_minus(),-*left,-*right)
        }
    }
}

    #[test]
    fn test_neg() {
        // x + x - (x)
        let expression = Expression::new_plus(
            create_term_with_variable(1.0,'x',1.0).into(),
            Expression::new_minus(
                create_term_with_variable(1.0,'x',1.0).into(),
                Expression::new_nested(create_term_with_variable(1.0,'x',1.0).into())
            )
        );

        let negated_expression = -expression;

        // x - x + (-x)
        let expected_expression = Expression::new_minus(
            create_term_with_variable(-1.0,'x',1.0).into(),
            Expression::new_plus(
                create_term_with_variable(-1.0,'x',1.0).into(),
                Expression::new_nested(create_term_with_variable(-1.0,'x',1.0).into())
            )
        );

        assert_eq!(negated_expression, expected_expression);
    }
}