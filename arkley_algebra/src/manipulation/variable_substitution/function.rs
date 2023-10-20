use crate::{Function, Expression};

use super::VariableSubstitution;

impl<SV : Clone,MV> VariableSubstitution<SV,MV> for Function where Expression : VariableSubstitution<SV,MV> {
    fn replace_single_variable(&mut self, variable: &char, value: SV) -> Option<()> {
        let r = self.arguments.iter_mut()
            .map(|(_,expr)| match expr {
                None => None,
                Some(e) => e.replace_single_variable(variable, value.clone())
            })
            .any(|v| v.is_some());

        if r { Some(()) } else { None }
    }

    fn replace_variables(&mut self, variable_values:&mut MV) {
        for (_,expression) in self.arguments.iter_mut() {
            if let Some(e) = expression {
                e.replace_variables(variable_values)
            }
        }        
    }
}