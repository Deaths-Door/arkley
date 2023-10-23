use std::{fmt::{Debug, Display}, collections::{BTreeMap, HashMap}};

use crate::{Expression, manipulation::VariableSubstitution};

/// Represents a mathematical function with a name and a set of arguments.
#[derive(Clone,Hash)]
pub struct Function {
    pub(crate) name: &'static str,
    pub(crate) arguments : FunctionArguments,
    pub(crate) expression : Option<Box<Expression>>,
    pub(crate) closure : fn(Function) -> Expression,
}

impl Eq for Function {}

impl PartialEq for Function {
    // If both instances are expressions (expr),
    // and if both expressions are terms (term),
    // then it tries to compare them for equality (eq).
    // If any of these conditions are not met, it returns false.
    fn eq(&self, other: &Self) -> bool {
        self.arguments.iter()
            .all(|(key,_svalue)| match other.arguments.get(key) {
                None => false,
                Some(_ovalue) => match (_svalue,_ovalue) {
                    (Some(svalue), Some(ovalue)) => match (svalue,ovalue) {
                        (Expression::Term(t1), Expression::Term(t2)) => match t1.variables.is_empty() && t2.variables.is_empty() {
                            true => t1.coefficient == t2.coefficient,
                            false => false
                        },
                        _ => false
                    },
                    _ => false
                }
            })
    }
}


pub(crate) type FunctionArguments = BTreeMap<char,Option<Expression>>;

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{name}({args})",
            name = self.name,
            args = self.arguments.iter().map(|(c,v)| match v {
                None => c.to_string(),
                Some(value) => value.to_string()
            })
            .collect::<Vec<String>>()
            .join(", "),
        )
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl Function {
    /// Gets the name of the function.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Gets the arguments of the function.
    pub const fn arguments(&self) -> &FunctionArguments {
        &self.arguments
    }

    /// Gets the expression of the function, if available.
    pub const fn expression(&self) -> &Option<Box<Expression>> {
        &self.expression
    }

    /// Gets a mutable reference to the arguments of the function, allowing you to modify them.
    pub fn arguments_mut(&mut self) -> &mut FunctionArguments {
        &mut self.arguments  
    }
}

impl Function {
    /// Creates a new `Function` instance with a default closure function.
    ///
    /// This constructor creates a `Function` with the given `name`, `expression`, and `arguments`.
    /// It sets a default closure that processes the function's expression and arguments.
    pub fn new_default(name: &'static str,expression : Expression,arguments : FunctionArguments) -> Self {
        let closure = |func: Function| {
            let mut arguments : HashMap<char,Expression> = func.arguments.into_iter()
                .filter(|(_,expr)| expr.is_some())
                .map(|(k,expr)| (k,expr.unwrap()))
                .collect();

            let mut expr = func.expression.unwrap();
            
            expr.replace_variables(&mut arguments);

            *expr
        };

        Self { name , arguments , expression : Some(Box::new(expression)) , closure }
    }
    
    /// Creates a new `Function` instance with a custom closure function.
    pub const fn new(name: &'static str,closure: fn(Function) -> Expression) -> Self {
        Self { name , arguments : BTreeMap::new() , expression : None , closure }
    }

    /// Sets the arguments of the function.
    pub fn with_arguments(mut self, arguments: FunctionArguments) -> Self {
        self.arguments = arguments;
        self
    }

    /// Sets the expression of the function.
    pub fn with_expression(mut self, expression: Expression) -> Self {
        self.expression = Some(Box::new(expression));
        self
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