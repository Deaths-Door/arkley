use num_notation::{Number, Pow};

use crate::{Term, Variables};

use super::ReplaceVariables;

impl ReplaceVariables for Term {
    fn try_replace_single_variable_with_value(&mut self,variable : &char,value : Number) -> Option<()> {
        self.variables.remove(variable).and_then(|exponent| {
            self.coefficient = self.coefficient * value.pow(exponent);
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
