use std::collections::HashMap;

use nom::{IResult, bytes::complete::{take_until1, take}};

use crate::{Expression, Function, parse_expression};

use super::tokens::Token;


/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
/// 
/// It allows using variables , user-defined functions within an expression used during the parsing phase.
/// When assigning to variables, the assignment is stored in a context. When the variable is read later on,
/// it is read from the context. Contexts can be preserved between multiple calls by creating them yourself.
/// 
/// TODO : Allow `context` to be in expression so no 'converting' maybe idk and also make the parser for values , tags and function thing
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

/*
fn alternative<'a,T : Clone>(alternatives: &'a std::collections::HashMap<&'a str,T>) -> impl FnMut(&'a str) -> nom::IResult<&'a str,T> {
    use nom::{combinator::value, bytes::complete::tag};

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

impl<'a : 'b,'b> Context<'a> {
    /*pub fn parse_values(&'b mut self) -> impl FnMut(&'a str) -> IResult<&'a str,> {
        move |input| {
            let (input,key_str) = take_until1("=")(input)?;
            let (input,_) = take(1usize)(input); // cuz take until = doesnt consume the =
            let (input,expression) = parse_expression(context)?;



            todo!()
        }
    }*/

    #[deprecated]
    pub(super) fn _parse_tags(&'a self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<Token>> {
        move |input| {
            // TODO : Check why does this fail at + five , parses first tag then an operator stops ; figure out solution for wheni first encountered this issue
            let (input,expression) = super::alternative(&self.tags())(input)?;    
                    
            let vec = vec![expression.into()];
            Ok((input,vec))
        }
    }
}*/