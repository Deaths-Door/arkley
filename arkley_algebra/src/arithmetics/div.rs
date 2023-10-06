use std::collections::BTreeSet;
use std::cmp::Ordering;

use crate::{Term,Expression,Variables};

impl std::ops::Div for Term {
    type Output = Expression;

    fn div(self,other : Term) -> Self::Output {
        let s_keys: BTreeSet<_> = self.variables.keys().cloned().collect();
        let o_keys: BTreeSet<_> = other.variables.keys().cloned().collect();

        let common : BTreeSet<_> = s_keys.intersection(&o_keys).cloned().collect();
        let s_unique_keys: BTreeSet<_> = s_keys.difference(&common).collect();
        let o_unique_keys: BTreeSet<_> = o_keys.difference(&common).collect();

        let mut s_variables = Variables::new();
        let mut o_variables =Variables::new();

        for key in s_unique_keys {
            s_variables.insert(*key,self.variables[key].clone());
        }

        for key in o_unique_keys {
            o_variables.insert(*key,other.variables[key].clone());
        }

        for key in common {
            let s_exponent = self.variables[&key].clone();
            let o_exponent = other.variables[&key].clone();

            let result = s_exponent - o_exponent;

            match result.partial_cmp(&0).unwrap() {
                // If the result is equal to zero, it means the exponents cancel each other out:
                // e.g., x^2 - x^2 = 0 (the terms cancel), so we ignore it.
                Ordering::Equal => {}

                // If the result is greater than zero, it means the exponent from the left term
                // is greater than the right term:
                // e.g., x^5 - x^3 = x^2, so we update the left term's exponent accordingly.
                Ordering::Greater => {
                    s_variables.insert(key,result);
                },
                
                // If the result is less than zero, it means the exponent from the right term
                // is greater than the left term:
                // e.g., x^2 - x^5 = x^-3, so we update the right term's exponent accordingly.
                Ordering::Less => {
                    o_variables.insert(key,result);
                },
            };
        }

        match s_variables.is_empty() && o_variables.is_empty() {
            // If both sets of variables are empty, it means there are no variables left,
            // and we have a simple division of coefficients:
            true => Expression::new_term(Term::new(self.coefficient / other.coefficient)),
            false => {
                let _t1 = Term::new_with_variable(self.coefficient,s_variables);
                // If the divisor is 1 and o_variables is empty (e.g., x / 1), the result is simply the dividend (x).
                match other.coefficient == 1 && o_variables.is_empty() {
                    true => Expression::new_term(_t1),
                    false => {
                        let _t2 = Term::new_with_variable(other.coefficient,o_variables);
                        Expression::new_durch(_t1.into(),_t2.into())
                    }
                }
            }
        }
    }
}

impl std::ops::Div<Term> for Expression {
    type Output = Self;
    fn div(self,other : Term) -> Self::Output {
        if other.variables.is_empty() && other.coefficient == 1 {
            return self;
        }

        match self {
            Expression::Term(t1) => t1 / other,
            _ => Expression::new_durch(self, other.into())
        }
    }
}


impl std::ops::Div for Expression {
    type Output = Self;
    fn div(self,other : Expression) -> Self::Output {
        match (self,other) {
            (Expression::Term(t1), Expression::Term(t2)) => t1 / t2,
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod term {
    use super::*;

    use num_notation::Number;
    use crate::Variables;

        // Helper function to create a Term with a single variable.
        fn create_term_with_variable(coeff: i32, var: char, exp: i32) -> Term {
            let mut variables = Variables::new();
            variables.insert(var,Number::Decimal(exp as f64));
            Term::new_with_variable(Number::Decimal(coeff as f64), variables)
        }
    
    #[test]
    fn basic() {
        let term = create_term_with_variable(1, 'x', 1);
        let n :Number = 1.0.into();

        let result = term.clone() / n;
        
        let expected = term.into();
        assert_eq!(result, expected);
    }
    
        #[test]
        fn division_with_single_variable() {
            // Test division with a single variable (x^2 / x).
            let term1 = create_term_with_variable(1, 'x', 2);
            let term2 = create_term_with_variable(1, 'x', 1);
            // (x^2 / x)
            let result = term1 / term2;
        
            let expected = Expression::new_term(create_term_with_variable(1, 'x', 1));
            assert_eq!(result, expected);
        }
    
        #[test]
        fn division_with_constants() {
            // Test division with constants (6 / 3).
            let term1 = Term::new(Number::Decimal(6.0));
            let term2 = Term::new(Number::Decimal(3.0));
            let result = term1 / term2;
            let expected = Expression::new_term(Term::new(Number::Decimal(2.0)));
            assert_eq!(result, expected);
        }
    
        #[test]
        fn division_with_common_variables() {
            // Test division with common variables (x^3 / x^2).
            let term1 = create_term_with_variable(1, 'x', 3);
            let term2 = create_term_with_variable(1, 'x', 2);
            let result = term1 / term2;
            let expected = Expression::new_term(create_term_with_variable(1, 'x', 1));
            assert_eq!(result, expected);
        }
    
        #[test]
        fn division_with_unique_variables() {
            let mut variables1 = Variables::new();
            variables1.insert('x', Number::Decimal(3.0));
            variables1.insert('y', Number::Decimal(2.0));
    
            // x^3 * y^2
            let term1 = Term::new_with_variable(Number::Decimal(1.0), variables1);
    
            let mut variables2 = Variables::new();
            variables2.insert('x',Number::Decimal(2.0));
            variables2.insert('z',Number::Decimal(1.0));
            // x^2 * z.
            let term2 = Term::new_with_variable(Number::Decimal(1.0), variables2);
    
            // (x^3 * y^2) / (x^2 * z).
            let result = term1 / term2;
    
    
            let ev1 = Variables::from([
                ('x',Number::Decimal(1.0)),
                ('y',Number::Decimal(2.0))
            ]);
    
            let et1 = Term::new_with_variable(Number::Decimal(1.0),ev1);
            
            let ev2 = Variables::from([
                    ('z',Number::Decimal(1.0))
            ]);
    
            let et2 = Term::new_with_variable(Number::Decimal(1.0),ev2);
            // (x * y^2) / z
            let expected = Expression::new_durch(
                et1.into(),
                et2.into()
            );
    
            assert_eq!(result, expected);
        }
}

#[cfg(test)]
mod expression_tests {
    use super::*;
    use crate::parse_expression;

    fn from_str(input :&str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    #[test]
    fn division_between_expressions() {
        // Create expressions.
        let expr1 = from_str("2x^2");
        let expr2 = from_str("x");

        // (2x^2) / (x) = 2x
        let result = expr1 / expr2;

        let expected = from_str("2x");
        assert_eq!(result, expected);
    }

    #[test]
    fn division_between_expression_and_term() {
        let term2 = from_str("2x");

        // Create an expression.
        let expr = from_str("2x^2");

        // (2x^2) / (2x) = x
        let result = expr / term2;

        let expected = from_str("x");
        assert_eq!(result, expected);
    }

    #[test]
    fn division_between_expression_and_constant_over1(){
        let expr = from_str("2x + 1");
        let num : Term = 1.0.into();

        let result = expr.clone() / num.clone();
        assert_eq!(result,expr);
    }

    #[test]
    fn division_between_expression_and_constant(){
        let expr = from_str("2x + 1");
        let num : Term = 3.0.into();

        let result = expr.clone() / num.clone();
        assert_eq!(result,Expression::new_durch(expr, num.into()));
    }
}
