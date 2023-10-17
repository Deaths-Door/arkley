use std::{collections::{HashSet, HashMap}, fmt::Debug};

use lazy_static::lazy_static;

use crate::Expression;

lazy_static! {
    #[allow(missing_docs)]
    pub static ref FUNCTIONS : HashMap<&'static str,Function<'static>> =HashMap::new();
}

/// Represents a mathematical function with a name and a set of arguments.
pub struct Function<'a> {
    name: &'a str,
    arguments: HashSet<char>,
    expression : Expression
}

impl Debug for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name}({args}) = {expr}",
            name = self.name,
            args = self.arguments
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            expr = self.expression
        )
    }
}

impl<'a> Function<'a> {
    /// Creates a new function with the given name and an empty set of arguments.
    pub fn new(name: &'a str,expression : Expression) -> Self {
        Self { name, arguments: HashSet::new() , expression }
    }

    /// Creates a new function with the given name and a set of arguments.
    pub fn new_with_arguments(name: &'a str,expression : Expression, arguments: HashSet<char>) -> Self {
        Self { name, arguments , expression }
    }

    /// Adds an argument to the function.
    pub fn add_argument(&mut self, arg: char) {
        self.arguments.insert(arg);
    }

    /// Get underlying arguments
    pub const fn arguments(&self) -> &HashSet<char> {
        &self.arguments
    }

    /// Get underlying name
    pub const fn name(&self) -> &str {
        self.name
    }
}
