use crate::Expression;

pub use self::remove_unnecessary_parentheses::*;

use super::Simplify;

impl Simplify for Expression {
    fn simplify_structure(self) -> Self {
        let mut value = self;
        value.remove_unnecessary_parentheses();

        value
    }
}