mod term;
mod expression;

use std::collections::BTreeMap;
use num_notation::Number;

use crate::{Variables, Expression};

/// Create a type alias for BTreeMap<char, Expression> 
pub type VariableExpressionAssociation = BTreeMap<char,Expression>;

/// A trait for types that support variable replacement.
///
/// Types implementing this trait can perform variable substitution in various ways (this is done for optimzation reasons).
pub trait VariableSubstitution {
    /// Attempts to replace a single variable with a specified value.
    ///
    /// # Arguments
    ///
    /// - `variable`: A reference to the variable (char) to be replaced.
    /// - `value`: The value (Number) to replace the variable with.
    ///
    /// # Returns
    ///
    /// An `Option<()>` where:
    /// - `Some(())` indicates the variable was found and successfully replaced.
    /// - `None` indicates the variable did not exist, and no replacement occurred.
    fn try_replace_single_variable_with_value(&mut self,variable : &char,value : Number) -> Option<()>;

    /// Attempts to replace multiple variables with specified values.
    ///
    /// # Arguments
    ///
    /// - `variable_values`: A reference to a `Variables` map containing variables and their values.
    ///
    /// # Returns
    ///
    /// The updated term with the specified variables replaced. Variables that do not exist in the term
    /// are left unchanged in the `variable_values` map given.
    fn try_replace_variables_with_value(&mut self,variable_values : &mut Variables);

    /// Attempts to replace a single variable with a specified expression.
    ///
    /// # Arguments
    ///
    /// - `variable`: A reference to the variable (char) to be replaced.
    /// - `value`: The expression (Expression) to replace the variable with.
    fn try_replace_single_variable_with_expr(&mut self,_variable : &char,_value : Expression) -> Expression {
        todo!("power for expression needs to be implemented")
    }

    /// Attempts to replace multiple variables with specified expressions.
    ///
    /// # Arguments
    ///
    /// - `variable_values`: A reference to a `BTreeMap<char, Expression>` containing variables and their expressions.
    fn try_replace_variables_with_expr(&mut self,_variable_values : &mut VariableExpressionAssociation) -> Expression {
        todo!("power for expression needs to be implemented")
    }
}
