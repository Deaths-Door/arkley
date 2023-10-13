use crate::{Expression, ArithmeticOperation};

impl Expression {
    /// Removes unnecessary parentheses from the expression, focusing on addition and subtraction operations.
    ///
    /// This method is designed to simplify expressions with addition and subtraction operations by removing
    /// unnecessary parentheses around sub-expressions. Parentheses are retained only when they are necessary
    /// for correct evaluation according to the order of operations.
    pub fn remove_unnecessary_parentheses(self) -> Self {
        match self {
            Expression::Binary { operation, mut left, mut right } if operation == ArithmeticOperation::Plus => {
                if let Expression::Nested(inner) = *left {
                    left = inner
                };

                if let Expression::Nested(inner) = *right {
                    right = inner
                };

                Expression::Binary { operation , left, right } 
            },
            Expression::Binary { operation, mut left, mut right } => {
                *left = left.remove_unnecessary_parentheses();

                if operation != ArithmeticOperation::Minus {
                    *right = right.remove_unnecessary_parentheses();
                }

                Expression::Binary { operation , left, right } 
            }
            Expression::Nested(inner) => inner.remove_unnecessary_parentheses(),
            _ => self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Expression;
    
    #[test]
    fn remove_unnecessary_parentheses_addition() {
        // Test removing unnecessary parentheses in an addition expression.
        let mut expression = Expression::new_nested(
            Expression::new_plus(
                Expression::new_nested( Expression::new_plus('x'.into(), 2.0.into())), 
                'y'.into()
            )
        );
        expression = expression.remove_unnecessary_parentheses();//Expression::parse("((x + 2) + y)").unwrap();
        assert_eq!(expression.to_string(), "x + 2 + y");
    }

    #[test]
    fn remove_unnecessary_parentheses_subtraction() {
        // Test removing unnecessary parentheses in a subtraction expression.
        let mut expression = Expression::new_nested(
            Expression::new_minus(
                Expression::new_nested( Expression::new_plus('x'.into(), 2.0.into())), 
                'y'.into()
            )
        );// ((x+2)-y)
        expression = expression.remove_unnecessary_parentheses();
        assert_eq!(expression.to_string(), "x + 2 - y");
    }

    #[test]
    fn remove_unnecessary_parentheses_nested() {
        // Test removing unnecessary parentheses in a nested expression.
        let mut expression = Expression::new_nested(
            Expression::new_nested(
                Expression::new_nested(
                    Expression::new_plus('a'.into(), 'b'.into())
                )
            )
        );
        expression = expression.remove_unnecessary_parentheses();//Expression::parse("(((a + b)))").unwrap();
        assert_eq!(expression.to_string(), "a + b");
    }

    #[test]
    fn remove_unnecessary_parentheses_no_change() {
        // Test an expression with no unnecessary parentheses, should remain unchanged.
        let mut  expression = Expression::new_mal(
            Expression::new_plus('x'.into(), 'y'.into()), 
            'x'.into()
        );
        expression = expression.remove_unnecessary_parentheses();
        //Expression::parse("(x + y) * (x)").unwrap();
        assert_eq!(expression.to_string(), "(x + y)(x)");
    }
}
