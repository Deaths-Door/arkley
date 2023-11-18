use std::collections::{BTreeSet,HashMap, HashSet};
use std::cmp::Ordering;

use num_notation::{Number, One};

use crate::{Term, Expression, manipulation::VariableAnalysis};

impl Term  {
    pub(crate) fn is_numeric_one(&self) -> bool {
        self.coefficient.is_one() && self.variables.is_empty()
    }

    fn both_contain_no_variables(&self,other : &Self) -> bool {
        self.variables.is_empty() && other.variables.is_empty()
    }
}

#[cfg(feature="function")]
use crate::Function;

#[cfg(feature="function")]
impl std::ops::Div<Function> for Function  {
    type Output = Expression; 
    fn div(self, rhs: Function ) -> Self::Output {
        if self.same(&rhs) { 1.into() } else { Expression::new_durch(self,rhs) }
    }
}

#[cfg(feature="function")]
impl std::ops::Div<Term> for Function  {
    type Output = Expression; 
    fn div(self, other: Term) -> Self::Output {
        if other.is_numeric_one() {
            return self.into();
        };

        Expression::new_durch(self,other)
    }
}

#[cfg(feature="function")]
impl std::ops::Div<Function > for Expression {
    type Output = Expression; 
    fn div(self, rhs: Function) -> Self::Output {
        match self {
            Self::Function(func) if func == rhs => 1.into(),
            _ => Expression::new_durch(self,rhs)
        }        
    }
}

impl std::ops::Div for Term {
    type Output = Expression;

    fn div(mut self,mut other : Term) -> Self::Output {
        if other.is_numeric_one() {
            return self.into();
        };

        if self.both_contain_no_variables(&other) {
            return Term::new(self.coefficient / other.coefficient).into()
        }

        let s_keys: BTreeSet<_> = self.get_unique_variables();
        let o_keys: BTreeSet<_> = other.get_unique_variables();
        
        let common_variables : BTreeSet<_> = s_keys.intersection(&o_keys).collect();

        let mut min_exponents = HashMap::new();

        let sclone = self.clone();
        let oclone = other.clone();
        
        sclone.get_min_exponents(&common_variables,&mut min_exponents);
        oclone.get_min_exponents(&common_variables,&mut min_exponents);

        let gcd_coefficient = gcd(sclone.coefficient.clone(),oclone.coefficient.clone());

        self.cancel_variables_and_divide_coefficient(&min_exponents, gcd_coefficient.clone());
        other.cancel_variables_and_divide_coefficient(&min_exponents, gcd_coefficient);

        match other.is_numeric_one() && other.variables.is_empty() {
            true => self.into(),
            false => Expression::new_durch(self, other)
        }
    }
}

impl std::ops::Div<Term> for Expression {
    type Output = Self;
    fn div(mut self,mut other : Term) -> Self::Output {
        if other.is_numeric_one() {
            return self 
        }

        if let Expression::Term(term) = self {
            return term / other;
        }
        
        let expr_variables = self.get_unique_variables();
        let term_variables = other.get_unique_variables();

        let common_variables : BTreeSet<_> = expr_variables.intersection(&term_variables).collect();
        
        let mut min_exponents = HashMap::new();
        let mut coefficients = HashSet::new();
        
        let sclone = self.clone();
        let oclone = other.clone();

        sclone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);
        oclone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);

        // Figure out gcd to divide coefficients by
        let gcd_coefficient = calculate_gcd(coefficients);
        
        self.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient.clone());
        other.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient);

        match other.is_numeric_one() && other.variables.is_empty() {
            true => self,
            false => Expression::new_durch(self, other)
        }
    }
}

impl std::ops::Div for Expression {
    type Output = Self;
    fn div(self,other : Expression) -> Self::Output {
        match (self,other) {
            (Expression::Term(t1), Expression::Term(t2)) => t1 / t2,
            (expr @_, Expression::Term(term)) => expr / term,
            (mut top @_,mut bottom @_)=> {
                let top_variables = top.get_unique_variables();
                let bottom_variables = bottom.get_unique_variables();
        
                let common_variables : BTreeSet<_> = top_variables.intersection(&bottom_variables).collect();
        
                if common_variables.is_empty() {
                    return Expression::new_durch(top, bottom)
                };

                let mut min_exponents = HashMap::new();
                let mut coefficients = HashSet::new();
                
                let sclone = top.clone();
                let oclone = bottom.clone();
        
                sclone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);
                oclone.get_min_exponents_and_coefficient(&common_variables, &mut min_exponents, &mut coefficients);        

                // Figure out gcd to divide coefficients by
                let gcd_coefficient = calculate_gcd(coefficients);

                top.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient.clone());
                bottom.cancel_variables_and_divide_coefficient(&min_exponents,gcd_coefficient);

                Expression::new_durch(top,bottom)
            }
        }
    }
}

pub(crate) fn gcd(a : Number,b : Number) -> Number {
    match b == 0 {
        true => a,
        false => gcd(b.clone(),a % b)
    }
}

fn calculate_gcd(mut coefficients: HashSet<Number>) -> Number {
    let _first_entry = coefficients.iter().next().cloned().unwrap();// equal to 'first' entry
    let mut ans = coefficients.take(&_first_entry).unwrap(); 

    for item in coefficients.into_iter() {
        ans = gcd(item,ans);

        if ans.is_one() {
            ans.set_one();
            break;
        }
    }

    ans
}

impl Term {
    /// Recursively calculates the minimum variable exponent 
    pub(super) fn get_min_exponents<'a>(&'a self,common_variables : &BTreeSet<&&char>,min_exponents : &mut HashMap<&'a char,&'a Number>) {
        self.variables.keys()
            .filter(|key| common_variables.contains(key))
            .for_each(|key|{
                let possible_value = self.variables.get(key).unwrap();
                min_exponents.entry(key)
                    .and_modify(|prev| *prev = (*prev).min(possible_value))
                    .or_insert(possible_value);
            });
    }

    /// Cancels common variables
    ///
    /// So 2x^2 / 2x = 2x / 2 with this methods
    pub(super) fn cancel_variables(&mut self,min_exponents : &HashMap<&char, &Number>) {
        for (key,min_exponent) in min_exponents {
            match min_exponent.cmp(&self.variables.get(key).unwrap()) {
                Ordering::Less => *self.variables.get_mut(key).unwrap() -= (*min_exponent).clone(),
                Ordering::Equal  | Ordering::Greater => {
                    self.variables.remove(key);
                }
            }
        }
    }

    /// Recursively calculates the minimum exponent values for common variables between two expression trees.
    fn get_min_exponents_and_coefficient<'a>(&'a self,common_variables : &BTreeSet<&&char>,min_exponents : &mut HashMap<&'a char, &'a Number>,coefficients :&mut HashSet<Number>) {
        if !coefficients.contains(&self.coefficient) {
            coefficients.insert(self.coefficient.clone());
        }

        self.get_min_exponents(common_variables,min_exponents);
    }

    /// Cancels common variables and divides the coefficient by the greatest common divisor (GCD).
    ///
    /// So 2x^2 / 2x = x with this method
    fn cancel_variables_and_divide_coefficient(&mut self,min_exponents : &HashMap<&char, &Number>,gcd_coefficient : Number) {
        self.coefficient /= gcd_coefficient;
        self.cancel_variables(min_exponents);
    }
}

impl Expression {
    /// Recursively calculates the minimum exponent values and coefficients for common variables
    /// within the expression.
    fn get_min_exponents_and_coefficient<'a>(
        &'a self,
        common_variables : &BTreeSet<&&char>,
        min_exponents : &mut HashMap<&'a char, &'a Number>,
        coefficients :&mut HashSet<Number>
    ) {
        match self {
            Expression::Term(term) => term.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients),
            Expression::Binary { left, right, .. } => {
                left.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients);
                right.get_min_exponents_and_coefficient(common_variables, min_exponents, coefficients);
            },
            Expression::Function { .. } => (),
        }
    }

    /// Recursively cancels common variables and divides coefficients within the expression.
    ///
    /// This method cancels out common variables based on their minimum exponents and divides all
    /// coefficients within the expression by a common divisor (GCD - Greatest Common Divisor).
    fn cancel_variables_and_divide_coefficient(&mut self,min_exponents : &HashMap<&char, &Number>,gcd_coefficient : Number) {
        match self {
            Expression::Term(ref mut term) => term.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient),
            Expression::Binary { left, right, .. } => {
                left.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient.clone());
                right.cancel_variables_and_divide_coefficient(min_exponents, gcd_coefficient);
            },
            Expression::Function { .. } => (),
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
                et1,
                et2
            );
    
            assert_eq!(result, expected);
        }
}

#[cfg(test)]
mod expression_tests {
    use super::*;

    fn from_str(input :&str) -> Expression {
        Expression::try_from((input,&Default::default())).unwrap()
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
        assert_eq!(result,Expression::new_durch(expr, num));
    }
}