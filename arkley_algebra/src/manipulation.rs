use std::collections::BTreeSet;
use num_notation::Number;

use crate::{Expression,Term,Variables};

impl Term {
    /// Checks if a variable is present in the term.
    ///
    /// # Arguments
    ///
    /// * `variable`: The variable (char) to check for in the term.
    ///
    /// # Returns
    ///
    /// `true` if the variable is present in the term, `false` otherwise.
    pub fn contains_variable(&self, variable: &char) -> bool {
        self.variables.contains_key(variable)
    }

    /// Attempts to replace a single variable in the term with a specified value.
    ///
    /// # Arguments
    ///
    /// - `variable`: A reference to the variable (char) to be replaced in the term.
    /// - `value`: The value (Number) to replace the variable with.
    ///
    /// # Returns
    ///
    /// An `Option<()>` where:
    /// - `Some(())` indicates the variable was found and successfully replaced.
    /// - `None` indicates the variable did not exist in the term, and no replacement occurred.
    pub fn try_replace_single_variable(&mut self,variable : &char,_value : Number) -> Option<()> {
        self.variables.remove(variable).and_then(|_exponent| todo!("Implement `Pow` for Number so self.coeffiecent * (value ^ exponent)"))
    }

    /// Attempts to replace multiple variables in the term with specified values.
    ///
    /// # Arguments
    ///
    /// - `variable_values`: A reference to a `Variables` map containing variables and their values.
    ///
    /// # Returns
    ///
    /// The updated term with the specified variables replaced. Variables that do not exist in the term
    /// are left unchanged in the `variable_values` map given.
    pub fn try_replace_variables(&mut self,variable_values : &mut Variables) {
        let keys_to_remove: Vec<_> = variable_values.keys()
            .filter(|var| self.variables.contains_key(var))
            .collect();

        for var in &keys_to_remove {
            let _exponent = self.variables.remove(var).unwrap();
            todo!("Implement `Pow` for Number so self.coeffiecent * (value ^ exponent)");
        }
    }
}

impl Expression {
    /// Extracts unique variables from the expression.
    ///
    /// This function recursively traverses the expression and collects all unique variables
    /// found within it. The result is returned as a `BTreeSet<char>`, where each character
    /// represents a unique variable.
    ///
    /// # Returns
    ///
    /// A `BTreeSet<char>` containing the unique variables present in the expression.
    pub fn get_unique_variables(&self) -> BTreeSet<char> {
        let mut unique_variables = BTreeSet::new();

        // Helper function to recursively traverse the Expression.
        fn extract_variables(expr: &Expression, unique_vars: &mut BTreeSet<char>) {
            match expr {
                Expression::Term(term) => {
                    unique_vars.extend(term.variables.keys().cloned());
                },
                Expression::Binary { left, right, .. } => {
                    extract_variables(left, unique_vars);
                    extract_variables(right, unique_vars);
                },
                Expression::Nested(inner) => {
                    extract_variables(inner, unique_vars);
                },
            }
        }

        extract_variables(self, &mut unique_variables);
        unique_variables
    }

    /// Checks if a variable is present in the expression.
    ///
    /// # Arguments
    ///
    /// * `variable`: The variable (char) to check for in the expression.
    ///
    /// # Returns
    ///
    /// `true` if the variable is present in the expression, `false` otherwise.
    pub fn contains_variable(&self, variable: &char) -> bool {
        // Helper function to recursively check for the variable.
        fn check_variable(expr: &Expression, target: &char) -> bool {
            match expr {
                Expression::Term(term) => term.variables.contains_key(target),
                Expression::Binary { left, right, .. } => check_variable(left, target) || check_variable(right, target),
                Expression::Nested(inner) => check_variable(inner, target),
            }
        }

        check_variable(self, variable)
    }

    /// Attempts to replace a single variable in the expression with a specified value.
    ///
    /// # Arguments
    ///
    /// - `variable`: A reference to the variable (char) to be replaced in the term.
    /// - `value`: The value (Number) to replace the variable with.
    ///
    /// # Returns
    ///
    /// An `Option<()>` where:
    /// - `Some(())` indicates the variable was found and successfully replaced.
    /// - `None` indicates the variable did not exist in the term, and no replacement occurred.
    pub fn try_replace_single_variable(&mut self,variable : &char,value : Number) -> Option<()> {
        match self {
            Expression::Term(term) => term.try_replace_single_variable(variable,value),
            Expression::Nested(inner) => inner.try_replace_single_variable(variable,value),
            Expression::Binary { left , right , ..} => {
                let left_result = left.try_replace_single_variable(variable, value.clone());
                let right_result = right.try_replace_single_variable(variable, value);

                match (left_result,right_result) {
                    (None,None) => None,
                    _ => Some(()),
                }
            }
        }
    }

    /// Attempts to replace multiple variables in the expression with specified values.
    ///
    /// # Arguments
    ///
    /// - `variable_values`: A reference to a `Variables` map containing variables and their values.
    ///
    /// # Returns
    ///
    /// The updated term with the specified variables replaced. Variables that do not exist in the term
    /// are left unchanged in the `variable_values` map given.
    pub fn try_replace_variables(&mut self,variable_values : &mut Variables) {
        match self {
            Expression::Term(term) => term.try_replace_variables(variable_values),
            Expression::Nested(inner) => inner.try_replace_variables(variable_values),
            Expression::Binary { left , right , ..} => {
                left.try_replace_variables(variable_values);
                right.try_replace_variables(variable_values);
            }
        }
    }
}


#[cfg(test)]
mod term {
    use super::*;
    use std::collections::BTreeMap;
    

    #[test]
    fn try_replace_single_variable_success() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0))]));
        let result = term.try_replace_single_variable(&'x', Number::Decimal(4.0));
        assert_eq!(result, Some(()));
        // Check that 'x' variable was replaced with 4.0
        assert_eq!(term.variables.get(&'x'), None);
    }

    #[test]
    fn try_replace_single_variable_failure() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let result = term.try_replace_single_variable(&'x', Number::Decimal(4.0));
        assert_eq!(result, None);
        // Check that 'x' variable was not found, so the term remains unchanged
        assert_eq!(term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
    }

    #[test]
    fn try_replace_variables_success() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0)),('y',Number::Decimal(4.0))]));
        let mut variable_values = BTreeMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        variable_values.insert('z', Number::Decimal(6.0));
        term.try_replace_variables(&mut variable_values);
        // Check that 'x' variable was replaced with 5.0 and 'z' remains unchanged
        assert_eq!(term.variables.get(&'x'), None);
        assert_eq!(variable_values.get(&'z'), Some(&Number::Decimal(6.0)));
    }

    #[test]
    fn try_replace_variables_failure() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let mut variable_values = BTreeMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        term.try_replace_variables(&mut variable_values);
        // Check that 'x' variable was not found, so the term remains unchanged
        assert_eq!(term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
    }
}

#[cfg(test)]
mod expression {
    use super::*;
    use num_notation::Number;
    use std::collections::BTreeMap;

    #[test]
    fn try_replace_single_variable_success() {
        let term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0))]));
        let mut expression = Expression::Term(term.clone());
        let result = expression.try_replace_single_variable(&'x', Number::Decimal(4.0));
        assert_eq!(result, Some(()));
        // Check that 'x' variable was replaced with 4.0
        if let Expression::Term(new_term) = expression {
            assert_eq!(new_term.variables.get(&'x'), None);
        } else {
            panic!("Expected Expression::Term after replacement.");
        }
    }

    #[test]
    fn try_replace_single_variable_failure() {
        let term = Term::new_with_variable(Number::Decimal(2.0), Variables::from([('y',Number::Decimal(3.0))]));
        let mut expression = Expression::Term(term.clone());
        let result = expression.try_replace_single_variable(&'x', Number::Decimal(4.0));
        assert_eq!(result, None);
        // Check that 'x' variable was not found, so the term remains unchanged
        if let Expression::Term(new_term) = expression {
            assert_eq!(new_term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
        } else {
            panic!("Expected Expression::Term after replacement.");
        }
    }

    #[test]
    fn try_replace_variables_success() {
        let term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0)),('y',Number::Decimal(4.0))]));
        let expression = Expression::Term(term.clone());

        let mut variable_values = BTreeMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        variable_values.insert('z', Number::Decimal(6.0));

        let mut result = expression.clone();
        result.try_replace_variables(&mut variable_values);

        // Check that 'x' variable was replaced with 5.0 and 'z' remains unchanged
        if let Expression::Term(new_term) = result {
            assert_eq!(new_term.variables.get(&'x'), None);
        } else {
            panic!("Expected Expression::Term after replacement.");
        }

        // Check that variable_values still contains 'z'
        assert_eq!(variable_values.get(&'z'), Some(&Number::Decimal(6.0)));
    }

    #[test]
    fn try_replace_variables_failure() {
        let term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let expression = Expression::Term(term.clone());

        let mut variable_values = BTreeMap::new();
        variable_values.insert('x', Number::Decimal(5.0));

        let mut result = expression.clone();
        result.try_replace_variables(&mut variable_values);

        // Check that 'x' variable was not found, so the term remains unchanged
        if let Expression::Term(new_term) = result {
            assert_eq!(new_term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
        } else {
            panic!("Expected Expression::Term after replacement.");
        }
    }
}
