use std::collections::HashMap;
use itertools::Itertools;
use num_notation::{Number, One};

use crate::Variable;

/// Represents a collection of variables, each associated with a numerical value.
/// The `Variables` type is an alias for [HashMap<Variable, Number>]
pub type Variables = HashMap<Variable,Number>;

/// A struct representing a mathematical term.
///
/// A `Term` is a basic unit in a mathematical expression. It consists of a coefficient and variables represented
/// as [Variables]
#[derive(Clone,PartialEq)]
pub struct Term {
    /// The coefficient of the term.
    pub(crate) coefficient: Number,

    /// The variables and their exponents in the term.
    pub(crate) variables: Variables,
}

impl Term {
    /// Creates new instance of Term using coefficient
    pub fn new(coefficient: Number) -> Self {
        Self { coefficient , variables : Variables::new() }
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
}