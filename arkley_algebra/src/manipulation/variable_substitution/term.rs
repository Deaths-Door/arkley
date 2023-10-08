use num_notation::{Number, Pow};

use crate::{Term, Variables};

use super::VariableSubstitution;

impl VariableSubstitution for Term {
    fn try_replace_single_variable_with_value(&mut self,variable : &char,value : Number) -> Option<()> {
        self.variables.remove(variable).and_then(|exponent| {
            self.coefficient = self.coefficient.clone() * value.pow(exponent);
            Some(())
        })
    }

    fn try_replace_variables_with_value(&mut self,variable_values : &mut Variables) {
        let mut to_remove = Vec::new();

        for (key,exponent) in &self.variables {
            match variable_values.remove(&key) {
                Some(value) => {
                    // n * (value ^ exponent)
                    self.coefficient *= value.pow(exponent.clone());

                    to_remove.push(key.clone());
                }
                _ => ()
            }
        }

        for key in to_remove {
            self.variables.remove(&key);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    

    #[test]
    fn try_replace_single_variable_success() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0))]));
        let result = term.try_replace_single_variable_with_value(&'x', Number::Decimal(4.0));
        assert_eq!(result, Some(()));
        // Check that 'x' variable was replaced with 4.0
        assert_eq!(term.variables.get(&'x'), None);
    }

    #[test]
    fn try_replace_single_variable_failure() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let result = term.try_replace_single_variable_with_value(&'x', Number::Decimal(4.0));
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
        term.try_replace_variables_with_value(&mut variable_values);
        // Check that 'x' variable was replaced with 5.0 and 'z' remains unchanged
        assert_eq!(term.variables.get(&'x'), None);
        assert_eq!(variable_values.get(&'z'), Some(&Number::Decimal(6.0)));
    }

    #[test]
    fn try_replace_variables_failure() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let mut variable_values = BTreeMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        term.try_replace_variables_with_value(&mut variable_values);
        // Check that 'x' variable was not found, so the term remains unchanged
        assert_eq!(term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
    }

    #[test]
    fn test_try_replace_variables_with_value() {
        // Create a sample Term with variables and values
        // 2x^3y^2
        // 2 * (2 ^3)
        let mut term = Term::new_with_variable(2.0.into(), Variables::from([('x',3.0.into()),('y',2.0.into())]));
        
        // Create a sample variable_values map
        let mut variable_values = BTreeMap::new();
        variable_values.insert('x', Number::Decimal(2.0));
        variable_values.insert('z', Number::Decimal(4.0));

        // Call try_replace_variables_with_value on the term
        term.try_replace_variables_with_value(&mut variable_values);

        // Check if 'x' was replaced with 2.0 and 'y' was not present
        assert_eq!(term.variables.get(&'x'), None);
        assert_eq!(term.variables.get(&'y'), Some(&Number::Decimal(2.0)));

        // Check if the coefficient was updated correctly 2 * (2 ^3) = 16
        assert_eq!(term.coefficient, Number::Decimal(16.0));

        // Check if 'z' was not affected in the variable_values map
        assert_eq!(variable_values.get(&'z'), Some(&Number::Decimal(4.0)));
    }
}
