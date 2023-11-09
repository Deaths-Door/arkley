use num_notation::Number;

use crate::{Term,Expression,manipulation::Find, Function};

use super::{VariableSubstitution, SingleVariableReplacements, MultipleVariableReplacements};
// Term 

macro_rules! impl_trait {
    ($($t : ty),*) => {
        $(
            impl VariableSubstitution<$t> for Term {}
            
            impl VariableSubstitution<$t> for Expression {}
            
            impl VariableSubstitution<$t> for Function {}

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
    };
}
impl_trait!(term => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
impl_trait!(u8,u16,u32,u64,i8,i16,i32,i64,f32,f64,Number,Term,Function,Expression);

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
        if self.values.is_empty() {
            return self.source.into()
        }

        todo!("Implement this once `Pow` trait for expressions and terms is finished")
    }
}

impl Find for MultipleVariableReplacements<'_,Term,Expression> {
    type Output = Expression;
    fn find(self) -> Self::Output {
        if self.values.is_empty() {
            return self.source.into()
        }

        todo!("Implement this once `Pow` trait for expressions and terms is finished")
    }
}

// -----------------
// Function
impl<T : Into<Expression>> Find for SingleVariableReplacements<Function,T> {
    type Output = Function;
    fn find(self) -> Self::Output {
        let mut func = self.source;
        let variable = self.variable;

        let value = Some(self.value.into());

        match func.arguments.get_mut(&variable) {
            Some(e) => *e = value,
            None => {
                func.arguments.insert(variable, value);
            },
        };

        func
    }
}

impl<'a,T : Clone + Into<Expression>> Find for MultipleVariableReplacements<'a,Function,T> {
    type Output = Function;
    fn find(self) -> Self::Output {
        let mut func = self.source;
        let values = self.values;
        
        for (variable,value) in values {
            let value = Some((*value).clone().into());

            match func.arguments.get_mut(variable) {
                Some(e) => *e = value,
                None => {
                    func.arguments.insert(*variable, value);
                },
            };
        }
    
        func
    }
}

// -----------------
// Expression

impl<T : Clone> Find for SingleVariableReplacements<Expression,T> 
    where 
    Term: VariableSubstitution<T>,
    Expression : VariableSubstitution<T>
         + From<<SingleVariableReplacements<Term,T> as Find>::Output>
         + From<<SingleVariableReplacements<Function, T> as Find>::Output>,
    Function : VariableSubstitution<T> ,
    SingleVariableReplacements<Term, T> : Find,
    SingleVariableReplacements<Function, T> : Find
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
            Expression::Function(func) => func.replace_single_variable(variable, value).find().into(),
        }
    }
}

impl<'a,T : Clone + 'a> Find for MultipleVariableReplacements<'a,Expression,T> 
    where 
    Term: VariableSubstitution<T>,
    Expression : VariableSubstitution<T> 
        + From<<MultipleVariableReplacements<'a,Term, T> as Find>::Output>
        + From<<MultipleVariableReplacements<'a,Function, T> as Find>::Output>,
    Function : VariableSubstitution<T> ,

    MultipleVariableReplacements<'a,Term, T> : Find,
    MultipleVariableReplacements<'a,Function, T> : Find
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
            Expression::Function(func) => func.replace_variables(values).find().into(),
        }
    }
}

// -----------------
// Equation

#[cfg(feature="equation")]
impl<T : Clone> Find for SingleVariableReplacements<crate::Equation,T> 
    where 
    Expression: VariableSubstitution<T> 
        + From<<SingleVariableReplacements<Expression,T> as Find>::Output> 
        + From<<SingleVariableReplacements<Term,T> as Find>::Output>,
    Term: VariableSubstitution<T>,
    SingleVariableReplacements<Expression,T> : Find ,
    SingleVariableReplacements<Term, T> : Find  {
    type Output = crate::Equation;
    fn find(self) -> Self::Output {
        let value = self.value;
        let variable = &self.variable;
        let mut equation = self.source;

        equation.right = equation.right.replace_single_variable(variable, value.clone()).find().into();
        equation.left = equation.left.replace_single_variable(variable, value).find().into();
        equation
    }
}

#[cfg(feature="equation")]
impl<'a,T : Clone> Find for MultipleVariableReplacements<'a,crate::Equation,T> 
    where 
    Expression: VariableSubstitution<T> 
        + From<<MultipleVariableReplacements<'a,Expression,T> as Find>::Output> 
        + From<<MultipleVariableReplacements<'a,Term,T> as Find>::Output>,
    Term: VariableSubstitution<T>,
    MultipleVariableReplacements<'a,Expression,T> : Find ,
    MultipleVariableReplacements<'a,Term, T> : Find  {
    type Output = crate::Equation;
    fn find(self) -> Self::Output { 
        let values = self.values;
        let mut equation = self.source;

        equation.right = equation.right.replace_variables(values).find().into();
        equation.left = equation.left.replace_variables(values).find().into();
        equation
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