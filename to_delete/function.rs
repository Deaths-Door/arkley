use std::collections::BTreeMap;

use crate::{Function, Expression};

use super::{VariableSubstitution, replace_variables::VariableReplacements};

impl <SingleValue,MultipleValues> VariableSubstitution<SingleValue,MultipleValues> for Function 
    where Expression : VariableSubstitution<SingleValue,MultipleValues> , SingleValue : Clone {

    fn replace_single_variable(&mut self, variable: &char, value: SingleValue) -> Option<()> {
        let r = self.arguments.iter_mut()
            .map(|(_,expr)| match expr {
                None => None,
                Some(e) => e.replace_single_variable(variable, value.clone())
            })
            .any(|v| v.is_some());

        if r { Some(()) } else { None }
    }

    fn replace_variables(mut self, variable_values : &MultipleValues) -> Self {
        self.arguments = self.arguments.into_iter()
            .filter(|(_,e)| e.is_some())
            .fold(BTreeMap::new(),|mut acc,(k,e)|{
                acc.insert(k,Some(e.unwrap().replace_variables(variable_values)));
                acc
            });

        self
    }

    fn with_single_variable_replacement(self, _: char, _: SingleValue) -> VariableReplacements<Self,MultipleValues> where Self : Sized {
        todo!()
    }
}