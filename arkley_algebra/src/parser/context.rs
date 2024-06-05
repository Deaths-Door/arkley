use std::{borrow::Cow, collections::{HashMap, HashSet}};
use nom::{bytes::complete::tag, combinator::value, error::{Error, ErrorKind}, IResult};

use crate::Expression;

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
    tags : Tags<'a>,
    additional_parsers : Vec<AdditionParsers>
}

/// Represents the possible values within a `Context`.
/// This enum defines the different types of data that can be stored in a `Context`:
/// - `Parsed(Expression)`: A successfully parsed expression ready for evaluation.
/// - `Pending(&'a str)`: Raw input string that has not yet been parsed.
#[derive(Clone, Debug)]
pub(super) enum Value<'a> {
    /// A parsed expression, holding the result of parsing an input string.
    Parsed(Expression),

    // TODO: convert this pending into parsed somehow which may solve the stack overflow
    /// Raw input string that needs to be parsed into an expression.
    Pending(&'a str),
}

/// An alias for a hash map that stores variable names (`&'a str`) and their corresponding `Value` instances.
type Tags<'a> = HashMap<&'a str,Value<'a>>;
type AdditionParsers = for<'l> fn (&'l str) -> IResult<&'l str,Expression>;

impl<'a> Context<'a> {
    /// Replaces the current variable mappings in the context with the provided `tags`.
    /// This method allows you to completely overwrite the existing variable mappings in the context with a new set of mappings
    /// Arguments:
    ///   * `tags`: A reference to a `Tags<'a>` hash map containing the new variable mappings.
    pub(super) fn set_tags(&mut self,tags : Tags<'a>) {
        self.tags = tags
    }

    /// Returns a reference to the current variable mappings in the context.
    pub(super) const fn tags(&self) -> &Tags<'a> {
        &self.tags
    }

    /// Returns a mutable reference to the current variable mappings in the context.
    pub(super) fn tags_mut(&mut self) -> &mut Tags<'a> {
        &mut self.tags
    }

    /// Adds a new variable mapping to the context with the provided name and a pending (unparsed) value.
    /// Arguments:
    ///   * `tag`: The name of the variable to be added (`&'a str`).
    ///   * `value`: The initial value (unparsed string) to be associated with the variable (`&'a str`).
    pub fn add_tag_str(&mut self,tag : &'a str,value : &'a str) {
        self.tags.insert(tag, Value::Pending(value));
    }

    /// Adds a new variable mapping to the context with the provided name and a parsed expression value.
    /// Arguments:
    ///   * tag: The name of the variable to be added (&'a str).
    ///   * value: The parsed expression value to be associated with the variable (Expression).
    pub fn add_tag_expr(&mut self,tag : &'a str,value : Expression) {
        self.tags.insert(tag, Value::Parsed(value));
    }
}


impl<'a> Context<'a> {
    /// Extends the current variable mappings in the context with additional mappings from an iterator.
    pub fn extend_tags_str<I>(&mut self,iter : I) where I : Iterator<Item = <<HashMap<&'a str,&'a str> as IntoIterator>::IntoIter as Iterator>::Item>   {
        self.tags.extend(iter.map(|(k,v)| (k,Value::Pending(v))));
    }

    /// Extends the current variable mappings in the context with additional mappings from an iterator.
    pub fn extend_tags_expr<I>(&mut self,iter : I) where I : Iterator<Item = <<HashMap<&'a str,Expression> as IntoIterator>::IntoIter as Iterator>::Item> {
        self.tags.extend(iter.map(|(k,v)| (k,Value::Parsed(v))));
    }

    /// Provides a mutable reference to the list of additional parsers stored in the context.
    /// This method allows you to add, remove, or modify custom parsers that will be used during the parsing process. 
    /// These parsers are invoked in addition to the default parsing logic.
    /// Returns:
    ///   A mutable reference to the `Vec<AdditionParsers>` containing the additional parsers.
    pub fn additional_parsers_mut(&mut self) -> &mut Vec<AdditionParsers> {
        &mut self.additional_parsers
    }
}

impl<'ctx> Context<'ctx> {
    pub(super) fn run_additional_parsers<'a :'b,'b>(&'b self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b  {
        move|input| {
            let mut last_err : Option<_> = None;
            
            for parser in &self.additional_parsers {
                match parser(input) {
                    Ok((input,expression)) => return Ok((input,vec![ExpressionToken::Expression(expression)])),
                    current_error@ Err(_) => {
                        last_err = Some(current_error);
                    },
                }
            };
            
            Err(last_err.unwrap_or_else(||Err(nom::Err::Error(Error { input, code: ErrorKind::Fail }))).unwrap_err())
        }
    }

    pub(super) fn parse_tags<'a :'b,'b>(&'b self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b  {
        move |input| {
            match self.tags()
                .iter()
                .find(|(key,_)| input.starts_with(*key)
            ) {
                None => Err(nom::Err::Error(Error { input, code: ErrorKind::Tag })),
                Some((key,value)) => {
                    let remaining_input =&input[key.len()..];
                    
                    match value {
                        Value::Parsed(expression) => Ok((
                            remaining_input,
                            vec![ExpressionToken::Expression((*expression).clone())]
                        )),
                        Value::Pending(tag_input) => {
                            // TODO: Update the $value to Value::Parsed(expression)
                            let expression = match Expression::try_from((*tag_input,self)) {
                                Ok(value) => value,
                                Err(_error) => return Err(_error.map_input(|_| input))
                            };

                            Ok((
                                remaining_input,
                                vec![ExpressionToken::Expression(expression)]
                            ))
                        }
                    }
                },
            }
        }
    }
}

impl From<Expression> for Value<'_> {
    fn from(value: Expression) -> Self {
        Self::Parsed(value)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Self::Pending(value)
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

        let mut context = Context::default();
        context.extend_tags_expr([("gravity",expression.clone())].into_iter());

        assert_eq!(
            context.parse_tags()("gravity".into()),
            Ok(("".into(),vec![ExpressionToken::Expression(expression)]))
        );
    }
}