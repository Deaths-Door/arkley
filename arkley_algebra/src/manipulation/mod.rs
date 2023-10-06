mod simplify;
mod evaluation;

mod variable_analysis;
mod variable_substitution;

pub use simplify::*;
pub use evaluation::*;

pub use variable_analysis::*;
pub use variable_substitution::*;

/// Create a type alias for BTreeMap<char, Expression> 
pub type VariableExpressionAssociation = std::collections::BTreeMap<char,crate::Expression>;