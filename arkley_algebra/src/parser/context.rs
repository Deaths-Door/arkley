use std::{borrow::{Borrow, Cow}, collections::HashMap};

use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::value, error::{Error, ErrorKind}, IResult, InputLength, Or, Parser};

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
    tags : ContextHashMap<'a,Expression>,
}

type ContextHashMap<'a,T> =  HashMap<&'a str,T>;

impl<'a> Context<'a,> {
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
            match self.tags()
                .iter()
                .find(|(key,_)| input.starts_with(*key)) {
                Some((key,value)) => {
                    println!("found => {key} for {input} so new input is {}",&input[key.len()..]);
                    Ok((&input[key.len()..],vec![ExpressionToken::Expression((*value).clone())]))
                },
                None => Err(nom::Err::Error(Error { input, code: ErrorKind::Fail }))
            }
        }
    }
}

fn alternative<'a,T>(input: &str, alternatives: impl Iterator<Item = (&'a &'a str,&'a T)>) -> IResult<&str,T> where T : Clone + 'a {
    let mut last_err = None;

    for (key,generic_value) in alternatives {
        match value(generic_value,tag(*key))(input) {
            Ok((input,value)) => return Ok((input,(*value).clone())),
            Err(error) => {
                last_err = Some(error);
            }
        }
    }

    Err(last_err.unwrap_or(nom::Err::Error(Error { input, code: ErrorKind::NonEmpty })))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_alternatives(){
        let hashmap = HashMap::from([("gravity",1u32)]);
        assert_eq!(
            alternative("gravity", hashmap.iter()),
            Ok(("",1))
        )
    }

    
    #[test]
    fn parse_alternatives_tags(){
        let expression = Expression::new_term(9.81);
        let hashmap = HashMap::from([("gravity",expression.clone())]);
        let mut context = Context::default();
        *context.tags_mut() = hashmap;

        assert_eq!(
            context.parse_tags()("gravity"),
            Ok(("",vec![ExpressionToken::Expression(expression)]))
        );
    }
}