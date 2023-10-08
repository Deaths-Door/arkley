use std::collections::{BTreeSet,BTreeMap};
use std::cmp::Ordering;

use num_notation::Number;

use crate::{Term, Expression, manipulation::VariableAnalysis};

impl std::ops::Div for Term {
    type Output = Expression;

    fn div(mut self,mut other : Term) -> Self::Output {
        if other.coefficient == 1 && other.variables.is_empty() {
            return self.into();
        };

        if self.variables.is_empty() && other.variables.is_empty() {
            return Term::new(self.coefficient / other.coefficient).into()
        }

        let s_keys: BTreeSet<_> = self.variables.keys().cloned().collect();
        let o_keys: BTreeSet<_> = other.variables.keys().cloned().collect();
        
        let common_variables : BTreeSet<_> = s_keys.intersection(&o_keys).collect();

        for key in common_variables {
            match self.variables[key].cmp(&other.variables[key]) {
                Ordering::Equal => {
                    self.variables.remove(key);
                    other.variables.remove(key);
                },

                // If the result is greater than zero, it means the exponent from the left term
                // is greater than the right term:
                // e.g., x^5 - x^3 = x^3 / 1
                Ordering::Greater => {
                    let s_exponent = self.variables.get_mut(key).unwrap();
                    *s_exponent -= other.variables[key].clone();
                    other.variables.remove(key);
                },

                // If the result is less than zero, it means the exponent from the right term
                // is greater than the left term:
                // e.g., x^2 - x^5 = 1 / x^3
                Ordering::Less => {
                    let o_exponent = other.variables.get_mut(key).unwrap();
                    *o_exponent -= self.variables[key].clone();
                    self.variables.remove(key);
                },
            }
        }

        if self.variables.is_empty() && other.variables.is_empty() {
            return Term::new(self.coefficient / other.coefficient).into()
        }

        match other.coefficient == 1 && other.variables.is_empty() {
            true => self.into(),
            false => Expression::new_durch(self.into(), other.into())
        }
    }
}

impl std::ops::Div<Term> for Expression {
    type Output = Self;
    fn div(mut self,mut other : Term) -> Self::Output {
        if other.coefficient == 1 && other.variables.is_empty() {
            return self.into();
        };

        let expr_variables = self.get_unique_variables();
        let term_variables = other.get_unique_variables();

        let common_variables : BTreeSet<_> = expr_variables.intersection(&term_variables).collect();

        let mut derefernced_char_common_variables_iterator = common_variables.iter().map(|c| **c);
        if common_variables.is_empty() || !self.all_terms_have(&mut derefernced_char_common_variables_iterator) {
            return Expression::new_durch(self, other.into());
        };
        
        let mut min_exponents = BTreeMap::new();
        let mut coefficients = BTreeSet::new();

        // TODO : Impl without clone
        let __other_clone = other.clone();
        __other_clone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);

        let __self_clone = self.clone();
        __self_clone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);

        // Figure out gcd to divide coefficients by
        let gcd_coefficient = figure_out_gcd(coefficients);
        
        self.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient.clone());
        other.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient);

        match other.coefficient == 1 && other.variables.is_empty() {
            true => self,
            false => Expression::new_durch(self, other.into())
        }
    }
}

impl std::ops::Div for Expression {
    type Output = Self;
    fn div(self,other : Expression) -> Self::Output {
        match (self,other) {
            (Expression::Term(t1), Expression::Term(t2)) => t1 / t2,
            (mut top @_,mut bottom @_)=> {
                let top_variables = top.get_unique_variables();
                let bottom_variables = bottom.get_unique_variables();
        
                let common_variables : BTreeSet<_> = top_variables.intersection(&bottom_variables).collect();
        
                let mut derefernced_char_common_variables_iterator = common_variables.iter().map(|c| **c);

                if common_variables.is_empty() ||
                     !top.all_terms_have(&mut derefernced_char_common_variables_iterator) || 
                     !bottom.all_terms_have(&mut derefernced_char_common_variables_iterator) {
                    return Expression::new_durch(top,bottom);
                };

                let mut min_exponents = BTreeMap::new();
                let mut coefficients = BTreeSet::new();
        
                // TODO : Impl without clone
                let __self_clone = top.clone();
                __self_clone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);

                let __other_clone = bottom.clone();
                __other_clone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);

                // Figure out gcd to divide coefficients by
                let gcd_coefficient = figure_out_gcd(coefficients);

                top.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient.clone());
                bottom.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient);

                Expression::new_durch(top,bottom)
            }
        }
    }
}

fn gcd(a : Number,b : Number) -> Number {
    match b == 0 {
        true => a,
        false => gcd(b.clone(),a % b)
    }
}

fn figure_out_gcd(mut coefficients: BTreeSet<Number>) -> Number {
    if coefficients.len() == 1 {
        return coefficients.pop_first().unwrap();
    };

    let mut gcd_coefficients : Number = coefficients.pop_first().unwrap();

    let __one : Number = 1.0.into();

    for item in coefficients.into_iter() {
        gcd_coefficients = gcd(item,gcd_coefficients);

        if gcd_coefficients == __one {
            gcd_coefficients = __one;
            break;
        }
    }

    gcd_coefficients
}

impl Term {
    /// Recursively calculates the minimum exponent values for common variables between two expression trees.
    ///
    /// This method inserts the term's coefficient into the `coefficients` set and calculates
    /// the minimum exponents for common variables, storing them in the `min_exponents` map.
    ///
    /// # Arguments
    ///
    /// - `common_variables`: A set containing common variables shared with another expression.
    /// - `min_exponents`: A mutable map to store the minimum exponents for common variables.
    /// - `coefficients`: A mutable set to store the term's coefficients.
    /// between two expression trees.
    fn get_min_exponents_and_coefficient<'a>(&'a self,common_variables : &BTreeSet<&&char>,min_exponents : &mut BTreeMap<&'a char, &'a Number>,coefficients :&mut BTreeSet<Number>) {
        coefficients.insert(self.coefficient.clone());

        self.variables.keys()
            .filter(|key| common_variables.contains(key))
            .for_each(|key|{
                let possible_value = self.variables.get(key).unwrap();
                min_exponents.entry(key)
                    .and_modify(|prev| *prev = &(*prev).min(possible_value))
                    .or_insert(possible_value);
            });
    }

    /// Cancels common variables and divides the coefficient by the greatest common divisor (GCD).
    ///
    /// So 2x^2 / 2x = x with this method
    /// 
    /// This method divides the term's coefficient by `gcd_coefficient` and cancels common variables
    /// based on the minimum exponents provided in `min_exponents`.
    ///
    /// # Arguments
    ///
    /// - `min_exponents`: A map containing the minimum exponents for common variables.
    /// - `gcd_coefficient`: The greatest common divisor (GCD) of the coefficients.
    fn cancel_variables_and_divide_coefficient(&mut self,min_exponents : &BTreeMap<&char, &Number>,gcd_coefficient : Number) {
        self.coefficient /= gcd_coefficient;

        for (key,min_exponent) in min_exponents {
            match &(*min_exponent).cmp(self.variables.get(key).unwrap()) {
                Ordering::Less => *self.variables.get_mut(key).unwrap() -= (*min_exponent).clone(),
                Ordering::Equal  | Ordering::Greater => {
                    self.variables.remove(key);
                }
            }
        }
    }
}

impl Expression {
    /// Recursively checks if all terms within the expression contain a set of common variables.
    ///
    /// This method takes an iterator of character references and checks if all terms within the
    /// expression contain these common variables.
    ///
    /// # Parameters
    ///
    /// - `iterator`: A mutable reference to an iterator yielding character references.
    ///
    /// # Returns
    ///
    /// Returns `true` if all terms within the expression contain the common variables, or `false`
    /// otherwise
    fn all_terms_have<'a,I>(&self,iterator : &mut I) -> bool where I : Iterator<Item = &'a char> + Clone {
        match self {
            Expression::Term(term) => term.contains_all(iterator),
            Expression::Binary { left, right, .. } => left.all_terms_have(iterator) && right.all_terms_have(iterator),
            Expression::Nested(inner) => inner.all_terms_have(iterator),
        }
    }

    /// Recursively calculates the minimum exponent values and coefficients for common variables
    /// within the expression.
    ///
    /// This method computes the minimum exponent values and collects coefficients for common
    /// variables across all terms within the expression.
    ///
    /// # Parameters
    ///
    /// - `common_variables`: A reference to a set containing character references representing
    ///   common variables.
    /// - `min_exponents`: A mutable reference to a BTreeMap that will store the minimum exponent
    ///   values for common variables.
    /// - `coefficients`: A mutable reference to a BTreeSet that will store coefficients from all
    ///   terms.
    fn get_min_exponents_and_coefficient<'a>(&'a self,common_variables : &BTreeSet<&&char>,min_exponents : &mut BTreeMap<&'a char, &'a Number>,coefficients :&mut BTreeSet<Number>) {
        match self {
            Expression::Term(term) => term.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients),
            Expression::Binary { left, right, .. } => {
                left.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients);
                right.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients);
            },
            Expression::Nested(inner) => {
                inner.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients);
            }
        }
    }

    /// Recursively cancels common variables and divides coefficients within the expression.
    ///
    /// This method cancels out common variables based on their minimum exponents and divides all
    /// coefficients within the expression by a common divisor (GCD - Greatest Common Divisor).
    ///
    /// # Parameters
    ///
    /// - `min_exponents`: A reference to a BTreeMap containing minimum exponent values for common
    ///   variables.
    /// - `gcd_coefficient`: The common divisor used to divide coefficients.
    fn cancel_variables_and_divide_coefficient(&mut self,min_exponents : &BTreeMap<&char, &Number>,gcd_coefficient : Number) {
        match self {
            Expression::Term(ref mut term) => term.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient),
            Expression::Binary { left, right, .. } => {
                left.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient.clone());
                right.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient);
            },
            Expression::Nested(inner) => inner.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient),
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
        let expr2 = from_str("x + 2x^2");
        // (2x^2) / (x + 2x^2 ) = 2x / (2x + 1)
        let result = expr1 / expr2;

        let _top = from_str("2x");
        let _bottom = from_str("1 + 2x"); // humans rewrite it as 2x + 1 
        let expected = Expression::new_durch(_top, _bottom);
        assert_eq!(result, expected);
    }

    #[test]
    fn division_between_expression_and_term() {
        let term2 = if let Expression::Term(term) = from_str("2x") {
            term
        } else { panic!() };

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