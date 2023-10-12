use crate::Expression;

use super::Simplify;

impl Simplify for Expression {
    fn simplify_structure(self) -> Self {
        self.remove_unnecessary_parentheses()
    }
}