mod relation;
mod rearrange;
pub use relation::*;

use crate::Expression;

/// A struct representing an equation with left and right expressions and a relational operator.
#[derive(Clone)]
pub struct Equation {
    /// The left-hand side expression.
    pub(crate) left: Expression,
    /// The relational operator.
    relation: RelationalOperator,
    /// The right-hand side expression.
    pub(crate) right: Expression,
}

impl Equation {
    /// Create a new equation with the specified left and right expressions and a relational operator.
    pub const fn new(left: Expression, relation: RelationalOperator, right: Expression) -> Self {
        Equation { left, relation, right }
    }
}


impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}",self.left,self.relation,self.right)
    }
}

impl std::fmt::Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}",self.left,self.relation,self.right)
    }
}