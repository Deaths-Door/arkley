use crate::{Term, Variables, Expression};


impl Term {
    /// Count the occurrences of variables in the term.
    ///
    /// This method counts the occurrences of variables in the term.
    ///
    /// # Parameters
    ///
    /// - `variables_to_count`: A reference to a set of variables to count occurrences for.
    ///
    /// # Returns
    ///
    /// Returns the count of variable occurrences in the term.
    fn count_variable_occurrences(&self, variables_to_count: &Variables) -> usize {
        self.variables
            .keys()
            .filter(|key| variables_to_count.contains_key(key))
            .count()
    }
}

impl Expression {
    /// Count the occurrences of variables in the expression.
    ///
    /// This method counts the occurrences of variables in the expression and its nested sub-expressions.
    ///
    /// # Parameters
    ///
    /// - `variables_to_count`: A reference to a set of variables to count occurrences for.
    ///
    /// # Returns
    ///
    /// Returns the count of variable occurrences in the expression.
    pub(super) fn count_variable_occurrences(&self, variables_to_count: &Variables) -> usize {
        match self {
            Expression::Term(term) => term.count_variable_occurrences(variables_to_count),
            Expression::Binary { left, right, .. } => left.count_variable_occurrences(variables_to_count) + right.count_variable_occurrences(variables_to_count),
            Expression::Nested(inner) => inner.count_variable_occurrences(variables_to_count),
        }
    }
}