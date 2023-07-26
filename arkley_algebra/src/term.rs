use std::ops::{Add,Sub,Mul,Div};
use std::collections::{BTreeMap,BTreeSet};

use arkley_numerics::Number;

use crate::Expression;

/// Represents a collection of variables, each associated with a numerical value.
/// The `Variables` type is an alias for `BTreeMap<char, Number>`.
pub type Variables = BTreeMap<char,Number>;

/// A struct representing a mathematical term.
///
/// A `Term` is a basic unit in a mathematical expression. It consists of a coefficient
/// (which can be any type that implements the `Numeric` trait) and variables represented
/// as `BTreeMap<char,Number>` .
pub struct Term {
    /// The coefficient of the term.
    coefficient: Number,

    /// The variables and their exponents in the term.
    variables: Variables,
}

impl Term {
    /// Creates new instance of Term using coefficient and variable
    pub const fn new(coefficient: Number,variables: Variables) -> Self {
        Self { coefficient , variables }
    }
}

impl Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        if self.variables == other.variables {
            let coefficient = self.coefficient + other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new(coefficient,variables));
        }
        Expression::new_plus(self.into(),other.into())
    }
}

impl Sub for Term {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        if self.variables == other.variables {
            let coefficient = self.coefficient - other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new(coefficient,variables));
        }
        Expression::new_minus(self.into(),other.into())
    }
}

impl Mul for Term {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        let coefficient = self.coefficient * other.coefficient;
        let mut variables = self.variables;
        for (&var,&exponent) in &other.variables {
            *variables.entry(var).or_insert(Number::Decimal(0.0)) += exponent;
        };

        Expression::new_term(Term::new(coefficient,variables))
    }
}

impl Div for Term {
    type Output = Expression;

    fn div(self,other : Term) -> Self::Output {
        let coefficient = self.coefficient / other.coefficient;

        let s_keys: BTreeSet<_> = self.variables.keys().cloned().collect();
        let o_keys: BTreeSet<_> = other.variables.keys().cloned().collect();

        let common : BTreeSet<_> = s_keys.intersection(&o_keys).cloned().collect();
        let s_unique_keys: BTreeSet<_> = s_keys.difference(&common).collect();
        let o_unique_keys: BTreeSet<_> = o_keys.difference(&common).collect();

        todo!("Not sure how to implement this to handle all cases of term variable etc")
       /* //common key .values for s and o - each other;
        let subtract_exponents = || -> Variables {
            let mut variables = Variables::new();
            for key in &common {
                let s_exponent = *self.variables.get(&key).unwrap();
                let o_exponent = *other.variables.get(&key).unwrap();
                variables.insert(*key,s_exponent - o_exponent);
            }
            variables
        };
        match s_unique_keys.is_empty() && o_unique_keys.is_empty() {
            // it means only common variables are there so eg 5x / 8x 
            true => Expression::new_term(Term::new(coefficient,subtract_exponents())),
            false => todo!("")
        }*/
    }
}