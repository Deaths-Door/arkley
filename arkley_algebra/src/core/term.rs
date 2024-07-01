use std::{collections::HashMap, hash::Hash, ops::Deref};
use itertools::Itertools;
use num_notation::{Number, One, Zero};

use crate::{Variable, Variables};

/// A struct representing a mathematical term.
///
/// A `Term` is a basic unit in a mathematical expression. It consists of a coefficient and variables represented
/// as [Variables]
#[derive(Clone, PartialEq)]
pub struct Term {
    /// The coefficient of the term.
    pub(crate) coefficient: Number,

    /// The variables and their exponents in the term.
    pub(crate) variables: Variables,
}

impl Term {
    /// Creates new instance of Term using coefficient
    pub fn new(coefficient: impl Into<Number>) -> Self {
        Self { coefficient : coefficient.into() , variables : Variables::new() }
    }
    
    /// Creates new instance of Term using coefficient and variable
    pub fn new_with_variable(coefficient: impl Into<Number>,variable: impl Into<Variable>) -> Self {
        Self::new_with_variable_to(coefficient,variable,Number::one())
    }

    /// Creates new instance of Term using coefficient and variables
    pub fn new_with_variables(coefficient: impl Into<Number>,variables: Variables) -> Self {
        Self { coefficient : coefficient.into() , variables }
    }

    /// Creates new instance of Term only variable
    pub fn new_with_only_variable(variable: impl Into<Variable>) -> Self {
        Self::new_with_variable(Number::one(), variable)
    }

    /// Creates new instance of Term only variables
    pub fn new_with_only_variables(variables: Variables) -> Self {
        Self::new_with_variables(Number::one(), variables)
    }

    /// Creates new instance of Term only the variable and its exponent
    pub fn new_with_only_variable_to(variable: impl Into<Variable>,exponent : impl Into<Number>) -> Self {
        Self::new_with_variable_to(Number::one(), variable,exponent)
    }

    /// Creates new instance of Term using coefficient, variable and its exponent
    pub fn new_with_variable_to(coefficient: impl Into<Number>,variable: impl Into<Variable>,exponent : impl Into<Number>) -> Self {
        let variables = Variables::from([(variable.into(),exponent.into())]);
        Self::new_with_variables(coefficient, variables)
    }

    /// Checks if the polynomial is numeric, meaning it has no variables and
    /// its coefficient is equal to the given number.
    ///
    /// # Arguments
    ///
    /// * `number` - The number to compare the polynomial's coefficient with.
    ///
    /// # Returns
    ///
    /// `true` if the polynomial is numeric, `false` otherwise.
    pub fn is_numeric(&self,number : impl Into<Number>) -> bool {
        self.variables.is_empty() && self.coefficient == number.into()
    }

    /// Checks if the polynomial is the numeric constant one (coefficient of 1
    /// and no variables).
    ///
    /// This is a shorthand for `self.is_numeric(Number::one())`.
    ///
    /// # Returns
    ///
    /// `true` if the polynomial is the numeric constant one, `false` otherwise.
    pub fn is_numeric_one(&self) -> bool {
        self.is_numeric(Number::one())
    }

    /// Checks if the polynomial is the numeric constant zero (coefficient of 0
    /// and no variables).
    ///
    /// This is a shorthand for `self.is_numeric(Number::zero())`.
    ///
    /// # Returns
    ///
    /// `true` if the polynomial is the numeric constant zero, `false` otherwise.
    pub fn is_numeric_zero(&self) -> bool {
        self.is_numeric(Number::zero())
    }
}

impl From<Number> for Term {
    fn from(coefficient: Number) -> Self {
        Term::new(coefficient)
    }
}

impl<T> From<(Number,T)> for Term  where T : Into<Variable> {
    fn from((coefficient,variable): (Number,T)) -> Self {
        Term::new_with_variable(coefficient, variable)
    }
}

impl From<(Number,Variables)> for Term {
    fn from((value,variables): (Number,Variables)) -> Self {
        Term::new_with_variables(value, variables)
    }
}

impl<T> From<T> for Term  where T : Into<Variable> {
    fn from(variable: T) -> Self {
        Term::new_with_only_variable(variable)
    }
}

impl From<Variables> for Term {
    fn from(variables: Variables) -> Self {
        Term::new_with_only_variables(variables)
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.coefficient.is_one() {
            false => write!(f, "{}", self.coefficient)?,
            true => match self.variables.is_empty() {
                true => return write!(f, "{}", self.coefficient),
                false => {}
            }
        }

        // https://stackoverflow.com/a/70125221/20243803
        for (name,exponent) in self.variables.iter().sorted_by_key(|v| v.0) {
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

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // proptize the variables
        match self.variables.partial_cmp(&other.variables) {
            Some(core::cmp::Ordering::Equal) => self.coefficient.partial_cmp(&other.coefficient),
            ord => return ord,
        }
    }
}

macro_rules! primitives_others {
    (pfrom => $($t :ty),*) => {
        $(
            impl From<$t> for Term {
                fn from(value : $t) -> Self {
                    Term::new(Number::Decimal(value as f64))
                }
            }
        )*
    };
}

primitives_others!(pfrom => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64 );

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::*;
    #[test]
    fn display_term() {
        let variables: Variables = [
            ('x'.into(),Number::Decimal(2.0)), 
            ('y'.into(), Number::Decimal(3.0))]
            .iter()
            .cloned()
            .collect();
        let term = Term::new_with_variables(Number::Decimal(2.5),variables);
        assert_eq!(term.to_string(), "2.5x^2y^3");
    }

    #[test]
    fn display_term_single_variable() {
        let variables: Variables = [('x'.into(), Number::one())].iter().cloned().collect();
        let term = Term::new_with_variables(Number::Decimal(3.0),variables);
        assert_eq!(term.to_string(), "3x");
    }

    #[test]
    fn display_term_constant() {
        let variables: Variables = Variables::new();
        let term = Term::new_with_variables(Number::Decimal(5.0),variables);
        assert_eq!(term.to_string(), "5");
    }

    use test_case::test_case;

    #[test_case(1,&[('x',2),('y',2),('z',1)])]
    fn partial_eq(coeffiecent : i32,variables : &[(char,i32)]) {
        let expected_variables = Variables::from_iter(
            variables.into_iter().map(|(l,e)| (Variable::from(*l),Number::from((*e) as f64)))
        );

        assert_eq!(
            Term::new_with_variables(coeffiecent as f64, expected_variables.clone()),
            Term::new_with_variables(coeffiecent as f64, expected_variables)
        );
    }

    #[test_case("2x","x^2",Ordering::Less)]
    //#[test_case()]
    fn ord(term : &str,variables : &str,expect : impl Into<Option<Ordering>>) { 
        assert_eq!(
            Term::try_from(term).unwrap().partial_cmp(&(Variables::try_from(variables).unwrap().into())),
            expect.into()
        )
    }
}