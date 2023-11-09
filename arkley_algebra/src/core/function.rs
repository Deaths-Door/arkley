use std::{fmt::{Debug, Display}, collections::{BTreeMap, HashMap}};

use crate::{Expression, manipulation::{VariableSubstitution, Find}};

/// Represents a mathematical function with a name and a set of arguments.
// TODO : Create functions to validate arguemnts given to functions + corrcet number + corrcte ones etc
#[derive(Clone,Hash)]
pub struct Function {
    // TODO : Rn use string into future change to maybe Cow<'a,str>
    pub(crate) name: String,//&'static str,
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
        if self.name != other.name {
            return false;
        }

        if self.arguments.len() != other.arguments.len() {
            return false;
        }

        match (&self.expression,&other.expression) {
            (None, Some(_)) |  (Some(_), None) => return false,
            _ => () // next check
        }

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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the arguments of the function.
    pub const fn arguments(&self) -> &FunctionArguments {
        &self.arguments
    }

    /// Gets the expression of the function, if available.
    pub const fn expression(&self) -> &Option<Box<Expression>> {
        &self.expression
    }

    /// Gets a mutable reference to the expression of the function, if available.
    pub fn expression_mut(&mut self) -> &mut Option<Box<Expression>> {
        &mut self.expression
    }

}

impl Function {
    /// Creates a new `Function` instance with a default closure function.
    ///
    /// This constructor creates a `Function` with the given `name`, `expression`, and `arguments`.
    /// It sets a default closure that processes the function's expression and arguments.
    pub fn new_default(name: String,expression : Expression,arguments : FunctionArguments) -> Self {
        let closure = |func: Function| {
            let mut arguments : HashMap<char,Expression> = func.arguments.into_iter()
                .filter(|(_,expr)| expr.is_some())
                .map(|(k,expr)| (k,expr.unwrap()))
                .collect();

            func.expression.unwrap().replace_variables(&mut arguments).find()
        };

        Self { name , arguments , expression : Some(Box::new(expression)) , closure }
    }
    
    /// Creates a new `Function` instance with a custom closure function.
    pub const fn new(name: String,closure: fn(Function) -> Expression) -> Self {
        Self { name , arguments : BTreeMap::new() , expression : None , closure }
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