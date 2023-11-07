use num_notation::Number;

use crate::{Term,Expression,manipulation::Find};

use super::{VariableSubstitution, SingleVariableReplacements, MultipleVariableReplacements};

macro_rules! impl_trait {
    ($($t : ty),*) => {
        $(
            impl VariableSubstitution<$t> for Term {}
            
            impl VariableSubstitution<$t> for Expression {}
            
            #[cfg(feature="equation")]
            impl VariableSubstitution<$t> for crate::Equation {}
        )*
    };

    (term => $($value : ty),*) => {
        $(
            impl Find for SingleVariableReplacements<Term,$value> {
                type Output = Term;
                fn find(self) -> Self::Output {                    
                    let mut term = self.source;
                    if let Some(exponent) = term.variables.remove(&self.variable) {
                        term.coefficient *= (self.value as f64).powf(f64::from(exponent));
                    }
                    
                    term
                }
            }

            impl Find for MultipleVariableReplacements<'_,Term,$value> {
                type Output = Term;
                fn find(self) -> Self::Output {
                    let mut term = self.source;
                    for (k,v) in self.values.iter() {
                        if let Some(exponent) = term.variables.remove(k) {
                            term.coefficient *= (*v as f64).powf(f64::from(exponent));
                        }
                    }

                    term
                }
            }
        )*
    }
}
impl_trait!(term => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
impl_trait!(u8,u16,u32,u64,i8,i16,i32,i64,f32,f64,Number,Term,Expression);

impl Find for SingleVariableReplacements<Term,Number> {
    type Output = Term;
    fn find(self) -> Self::Output {                    
        let mut term = self.source;
        if let Some(exponent) = term.variables.remove(&self.variable) {
            term.coefficient *= f64::from(self.value).powf(f64::from(exponent));
        }
        
        term
    }
}

impl Find for MultipleVariableReplacements<'_,Term,Number> {
    type Output = Term;
    fn find(self) -> Self::Output {
        let mut term = self.source;
        for (k,v) in self.values.iter() {
            if let Some(exponent) = term.variables.remove(k) {
                term.coefficient *= f64::from((*v).clone()).powf(f64::from(exponent));
            }
        }

        term
    }
}

impl Find for SingleVariableReplacements<Term,Term> {
    type Output = Expression;
    fn find(self) -> Self::Output {
        match self.value.variables.is_empty() {
            true => self.source.replace_single_variable(&self.variable, self.value.coefficient).find().into(),
            false => todo!("Implement this once `Pow` trait for expressions and terms is finished"),
        }
    }
}

impl Find for SingleVariableReplacements<Term,Expression> {
    type Output = Expression;
    fn find(self) -> Self::Output {
        match self.value {
            Expression::Term(term) => self.source.replace_single_variable(&self.variable, term).find().into(),
            _ => todo!("Implement this once `Pow` trait for expressions and terms is finished")
        }
    }
}

impl Find for MultipleVariableReplacements<'_,Term,Term> {
    type Output = Expression;
    fn find(self) -> Self::Output {
        todo!("Implement this once `Pow` trait for expressions and terms is finished")
    }
}

impl Find for MultipleVariableReplacements<'_,Term,Expression> {
    type Output = Expression;
    fn find(self) -> Self::Output {
        todo!("Implement this once `Pow` trait for expressions and terms is finished")
    }
}

impl<T : Clone> Find for SingleVariableReplacements<Expression,T> 
    where 
    Term: VariableSubstitution<T>,
    Expression : VariableSubstitution<T>,
    Expression : From<<SingleVariableReplacements<Term,T> as Find>::Output>,
    SingleVariableReplacements<Term, T> : Find
    
    {

    type Output = Expression;
    fn find(self) -> Self::Output {
        let variable = &self.variable;
        let value = self.value;
        
        match self.source {
            Expression::Term(term) => term.replace_single_variable(variable, value).find().into(),
            Expression::Binary { operation, mut left, mut right } => {
                *left = left.replace_single_variable(variable, value.clone()).find();
                *right = right.replace_single_variable(variable, value).find();

                Expression::Binary { operation , left , right }
            },
            Expression::Nested(inner) => inner.replace_single_variable(variable, value).find().into(),
            Expression::Function(_) => todo!(),
        }
    }
}

impl<'a,T : Clone + 'a> Find for MultipleVariableReplacements<'a,Expression,T> 
    where 
    Term: VariableSubstitution<T>,
    Expression : VariableSubstitution<T> + From<<MultipleVariableReplacements<'a,Term, T> as Find>::Output>,
    MultipleVariableReplacements<'a,Term, T> : Find
    {

    type Output = Expression;
    fn find(self) -> Self::Output {
        let values = &self.values;

        match self.source {
            Expression::Term(term) => term.replace_variables(values).find().into(),
            Expression::Binary { operation, mut left, mut right } => {
                *left = left.replace_variables(values).find();
                *right = right.replace_variables(values).find();

                Expression::Binary { operation , left , right }
            },
            Expression::Nested(inner) => inner.replace_variables(values).find().into(),
            Expression::Function(_) => todo!(),
        }
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
        expression = expression.replace_single_variable(&'x', Number::Decimal(4.0)).find();
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
        expression = expression.replace_single_variable(&'x', Number::Decimal(4.0)).find();
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

        let result = expression.replace_variables(&variable_values).find();

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

        let result = expression.replace_variables(&variable_values).find();


        // Check that 'x' variable was not found, so the term remains unchanged
        if let Expression::Term(new_term) = result {
            assert_eq!(new_term.variables.get(&'y'), Some(&Number::Decimal(3.0)));
        } else {
            panic!("Expected Expression::Term after replacement.");
        }
    }
}