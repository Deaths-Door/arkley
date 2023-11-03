use crate::Expression;
use crate::manipulation::VariableAnalysis;

impl Expression {
    /// Check if the polynomial equation is quadratic (degree 2).
    /// **Note** : `None` is returned if given variable is not in tree
    fn is_quadratic(&self,variable : &char) -> Option<bool> {
        self.degree(variable).map(|v| v == 2)
    }

    /// Check if the polynomial equation is cubic (degree 3).
    /// **Note** : `None` is returned if given variable is not in tree
    fn is_cubic(&self,variable : &char) -> Option<bool> {
        self.degree(variable).map(|v| v == 3)
    }

    /// Check if the polynomial equation is raised to the power of `x`.
    /// **Note** : `None` is returned if given variable is not in tree
    fn is_degree(&self,variable : &char, x: u32) -> Option<bool> {
        self.degree(variable).map(|v| v == x)
    }

    /// Get the degree of the polynomial equation.
    /// **Note** : `None` is returned if given variable is not in tree
    fn degree(&self,variable : &char) -> Option<u32> {
        match !self.contains_variable(variable) {
            true => None,
            false => self.get_max_exponent_for(variable).into(),
        }
    }
}

impl Expression {
    fn get_max_exponent_for(&self,variable : &char) -> u32 {
        match self {
            Self::Term(term) => term.variables.get(variable).map_or(0, |_| 1),
            Self::Binary { left , right , .. } => left.get_max_exponent_for(variable) + right.get_max_exponent_for(variable),
            Self::Nested(inner) => inner.get_max_exponent_for(variable),
            _ => 0
        }
    }
}