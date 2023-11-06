use num_notation::Number;

use crate::{Term,Expression,manipulation::Find, Equation};

use super::{VariableSubstitution, SingleVariableReplacements, MultipleVariableReplacements};

macro_rules! impl_trait {
    (term => $($value : ty),*) => {
        $(
            impl Find for SingleVariableReplacements<Term,$value> {
                type Output = Term;
                fn find(mut self) -> Self::Output {
                    if let Some(exponent) = self.source.variables.remove(&self.variable) {
                        self.coefficient *= self.value.pow(exponent);
                    }
                    
                    self
                }
            }

            impl Find for MultipleVariableReplacements<'_,Term,$value> {
                type Output = Term;
                fn find(mut self) -> Self::Output {
                    for (k,v) in self.values.iter() {
                        if let Some(exponent) = self.source.variables.remove(k) {
                            self.coefficient *= self.value.pow(exponent);
                        }
                    }

                    self
                }
            }
        )*
    }
}
impl_trait!(term => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64,Number);

impl<T> VariableSubstitution for Term {
    type Input = T;
}

impl<T> VariableSubstitution for Expression {
    type Input = T;
}

impl<T> VariableSubstitution for Equation {
    type Input = T;
}

impl Find for SingleVariableReplacements<Term,Term> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        match self.value.variables.is_empty() {
            true => self.source.replace_single_variable(&self.variable, self.value.coefficient),
            false => todo!("Implement this once `Pow` trait for expressions and terms is finished"),
        }
    }
}

impl Find for MultipleVariableReplacements<'_,Term,Term> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        todo!("Implement this once `Pow` trait for expressions and terms is finished")
    }
}

impl Find for SingleVariableReplacements<Term,Expression> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        match self.value {
            Expression::Term(term) => self.source.replace_single_variable(&self.variable, term),
            _ => todo!("Implement this once `Pow` trait for expressions and terms is finished")
        }
    }
}

impl Find for MultipleVariableReplacements<'_,Term,Expression> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        todo!("Implement this once `Pow` trait for expressions and terms is finished")
    }
}

impl<T> Find for SingleVariableReplacements<Expression,T> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        let variable = &self.variable;
        let value = self.value;

        match self {
            Expression::Term(term) => term.replace_single_variable(variable,value).find(),
            Expression::Nested(inner) => inner.replace_single_variable(variable,value).find(),
            Expression::Binary { operation , mut left , mut right } => {
                left = left.replace_single_variable(variable, value.clone()).find();
                right = right.replace_single_variable(variable, value).find();

                Expression::Binary { operation , left , right }
            },
            Expression::Function(func) => func.replace_single_variable(variable, value).find(),
        }
    }
}

impl<T> Find for MultipleVariableReplacements<'_,Expression,T> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        let values = &self.values;
        match self {
            Expression::Term(term) => term.replace_variables(values).find(),
            Expression::Nested(inner) => inner.replace_variables(values).find(),
            Expression::Binary { operation , mut left , mut right } => {
                left  = left.replace_variables(values).find();
                right = right.replace_variables(values).find();

                Expression::Binary { operation , left , right }

            },
            Expression::Function(func) => func.replace_variables(values).find(),
        }
    }
}

impl<T> Find for SingleVariableReplacements<Equation,T> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        self.left = self.left.replace_single_variable(self.variable,self.value).find();
        self.right = self.right.replace_single_variable(self.variable,self.value).find();
        self
    }
}

impl<T> Find for MultipleVariableReplacements<'_,Equation,T> {
    type Output = Term;
    fn find(mut self) -> Self::Output {
        self.left = self.left.replace_variables(self.values).find();
        self.right = self.right.replace_variables(self.values).find();
        self
    }
}

#[cfg(test)]
mod term {
    use crate::Variables;

    use super::*;
    use std::collections::HashMap;
    

    #[test]
    fn try_replace_single_variable_success() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0))]));
        let result = term.replace_single_variable(&'x', Number::Decimal(4.0));
        assert_eq!(result, Some(()));
        // Check that 'x' variable was replaced with 4.0
        assert_eq!(term.variables.get(&'x'), None);
    }

    #[test]
    fn try_replace_single_variable_failure() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let result = term.replace_single_variable(&'x', Number::Decimal(4.0));
        assert_eq!(result, None);
        // Check that 'x' variable was not found, so the term remains unchanged
        assert_eq!(term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
    }

    #[test]
    fn try_replace_variables_success() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0)),('y',Number::Decimal(4.0))]));
        let mut variable_values = HashMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        variable_values.insert('z', Number::Decimal(6.0));
        term = term.replace_variables(&mut variable_values);
        // Check that 'x' variable was replaced with 5.0 and 'z' remains unchanged
        assert_eq!(term.variables.get(&'x'), None);
        assert_eq!(variable_values.get(&'z'), Some(&Number::Decimal(6.0)));
    }

    #[test]
    fn try_replace_variables_failure() {
        let mut term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('y',Number::Decimal(3.0))]));
        let mut variable_values = HashMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        term = term.replace_variables(&mut variable_values);
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
        let mut variable_values = HashMap::new();
        variable_values.insert('x', Number::Decimal(2.0));
        variable_values.insert('z', Number::Decimal(4.0));

        // Call try_replace_variables_with_value on the term
        term = term.replace_variables(&mut variable_values);

        // Check if 'x' was replaced with 2.0 and 'y' was not present
        assert_eq!(term.variables.get(&'x'), None);
        assert_eq!(term.variables.get(&'y'), Some(&Number::Decimal(2.0)));

        // Check if the coefficient was updated correctly 2 * (2 ^3) = 16
        assert_eq!(term.coefficient, Number::Decimal(16.0));

        // Check if 'z' was not affected in the variable_values map
        assert_eq!(variable_values.get(&'z'), Some(&Number::Decimal(4.0)));
    }
}

#[cfg(test)]
mod expr {
    use super::*;
    use crate::{Term, Variables};
    
    use num_notation::Number;
    use std::collections::HashMap;

    #[test]
    fn try_replace_single_variable_success() {
        let term = Term::new_with_variable(Number::Decimal(2.0),Variables::from([('x',Number::Decimal(3.0))]));
        let mut expression = Expression::Term(term.clone());
        let result = expression.replace_single_variable(&'x', Number::Decimal(4.0));
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
        let result = expression.replace_single_variable(&'x', Number::Decimal(4.0));
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
        let expression = Expression::Term(term);

        let mut variable_values = HashMap::new();
        variable_values.insert('x', Number::Decimal(5.0));
        variable_values.insert('z', Number::Decimal(6.0));

        let result = expression.replace_variables(&variable_values);

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

        let mut variable_values = HashMap::new();
        variable_values.insert('x', Number::Decimal(5.0));

        let result = expression.replace_variables(&variable_values);


        // Check that 'x' variable was not found, so the term remains unchanged
        if let Expression::Term(new_term) = result {
            assert_eq!(new_term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
        } else {
            panic!("Expected Expression::Term after replacement.");
        }
    }
}
