use std::ops::{Add,Sub,Mul,Div,Neg};
use std::collections::BTreeSet;
use std::cmp::Ordering;

use num_notation::Number;

use crate::{Term,Expression,Variables};

impl Term {
    pub(in crate::arithmetics) fn is_combinable_with(&self,other : &Self) -> bool {
        self.variables == other.variables
    }
}

impl Neg for Term {
    type Output = Self;

    fn neg(self) -> Self {
        Term::new_with_variable(-self.coefficient,self.variables)
    }
}

impl Term {
    pub(crate) fn force_add_terms(self,other : Term) -> Self {
        let coefficient = self.coefficient + other.coefficient;
        let variables = self.variables;
        Term::new_with_variable(coefficient,variables)
    }
}


impl Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        if self.is_combinable_with(&other) {
            return self.force_add_terms(other).into();
        }

        Expression::new_plus(self.into(),other.into())
    }
}

impl Sub for Term {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        if self.is_combinable_with(&other) {
            let coefficient = self.coefficient - other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new_with_variable(coefficient,variables));
        }
        Expression::new_minus(self.into(),other.into())
    }
}

impl Mul for Term {
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

impl Div for Term {
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

macro_rules! primitives_operations {
    (padd => $($t : ty),*) => {
        $(
            impl Add<$t> for Term {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self + term
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

                    self - term
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

                    self * term
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

                    self / term
                }
            }
        )*
    };

    (vadd => $($t : ty),*) => {
        $(
            impl Add<$t> for Term {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self + term
                }
            }
        )*
    };

    (vsub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Term {
                type Output = Expression;
                fn sub(self, n: $t) -> Expression {
                    let term = Term::from(n);

                    self - term
                }
            }
        )*
    };

    (vmul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Term {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self * term
                }
            }
        )*
    };

    (vdiv => $($t : ty),*) => {
        $(
            impl Div<$t> for Term {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self / term
                }
            }
        )*
    };

    (pops => $($t:ty),*) => {
        $(
            primitives_operations!(padd => $t);
            primitives_operations!(psub => $t);
            primitives_operations!(pmul => $t);
            primitives_operations!(pdiv => $t);
        )*
    };

    (nvops => $($t : ty),*) => {
        $(
            primitives_operations!(vadd => $t);
            primitives_operations!(vsub => $t);
            primitives_operations!(vmul => $t);
            primitives_operations!(vdiv => $t);
        )*
    }
}

primitives_operations!(pops => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);
primitives_operations!(nvops => Number,Variables);

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
}