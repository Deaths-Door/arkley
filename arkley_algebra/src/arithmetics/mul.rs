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
        Term::new_with_variables(coefficient,variables).into()
    }
}

impl std::ops::Mul<Term> for Expression {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        let expr = match self {
            Expression::Term(term) => term * other,
            // if operation == ArithmeticOperation::Durch as 3x * (3/x) can be more simpily done as (3x/1) * (3/x) then other solution
            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Durch => {
                let lexpr = *left * other;
                Expression::Binary { operation, left : Box::new(lexpr), right } // to avoid unnesscary .combine_terms()
            },

            //  if operation == ArithmeticOperation::Mal as things like 3x(4x * 3) need to be 'evaluted' inside before mal with outside 
            Expression::Binary { operation, left , right } if operation == ArithmeticOperation::Mal => (*left * *right) * other,

            Expression::Binary { operation , left , right }  => {
                let lexpr = *left * other.clone();
                let rexpr = *right * other;
                Expression::new_binary(operation,lexpr,rexpr) // to avoid unnesscary .combine_terms()
            },
            Expression::Custom(_) => Expression::new_mal(self, other),
        };

        expr.combine_terms()
    }
}


impl std::ops::Mul for Expression {
    type Output = Expression;

    fn mul(self,other : Self) -> Self::Output {
        match (self,other) {

            (Expression::Term(t1),Expression::Term(t2)) => t1 * t2,
            
            (Expression::Term(term),expr @_) | (expr @_,Expression::Term(term)) => expr * term,

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

#[cfg(test)]
mod term {
    use super::*;
    use test_case::test_case;
    
    #[test_case("2x","3x^2"," 6x^3")]
    #[test_case("2.5x^2","3.5x^3","  8.75x^5")]
    #[test_case("5x^3","2.5x^2","12.5x^5")]
    fn multiplication_tests(input1 : &str,input2 : &str,expected: &str) {
        assert_eq!(
            (Term::try_from(input1).unwrap() * Term::try_from(input2).unwrap()).to_string().replace(" ",""),
            expected.replace(" ","")
        )
    }
}

#[cfg(test)]
mod expr {
    use super::*;
    use test_case::test_case;
    
    #[test_case("2x + 3y","2z","4xz + 6yz")]
    #[test_case("5x-3y","2z","10xz - 6yz")]
    #[test_case("2w","5z - 2x  - 3y","-4wx - 6wy + 10wz")]
    #[test_case("5z - (2x + 3y)","2w","-4wx - 6wy + 10wz")]
    fn multiplication_tests(input1 : &str,input2 : &str,expected: &str) {
        assert_eq!(
            (Expression::try_from(input1).unwrap() * Expression::try_from(input2).unwrap()).to_string().replace(" ",""),
            expected.replace(" ","")
        )
    }
}