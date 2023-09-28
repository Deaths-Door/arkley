use std::collections::BTreeMap;
use num_notation::Number;

/// Represents a collection of variables, each associated with a numerical value.
/// The `Variables` type is an alias for `BTreeMap<char, Number>`.
pub type Variables = BTreeMap<char,Number>;

/// A struct representing a mathematical term.
///
/// A `Term` is a basic unit in a mathematical expression. It consists of a coefficient and variables represented
/// as `BTreeMap<char,Number>` .
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Term {
    /// The coefficient of the term.
    pub(crate) coefficient: Number,

    /// The variables and their exponents in the term.
    pub(crate) variables: Variables,
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

    /// Method to create a Term with a single variable.
    pub fn create_single_variable_term(coeff: Number, var: char, exp: Number) -> Self {
        let variables = Variables::from([(var,exp)]);
        Term::new_with_variable(coeff, variables)
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
        let variables: Variables = [('x',Number::Decimal(2.0)), ('y', Number::Decimal(3.0))].iter().cloned().collect();
        let term = Term::new_with_variable(Number::Decimal(2.5),variables);
        assert_eq!(term.to_string(), "2.5x^2y^3");
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
}