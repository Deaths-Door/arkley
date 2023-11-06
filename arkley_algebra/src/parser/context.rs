use std::collections::HashMap;

use nom::IResult;

use crate::{Expression, Function};

use super::tokens::Token;


/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
/// 
/// It allows using variables , user-defined functions within an expression used during the parsing phase.
/// When assigning to variables, the assignment is stored in a context. When the variable is read later on,
/// it is read from the context. Contexts can be preserved between multiple calls by creating them yourself.
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

impl<'a> Context<'a> {
    pub(super) fn parse_tags(&'a self) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<Token>> {
        move |input| {
            // TODO : Check why does this fail at + five , parses first tag then an operator stops ; figure out solution for wheni first encountered this issue
            let (input,expression) = super::alternative(&self.tags())(input)?;    
                    
            let vec = vec![expression.into()];
            Ok((input,vec))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Context, parse_expression, parser::tokens::Token};
    #[test]
    fn parsing_tag() {
        let mut context = Context::default();
        context.tags_mut().insert("five", 5.into());
        context.tags_mut().insert("two", 2.into());
        context.tags_mut().insert("sieben", 7.into());

        let result = context.parse_tags()("five + sieben");
        
        assert!(result.is_ok());

        let (input,vec) = result.unwrap();

        println!("{input}");

        let expr = Token::into_expression_tree(Token::to_rpn(vec));
        assert_eq!(&expr.to_string(),"5 + 7")
    }

    #[test]
    fn with_context() {
        let mut context = Context::default();
        context.tags_mut().insert("five", 5.into());
        context.tags_mut().insert("two", 2.into());
        context.tags_mut().insert("sieben", 7.into());

        let result = parse_expression(&context)("five * two + sieben");

        assert!(result.is_ok());

        assert_eq!(&result.unwrap().1.to_string(),"5 * 2 + 7")
    }
}