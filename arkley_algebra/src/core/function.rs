use std::fmt::{Debug, Display};

use crate::Expression;

/// Represents a mathematical function with a name and a set of arguments.
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Function {
    name: &'static str,
    pub(crate) arguments : Vec<(char,Option<Expression>)>,
    expression : Box<Expression>,
    pub(crate) closure : fn(Function) -> Expression,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name}({args}) = {expr}",
            name = self.name,
            args = self.arguments.iter().map(|(c,v)| match v {
                None => c.to_string(),
                Some(value) => value.to_string()
            })
            .collect::<Vec<String>>()
            .join(", "),
            expr = *self.expression
        )
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl Function {
    /// Creates a new function with the given name and an empty set of arguments.
    pub const fn new(name: &'static str,expression : Box<Expression>,closure : fn(Function) -> Expression) -> Self {
        Self::new_with_arguments(name, Vec::new(),expression, closure)
    }

    /// Creates a new function with the given name and a set of arguments.
    pub const fn new_with_arguments(
        name: &'static str,
        arguments : Vec<(char,Option<Expression>)>,
        expression : Box<Expression>,
        closure : fn(Function) -> Expression,
    ) -> Self {
        Self { name , arguments , expression , closure }
    }
}

impl Function {
    pub(crate) fn same(&self,other : &Function) -> bool {
        self.name == other.name
    }

    pub(crate) fn arguments_empty(&self,other : &Function) -> bool {
        self.arguments.is_empty() == other.arguments.is_empty()
    }
}