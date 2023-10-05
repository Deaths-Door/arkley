use crate::Expression;

use super::Simplify;

impl Simplify for Expression {
    fn simplify_structure(self) -> Self {
        let mut value = self;
        value.remove_unnecessary_parentheses();

        value
    }
}