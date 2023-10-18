use std::{
    collections::{HashSet, HashMap}, 
    fmt::{Debug, Display}, 
    sync::RwLock
};

use lazy_static::lazy_static;

use crate::Expression;

lazy_static! {
    #[allow(missing_docs)]
    pub static ref FUNCTIONS : RwLock<HashMap<&'static str,Function>> = HashMap::new().into();
}

macro_rules! function_get {
    ($name : expr) => {
        FUNCTIONS.read().unwrap().get($name).unwrap()
    };
}

pub(crate) use function_get;

/// Represents a mathematical function with a name and a set of arguments.
pub struct Function {
    name: &'static str,
    arguments: HashSet<char>,
    expression : Expression
}

impl Display for Function {
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

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl Function {
    /// Creates a new function with the given name and an empty set of arguments.
    pub fn new(name: &'static str,expression : Expression) -> Self {
        Self { name, arguments: HashSet::new() , expression }
    }

    /// Creates a new function with the given name and a set of arguments.
    pub fn new_with_arguments(name: &'static str,expression : Expression, arguments: HashSet<char>) -> Self {
        Self { name, arguments , expression }
    }

    /// Adds an argument to the function.
    pub fn add_argument(&mut self, arg: char) {
        self.arguments.insert(arg);
    }

    /// Get underlying arguments
    #[inline]
    pub const fn arguments(&self) -> &HashSet<char> {
        &self.arguments
    }

    /// Get underlying name
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}
