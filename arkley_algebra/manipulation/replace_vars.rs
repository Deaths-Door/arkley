impl ReplaceVariables for Term {
    fn try_replace_single_variable_with_value(&mut self,variable : &char,value : Number) -> Option<()> {
        self.variables.remove(variable).and_then(|exponent| {
            self.coefficient = self.coefficient.clone() * value.pow(exponent);
            Some(())
        })
    }

    fn try_replace_variables_with_value(&mut self,variable_values : &mut Variables) {
        let mut to_remove = Vec::new();

        for (key,value) in &mut self.variables {
            match variable_values.remove(&key) {
                Some(exponent) => {
                    self.coefficient = self.coefficient.clone() * value.clone().pow(exponent);
                    to_remove.push(key.clone());
                }
                _ => ()
            }
        }

        for key in to_remove {
            self.variables.remove(&key);
        }
    }
}

impl ReplaceVariables for Expression {
    fn try_replace_single_variable_with_value(&mut self,variable : &char,value : Number) -> Option<()> {
        match self {
            Expression::Term(term) => term.try_replace_single_variable_with_value(variable,value),
            Expression::Nested(inner) => inner.try_replace_single_variable_with_value(variable,value),
            Expression::Binary { left , right , ..} => {
                let left_result = left.try_replace_single_variable_with_value(variable, value.clone());
                let right_result = right.try_replace_single_variable_with_value(variable, value);

                match (left_result,right_result) {
                    (None,None) => None,
                    _ => Some(()),
                }
            }
        }
    }

    fn try_replace_variables_with_value(&mut self,variable_values : &mut Variables) {
        match self {
            Expression::Term(term) => term.try_replace_variables_with_value(variable_values),
            Expression::Nested(inner) => inner.try_replace_variables_with_value(variable_values),
            Expression::Binary { left , right , ..} => {
                left.try_replace_variables_with_value(variable_values);
                right.try_replace_variables_with_value(variable_values);
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
        let result = expression.try_replace_single_variable_with_value(&'x', Number::Decimal(4.0));
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
        let result = expression.try_replace_single_variable_with_value(&'x', Number::Decimal(4.0));
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
        result.try_replace_variables_with_value(&mut variable_values);

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
        result.try_replace_variables_with_value(&mut variable_values);

        // Check that 'x' variable was not found, so the term remains unchanged
        if let Expression::Term(new_term) = result {
            assert_eq!(new_term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
        } else {
            panic!("Expected Expression::Term after replacement.");
        }
    }
}
