use std::{borrow::{Borrow, Cow}, collections::HashMap};

use nom::{IResult,bytes::complete::tag, combinator::value};

use crate::{Expression, Variable};

use super::ExpressionToken;

/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
/// 
/// It allows using variables , user-defined functions within an expression used during the parsing phase.
/// When assigning to variables, the assignment is stored in a context. When the variable is read later on,
/// it is read from the context. Contexts can be preserved between multiple calls by creating them yourself.
#[derive(Clone,Debug,Default)]
pub struct Context<'a> {
    /// Used for storing input like
    /// ```no-test
    /// a = 0
    /// b = 543x
    /// x = 4y + 5u
    /// ```
    values : HashMap<Variable,Expression>,
    tags : ContextHashMap<'a,Expression>,
}

type ContextHashMap<'a,T> =  HashMap<&'a str,T>;

impl<'a> Context<'a,> {
    /// Gets reference to the values eg x = 10 
    pub const fn values(&self) -> &HashMap<Variable,Expression> {
        &self.values
    }
    /// Gets mutable reference to the values eg x = 10 
    pub fn values_mut(&mut self) -> &mut HashMap<Variable,Expression> {
        &mut self.values
    }

    /// Gets reference to the tags context 
    pub const fn tags(&self) -> &ContextHashMap<'a,Expression> {
        &self.tags
    }
 
    /// Gets a mutable reference to the tags context
    pub fn tags_mut(&mut self) -> &mut ContextHashMap<'a,Expression> {
        &mut self.tags
    }
}

impl Context<'_> {
    pub(super) fn parse_tags<'a : 'b,'b>(&'b self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,tag_expr) = alternative(&self.tags())(input)?;
            let vec = vec![ExpressionToken::Expression(tag_expr)];
            Ok((input,vec))
        }
    }

    pub(super) fn parse_values<'a : 'b,'b>(&'b self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,tag_expr) = alternative(&self.values())(input)?;
            let vec = vec![ExpressionToken::Expression(tag_expr)];
            Ok((input,vec))
        }
    }
}

fn alternative<'a : 'b,'b,T,K>(alternatives: &'b HashMap<K,T>) -> impl FnMut(&'a str) -> nom::IResult<&'a str,T> + 'b  where T: Clone , K : ToString {
    move |input| {
        let mut last_err = Err(nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::NonEmpty }));

        for (key,t) in alternatives {
            match value(t,tag(key.to_string().as_str()))(input) {
                Ok((input,other)) => return Ok((input,(*other).clone())),
                error @ Err(_) => last_err = error,
            }
        }

        last_err.map(|(i,v)| (i,v.clone()))
    }
}