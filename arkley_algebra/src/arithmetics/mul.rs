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
        Expression::new_term(Term::new_with_variable(coefficient,variables))
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
            Expression::Binary { operation,.. /*, left , right*/ } if operation == ArithmeticOperation::Mal => todo!(),//(*left * *right) * other,

            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Plus  => {
                let lexpr = *left * other.clone();
                let rexpr = *right * other;
                Expression::new_binary(operation,lexpr,rexpr) // to avoid unnesscary .combine_terms()
            },
            Expression::Binary { operation , left , right } /*if operation == ArithmeticOperation::Minus*/ => {
                let lexpr = *left * other.clone();
                // don't change if true statement so term * other
                let rexpr = if let Expression::Term(term) = *right { term * other } else { *right * other };
                Expression::new_binary(operation,lexpr,rexpr)
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

            (Expression::Term(term),x @ _) | (x @ _,Expression::Term(term)) => x * term, 
            (Expression::Nested(inner),x @ _) | (x @ _,Expression::Nested(inner)) => *inner * x,

            /*// if both + + => nichts,
            // if + - or - + => smth ,
            // if + and * then add then * then * ,
            // if - and * then sub then * then * ,
            // if + and / then add then / then * ,
            // if - and / then sub then / then *
            (Expression::Binary { operation: operation1, left : left1, right : right1 },Expression::Binary { operation: operation2, left : left2, right : right2 }) => match (operation1,operation2) {
                (ArithmeticOperation::Plus,ArithmeticOperation::Plus) => {
                    // Basically doing (x+2)(x+2)
                    let lexpr1 = *left1 * *left2;
                    let lexpr2 = *left1 * *right2;

                    let rexpr1 = *right1 * *left2;
                    let rexpr2 = *right1 * *right2;

                    lexpr1 + lexpr2 + rexpr1 + rexpr2
                }
                
                // (x+1)(x*x2) => so evalute * then left1 * eval and right1 * eval    
                (ArithmeticOperation::Plus,ArithmeticOperation::Mal) => {
                    let _reval_expr = *left2 * *right2;

                    let lexpr = _reval_expr * *left1;
                    let rexpr = _reval_expr * *right1;

                    lexpr + rexpr
                }
                (ArithmeticOperation::Mal,ArithmeticOperation::Plus) => {
                    let _leval_expr = *left1 * *right1;

                    let lexpr = _leval_expr * *left2;
                    let rexpr = _leval_expr * *right2;

                    lexpr + rexpr
                }
        
                (ArithmeticOperation::Plus,ArithmeticOperation::Durch) => {
                    let _reval_expr = *left2 / *right2;

                    let lexpr = _reval_expr * *left1;
                    let rexpr = _reval_expr * *right1;

                    lexpr + rexpr
                }
                (ArithmeticOperation::Durch,ArithmeticOperation::Plus) => {
                    let _leval_expr = *left1 / *right1;

                    let lexpr = _leval_expr * *left2;
                    let rexpr = _leval_expr * *right2;

                    lexpr + rexpr
                }
                
                (ArithmeticOperation::Plus,ArithmeticOperation::Minus) => todo!(),
                (ArithmeticOperation::Minus,ArithmeticOperation::Plus) => todo!(),

                (ArithmeticOperation::Mal,ArithmeticOperation::Minus) => todo!(),
                (ArithmeticOperation::Minus,ArithmeticOperation::Mal) => todo!(),

                (ArithmeticOperation::Minus,ArithmeticOperation::Durch) => todo!(),
                (ArithmeticOperation::Durch,ArithmeticOperation::Minus) => todo!(),
                (ArithmeticOperation::Minus, ArithmeticOperation::Minus) => todo!(),
                (ArithmeticOperation::Mal, ArithmeticOperation::Mal) => todo!(),
                (ArithmeticOperation::Mal, ArithmeticOperation::Durch) => todo!(),
                (ArithmeticOperation::Durch, ArithmeticOperation::Mal) => todo!(),
                (ArithmeticOperation::Durch, ArithmeticOperation::Durch) => todo!(),
            },*/
        }
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
        // Test multiplying a nested expression by a term
        let inner_expression = Expression::new_plus(
            create_term_with_variable(2.0, 'x', 1.0).into(),
            create_term_with_variable(3.0, 'y', 1.0).into(),
        );

        let expression = Expression::new_minus(
            create_term_with_variable(5.0, 'z', 1.0).into(),
            Expression::new_nested(inner_expression),
        );

        check_expression_str(expression.clone(), "5z - (2x + 3y)");

        // Create a term to multiply with the expression
        let term_to_multiply = create_term_with_variable(2.0, 'w', 1.0);

        // Multiply the expression by the term
        // 2w (5z - (2x + 3y))
        // 2w (5z - 2x + 3y)
        // 10wz - 4xw + 6wy
        let result = expression * term_to_multiply;

        check_expression_str(result, "10wz - 4wx + 6wy");
    }
}