use std::ops::{Add,Sub,Mul,Div};
use std::collections::{BTreeMap,BTreeSet};
use std::cmp::Ordering;

use num_notation::Number;

use crate::Expression;

/// Represents a collection of variables, each associated with a numerical value.
/// The `Variables` type is an alias for `BTreeMap<char, Number>`.
pub type Variables = BTreeMap<char,Number>;

/// A struct representing a mathematical term.
///
/// A `Term` is a basic unit in a mathematical expression. It consists of a coefficient
/// (which can be any type that implements the `Numeric` trait) and variables represented
/// as `BTreeMap<char,Number>` .
#[derive(PartialEq,Clone)]
pub struct Term {
    /// The coefficient of the term.
    coefficient: Number,

    /// The variables and their exponents in the term.
    variables: Variables,
}

impl Term {
    /// Creates new instance of Term using coefficient and variable
    pub const fn new_with_variable(coefficient: Number,variables: Variables) -> Self {
        Self { coefficient , variables }
    }

    /// Creates new instance of Term using coefficient
    pub const fn new(coefficient: Number) -> Self {
        Self { coefficient , variables : Variables::new() }
    }
}

impl From<Number> for Term {
    fn from(value : Number) -> Self {
        Term::new(value)
    }
}

impl From<Variables> for Term {
    fn from(value : Variables) -> Self {
        Term::new_with_variable(Number::Decimal(1.0),value)
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.coefficient == 1 {
            false => write!(f, "{}", self.coefficient)?,
            true => match self.variables.is_empty() {
                true => write!(f, "{}", self.coefficient)?,
                false => {}
            }
        }
        
        for (name,exponent) in self.variables.iter() {
            write!(f,"{name}")?;
            if exponent > &1 {
                write!(f,"^{exponent}")?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self}")
    }
}

mod operations {
    use super::*;

    pub(super) fn add(t1 : Term,t2 : Term) -> Expression {
        if t1.variables == t2.variables {
            let coefficient = t1.coefficient + t2.coefficient;
            let variables = t1.variables;
            return Expression::new_term(Term::new_with_variable(coefficient,variables));
        }
        Expression::new_plus(t1.into(),t2.into())
    }

    pub(super) fn sub(t1 : Term,t2 : Term) -> Expression {
        if t1.variables == t2.variables {
            let coefficient = t1.coefficient - t2.coefficient;
            let variables = t1.variables;
            return Expression::new_term(Term::new_with_variable(coefficient,variables));
        }
        Expression::new_minus(t1.into(),t2.into())
    }

    pub(super) fn mul(t1 : Term,t2 : Term) -> Expression {
        let coefficient = t1.coefficient * t2.coefficient;
        let mut variables = t1.variables;
        for (&var,&ref exponent) in &t2.variables {
            *variables.entry(var).or_insert(Number::Decimal(0.0)) += exponent.clone();
        };

        Expression::new_term(Term::new_with_variable(coefficient,variables))
    }

    pub(super) fn div(t1 : Term,t2 : Term) -> Expression {
        let s_keys: BTreeSet<_> = t1.variables.keys().cloned().collect();
        let o_keys: BTreeSet<_> = t2.variables.keys().cloned().collect();

        let common : BTreeSet<_> = s_keys.intersection(&o_keys).cloned().collect();
        let s_unique_keys: BTreeSet<_> = s_keys.difference(&common).collect();
        let o_unique_keys: BTreeSet<_> = o_keys.difference(&common).collect();

        let mut s_variables = Variables::new();
        let mut o_variables =Variables::new();

        for key in s_unique_keys {
            s_variables.insert(*key,t1.variables[key].clone());
        }

        for key in o_unique_keys {
            o_variables.insert(*key,t2.variables[key].clone());
        }

        for key in common {
            let s_exponent = t1.variables[&key].clone();
            let o_exponent = t2.variables[&key].clone();

            let result = s_exponent - o_exponent;

            match result.partial_cmp(&0).unwrap() {
                // if equal so x^2 - x^2 then just ignore it
                Ordering::Equal => {}
                // if top > bottom so x^5 - x^3 = x^2 so top.variable(key).power = result
                Ordering::Greater => {
                    s_variables.insert(key,result);
                },
                // if top < bottom so x^2 - x^5 = x^-3 so bottom.variable(key).power = result
                Ordering::Less => {
                    o_variables.insert(key,result);
                },
            };
        }

        match s_variables.is_empty() && o_variables.is_empty() {
            true => Expression::new_term(Term::new(t1.coefficient / t2.coefficient)),
            false => {
                let _t1 = Term::new_with_variable(t1.coefficient,s_variables);
                // so if x / 1 is equal to 1 so simplify it
                match t2.coefficient == 1 && o_variables.is_empty() {
                    true => Expression::new_term(_t1),
                    false => {
                        let _t2 = Term::new_with_variable(t2.coefficient,o_variables);
                        Expression::new_durch(_t1.into(),_t2.into())
                    }
                }
            }
        }
    }
}

impl Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        operations::add(self,other)
    }
}

impl Sub for Term {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        operations::sub(self,other)
    }
}

impl Mul for Term {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        operations::mul(self,other)
    }
}

impl Div for Term {
    type Output = Expression;

    fn div(self,other : Term) -> Self::Output {
        operations::div(self,other)
    }
}

macro_rules! primitives {
    (padd => $($t : ty),*) => {
        $(
            impl Add<$t> for Term {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    operations::add(self,term)
                }
            }
        )*
    };

    (psub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Term {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    operations::sub(self,term)
                }
            }
        )*
    };

    (pmul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Term {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    operations::mul(self,term)
                }
            }
        )*
    };

    (pdiv => $($t : ty),*) => {
        $(
            impl Div<$t> for Term {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    operations::div(self,term)
                }
            }
        )*
    };
    (operations => $($t:ty),*) => {
        $(
            primitives!(padd => $t);
            primitives!(psub => $t);
            primitives!(pmul => $t);
            primitives!(pdiv => $t);
        )*
    }
}

primitives!(operations => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_terms_with_same_variables() {
        // 2.5x
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));
        //3.5x
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(1.0))]));

        // 2.5x + 3.5x = 6x
        let result = term1.clone() + term2.clone();

        // 6x
        let expected_term = Term::new_with_variable(Number::Decimal(6.0), Variables::from([('x', Number::Decimal(1.0))]));
        
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn add_terms_with_different_variables() {
        //2.5x
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));

        //3.5y
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('y', Number::Decimal(1.0))]));

        // 2.5x + 3.5y
        let result = term1.clone() + term2.clone();

        // 2.5x + 3.5y
        let expected_expression = Expression::new_plus(term1.into(), term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn add_terms_with_same_variables_and_different_powers() {
        // 2.5x^2
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 3.5x^3
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2 + 3.5x^2
        let result = term1.clone() + term2.clone();

        // 2.5x^2 + 3.5x^2
        let expected_expression = Expression::new_plus(term1.into(),term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_same_variables() {
        // 5x
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(1.0))]));

        // 2.5x
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));

        // 5x - 2.5x
        let result = term1.clone() - term2.clone();


        // 2.5x
        let expected_term = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_different_variables() {

        // 5x
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(1.0))]));

        // 5y
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('y', Number::Decimal(1.0))]));

        // 5x - 5y
        let result = term1.clone() - term2.clone();

        // 5x - 5y
        let expected_expression = Expression::new_minus(term1.into(), term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_same_variables_and_different_powers() {
        // 5x^3
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 5x^3 - 2.5x^2
        let result = term1.clone() - term2.clone();

        let expected_expression = Expression::new_minus(term1.into(),term2.into());

        assert_eq!(result, expected_expression);
    }

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

    // Helper function to create a Term with a single variable.
     fn create_term_with_variable(coeff: i32, var: char, exp: i32) -> Term {
        let mut variables = Variables::new();
        variables.insert(var,Number::Decimal(exp as f64));
        Term::new_with_variable(Number::Decimal(coeff as f64), variables)
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

    #[test]
    fn display_term() {
        let variables: Variables = [('x',Number::Decimal(2.0)), ('y', Number::Decimal(3.0))].iter().cloned().collect();
        let term = Term::new_with_variable(Number::Decimal(2.5),variables);
        assert_eq!(term.to_string(), "2.5x^2y^3");
        println!("{}",term.to_string());
    }

    #[test]
    fn display_term_single_variable() {
        let variables: Variables = [('x', Number::Decimal(1.0))].iter().cloned().collect();
        let term = Term::new_with_variable(Number::Decimal(3.0),variables);
        assert_eq!(term.to_string(), "3x");
    }

    #[test]
    fn display_term_constant() {
        let variables: Variables = Variables::new();
        let term = Term::new_with_variable(Number::Decimal(5.0),variables);

        assert_eq!(term.to_string(), "5");
    }
/*
    #[test]
    fn try_set_variable_value_existing_variable() {
        // Create a term with the variable 'x' and exponent 2
        let mut term = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(2.0))]));

        // Set the value of variable 'x' to 7
        assert_eq!(term.try_set_variable_value(&'x', Number::Decimal(7.0)), Some(()));

        // Check that the coefficient is updated correctly
        assert_eq!(term.coefficient, Number::Decimal(5.0 * (7_i32.pow(2)) as f64));
    }

    #[test]
    fn try_set_variable_value_non_existing_variable() {
        // Create a term with the variable 'y' and exponent 3
        let mut term = Term::new_with_variable(Number::Decimal(3.0), Variables::from([('y', Number::Decimal(3.0))]));

        // Try to set the value of variable 'x' (non-existing variable) to 5
        assert_eq!(term.try_set_variable_value(&'x', Number::Decimal(5.0)), None);

        // Check that the coefficient remains unchanged
        assert_eq!(term.coefficient, Number::Decimal(3.0));
    }


    #[test]
    fn try_from_term_with_valid_input_no_variables() {
        let input = "123.45";
        let result = Term::try_from(input).unwrap();
        let expected = Term::new(Number::Decimal(123.45));
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_term_with_variables() {
        let input = "2x^3y^2z";
        let result = Term::try_from(input).unwrap();//.unwrap();
        
        let expected_variables: Variables = [('x', Number::Decimal(3.0)),
                                            ('y', Number::Decimal(2.0)),
                                            ('z', Number::Decimal(1.0))].iter().cloned().collect();

        let expected = Term::new_with_variable(Number::Decimal(2.0), expected_variables);
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_term_with_valid_input_no_exponents() {
        let input = "xyz";
        let result = Term::try_from(input);//.unwrap();
        assert!(result.is_ok());

        let ev = Variables::from(
            [
                ('x',Number::Decimal(1.0)),
                ('y',Number::Decimal(1.0)),
                ('z',Number::Decimal(1.0))
            ]
        );
        let expected = Term::new_with_variable(Number::Decimal(1.0), ev);
        assert_eq!(result.unwrap(), expected);
    }*/
}