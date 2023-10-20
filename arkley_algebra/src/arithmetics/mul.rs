use num_notation::{One, Zero};

use crate::{Term,Expression,ArithmeticOperation};

impl std::ops::Mul for Term {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        let mut variables = self.variables;
        for (var,exponent) in other.variables {
            variables.entry(var)
                .and_modify(|e| *e += exponent.clone())
                .or_insert(exponent);
        };

        let coefficient = self.coefficient * other.coefficient;
        Term::new_with_variable(coefficient,variables).into()
    }
}

impl std::ops::Mul<Term> for Expression {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        let expr = match self {
            Expression::Nested(inner) => *inner * other,
            Expression::Term(term) => term * other,
            // if operation == ArithmeticOperation::Durch as 3x * (3/x) can be more simpily done as (3x/1) * (3/x) then other solution
            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Durch => {
                let lexpr = *left * other;
                Expression::Binary { operation,left : Box::new(lexpr), right } // to avoid unnesscary .combine_terms()
            },

            //  if operation == ArithmeticOperation::Mal as things like 3x(4x * 3) need to be 'evaluted' inside before mal with outside 
            Expression::Binary { operation, left , right } if operation == ArithmeticOperation::Mal => (*left * *right) * other,

            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Plus  => {
                let lexpr = *left * other.clone();
                let rexpr = *right * other;
                Expression::new_binary(operation,lexpr,rexpr) // to avoid unnesscary .combine_terms()
            },
            Expression::Binary { operation , left , right } /*if operation == ArithmeticOperation::Minus*/ => {
                let lexpr = *left * other.clone();
                let rexpr = -*right * other;
                Expression::new_binary(operation,lexpr,rexpr)
            },
            Expression::Function { .. } => {
                if other.variables.is_empty() {
                    return if other.coefficient.is_one() {
                        self.into()
                    }
                    else if other.coefficient.is_zero() {
                        0.0.into()
                    }
                    else {
                        Expression::new_mal(self.into(),other.into())
                    }
                }
        
                Expression::new_mal(self.into(),other.into())
            },
        };

        expr.combine_terms()
    }
}


impl std::ops::Mul for Expression {
    type Output = Expression;

    fn mul(self,other : Self) -> Self::Output {
        match (self,other) {
            (Expression::Nested(inner1),Expression::Nested(inner2)) => *inner1 * *inner2,

            (Expression::Term(t1),Expression::Term(t2)) => t1 * t2,
            (Expression::Term(term),Expression::Nested(inner)) | (Expression::Nested(inner),Expression::Term(term)) => *inner * term,
            
            (Expression::Term(term),expr @_) | (expr @_,Expression::Term(term)) => expr * term,

            (expr @_, Expression::Nested(inner)) |
            (Expression::Nested(inner),expr @_) => *inner * expr,

            // div , div
            (Expression::Binary { operation : op2, left : left1, right : right1 }, 
                Expression::Binary { operation : op1, left : left2, right : right2 }
            ) if op1 == ArithmeticOperation::Durch && op2 == ArithmeticOperation::Durch => Expression::new_durch(*left2 * *left1, *right1 * *right2),
        
            // div , any
            (
                Expression::Binary { operation, left, right }, 
                expr @_ 
            ) |
            (
                expr @_ ,
                Expression::Binary { operation, left, right }, 
            )
            if operation == ArithmeticOperation::Durch  => Expression::new_durch(*left * expr, *right),
        
        
            (
                Expression::Binary { operation : op1, left : left1, right : right1 }, 
                Expression::Binary { operation : op2, left : left2, right : right2 }
            ) => {
                /*
                (left1)   (right1)  (left2)   (right2)
                  x   -    2      *    (x   -    2)

                    left1 * left2 so x * x = x^2

                    if minus

                    -rightt1 * left2 so -2 * x
                    lfet * -right1 so -x * -2

                    -right1 * right2 so -2 * -2

                    so x^2 - 2x - 2x + 4
                */
                               
                let expr1 = *left1 * (*left2).clone();

                let new_right1 = if op1 == ArithmeticOperation::Minus { -*right1 } else { *right1 };

                let expr2 = new_right1.clone() * *left2;
                let expr3 = expr2.clone();

                let new_right2 = if op2 == ArithmeticOperation::Minus { -*right2 } else { *right2 };
                let expr4 = new_right1 * new_right2;

                let lexpr = Expression::new_plus(expr1, expr2);
                let rexpr = Expression::new_plus(expr3, expr4);

                lexpr + rexpr
            },
            (left @_,right @_) => Expression::new_mal(left, right),
        } 
    }
}

#[cfg(feature="function")]
use crate::Function;

#[cfg(feature="function")]
impl std::ops::Mul<Function > for Function  {
    type Output = Expression; 
    fn mul(self, rhs: Function ) -> Self::Output {
        // TODO : For cases like f(x) * f(x) maybe output (f(x))^2
        Expression::new_mal(self.into(), rhs.into())
    }
}

#[cfg(feature="function")]
impl std::ops::Mul<Term> for Function  {
    type Output = Expression; 
    fn mul(self, rhs: Term) -> Self::Output {
        if rhs.variables.is_empty() {
            return if rhs.coefficient.is_one() {
                self.into()
            }
            else if rhs.coefficient.is_zero() {
                0.0.into()
            }
            else {
                Expression::new_mal(self.into(),rhs.into())
            }
        }

        Expression::new_mal(self.into(),rhs.into())
    }
}

#[cfg(feature="function")]
impl std::ops::Mul<Function > for Expression {
    type Output = Expression; 
    fn mul(self, rhs: Function) -> Self::Output {
        if let Expression::Term(value) = self  {
            return rhs * value
        }
        // TODO : For cases like f(x) * f(x) maybe output (f(x))^2
        Expression::new_mal(self.into(), rhs.into())
    }
}

#[cfg(test)]
mod term {
    use super::*;

    use num_notation::Number;
    use crate::Variables;
    
    #[test]
    fn multiply_terms() {
        // 2x
        let term1 = Term::new_with_variable(Number::Decimal(2.0), Variables::from([('x', Number::Decimal(1.0))]));

        //3x^2
        let term2 = Term::new_with_variable(Number::Decimal(3.0), Variables::from([('x', Number::Decimal(2.0))]));

        // 2x * 3x^2
        let result = term1.clone() * term2.clone();

        // 6x^3
        let expected_term = Term::new_with_variable(Number::Decimal(6.0), Variables::from([('x', Number::Decimal(3.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn multiply_terms_with_same_variables_and_different_powers() {
        // 2.5x^2
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 3.5x^3
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2 * 3.5x^3
        let result = term1.clone() * term2.clone();

        // 2.5x^2 * 3.5x^3 = 8.75x^5
        let expected_term = Term::new_with_variable(Number::Decimal(8.75), Variables::from([('x', Number::Decimal(5.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }
}

#[cfg(test)]
mod expr {
    use super::*;

    use num_notation::Number;
    use crate::{Variables, manipulation::Simplify};
    // Helper function to create a Term with a single variable.
    fn create_term_with_variable(coeff: f64, var: char, exp: f64) -> Term {
        let mut variables = Variables::new();
        variables.insert(var, Number::Decimal(exp));
        Term::new_with_variable(Number::Decimal(coeff), variables)
    }    

    use crate::parse_expression;

    fn from_str(input :&str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }
    fn check_expression_str(expression : Expression,_str : &str) {
        assert_eq!(&expression.to_string(),_str)
    }

    #[test]
    fn combine_terms_with_mul() {
        let expr1 : Expression = Term::new(Number::Decimal(1.0)).into();
        let expr2 : Expression = Expression::new_mal(
            Expression::new_term(create_term_with_variable(3.0, 'x', 1.0)),
            Expression::new_plus(
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
            )
        );

        
        let result = expr1 - expr2;

        check_expression_str(result,"1 + 3x(4x)");
    }

    #[test]
    fn mul_expression_by_term_addition() {
        // Test multiplying an expression containing addition by a term
        let expression = Expression::new_plus(
            create_term_with_variable(2.0, 'x', 1.0).into(),
            create_term_with_variable(3.0, 'y', 1.0).into(),
        );

        check_expression_str(expression.clone(), "2x + 3y");

        // Create a term to multiply with the expression
        let term_to_multiply = create_term_with_variable(2.0, 'z', 1.0);

        // Multiply the expression by the term
        let result = expression * term_to_multiply;

        check_expression_str(result, "4xz + 6yz");
    }

    #[test]
    fn mul_expression_by_term_subtraction() {
        // Test multiplying an expression containing subtraction by a term
        let expression = Expression::new_minus(
            create_term_with_variable(5.0, 'x', 1.0).into(),
            create_term_with_variable(3.0, 'y', 1.0).into(),
        );

        check_expression_str(expression.clone(), "5x - 3y");

        // Create a term to multiply with the expression
        let term_to_multiply = create_term_with_variable(2.0, 'z', 1.0);

        // Multiply the expression by the term
        let result = expression * term_to_multiply;

        check_expression_str(result, "10xz - 6yz");
    }

    #[test]
    fn mul_expression_by_term_nested() {
        let term = match from_str("2w") {
            Expression::Term(term) => term,
            _ => panic!()
        };

        let expression = from_str("5z - 2x  - 3y");
        let result = expression * term;

        check_expression_str(result, "-4wx - 6wy + 10wz");
    }
    
    #[test]
    fn mul_expression_by_term_nested_expr() {
        // Test multiplying a nested expression by a term2
        let inner_expression = from_str("2x + 3y");
        let expression = Expression::new_minus(
            create_term_with_variable(5.0, 'z', 1.0).into(),
            Expression::new_nested(inner_expression),
        );

        // Create a term to multiply with the expression
        let term_to_multiply : Expression = create_term_with_variable(2.0, 'w', 1.0).into();

        let result = expression.simplify_structure() * term_to_multiply;

        check_expression_str(result, "-4wx - 6wy + 10wz");
    }
}