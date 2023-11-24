use std::collections::HashMap;

use nom::{combinator::value, bytes::complete::{tag, take_until1, take},IResult, sequence::{preceded, delimited, terminated}, character::complete::{multispace0, char}};

use crate::{Expression, Function, parse_expression};

use super::{
    satisfies_variable_name,
    tokens::Token
};

/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
/// 
/// It allows using variables , user-defined functions within an expression used during the parsing phase.
/// When assigning to variables, the assignment is stored in a context. When the variable is read later on,
/// it is read from the context. Contexts can be preserved between multiple calls by creating them yourself.
/// 
/// TODO : Make parser for functions
#[derive(Clone, Debug,Default)]
pub struct Context<'a> {
    /// Used for storing input like
    /// ```
    /// a = 0
    /// b = 543x
    /// x = 4y + 5u
    /// ```
    values : HashMap<char,Expression>,

    tags : ContextHashMap<'a,Expression>,
    #[cfg(feature="function")]
    // used cuz functions will have different 'parsing' logic
    functions : ContextHashMap<'a,Function>
}

type ContextHashMap<'a,T> =  HashMap<&'a str,T>;

impl<'a> Context<'a,>{
    /// Gets reference to the values eg x = 10 
    pub const fn values(&self) -> &HashMap<char,Expression> {
        &self.values
    }
    /// Gets mutable reference to the values eg x = 10 
    pub fn values_mut(&mut self) -> &mut HashMap<char,Expression> {
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

    #[cfg(feature="function")]
    /// Gets reference to the function context 
    pub const fn functions(&self) -> &ContextHashMap<'a,Function> {
        &self.functions
    }

    /// Gets a mutable reference to the function context
    #[cfg(feature="function")]
    pub fn functions_mut(&mut self) -> &mut ContextHashMap<'a,Function> {
        &mut self.functions
    }
}

impl<'l> Context<'l> {
    pub(super) fn parse_tags_into_tokens<'a : 'b,'b>(&'b self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<Token>> + 'b {
        move |input|{
            let (input,expression) = alternative(&self.tags())(input)?;    

            let vec = vec![expression.into()];
            Ok((input,vec))
        }
    }

    /// Used for parsing input like
    /// ```
    /// speed_of_light = 3*10^8
    /// five_x = 5x
    /// magical_thing = 3x(4/x)
    /// ```
    pub fn parse_tags<'a : 'b,'b : 'l>(&'b mut self) -> impl FnMut(&'a str) -> IResult<&'a str,()> + 'b {
        move |input|{
            // space .. text .. space .. = .. expr
            let (input,tag_str) = terminated(
                take_until1("="),
                take(1usize)
            )(input)?;


            let (input,expression) = parse_expression(self)(input)?;

            let tag = tag_str.trim();
            self.tags_mut().insert(tag, expression);

            Ok((input,()))
        }
    }

    /// Used for parsing input like
    /// ```
    /// a = 0
    /// b = 543x
    /// x = 4y + 5u
    /// ```
    pub fn parse_values<'a : 'b,'b : 'l>(&'b mut self) -> impl FnMut(&'a str) -> IResult<&'a str,()> + 'b {
        move |input| {
            // space .. var .. space .. = .. expr
            let (input,symbol) = delimited(
                multispace0, 
                satisfies_variable_name,
                multispace0
            )(input)?;

            let (input,expression) = preceded(
                char('='),
                parse_expression(self)
            )(input)?;
            
            self.values_mut().insert(symbol, expression);

            Ok((input,()))
        }
    }

}

fn alternative<'a : 'b,'b,T : Clone>(alternatives: &'b HashMap<&'b str,T>) -> impl FnMut(&'a str) -> nom::IResult<&'a str,T> + 'b {
    move |input| {
        let mut last_err = Err(nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::NonEmpty }));

        for (key,t) in alternatives {
            match value(t,tag(*key))(input) {
                Ok((input,other)) => return Ok((input,(*other).clone())),
                error @ Err(_) => last_err = error,
            }
        }

        last_err.map(|(i,v)| (i,v.clone()))
    }
}