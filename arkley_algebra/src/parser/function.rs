use nom::{
    IResult, 
    character::complete::{char, multispace0}, 
    sequence::{delimited, terminated}, 
    combinator::all_consuming, 
    bytes::complete::{take_till, take}
};

use crate::{Function, FunctionArguments, parse_expression, Context};
use super::satisfies_variable_name;

/// Parses a function definition from the given input string.
///
/// This function takes an input string, `input`, and attempts to parse a function definition
/// in the form of `name(arguments) = expression'.
pub fn parse_function_definition<'a>(context : &'a Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,Function> {
    move |input| {
        let (input,name) = Function::parse_name(input)?;
        let (input,arguments) = Function::parse_arguments(input)?;
        let (input,_) = delimited(multispace0, char('='), multispace0)(input)?;
    
        let (input,expression) = parse_expression(context)(input)?;
    
        let function = Function::new_default(name.into(),expression,arguments);
    
        Ok((input,function))
    }
}


/// Parses a custom function definition from the input string, using the provided `context`.
///
/// This function is designed to parse custom function definitions, which include user-defined functions
/// and potentially more complex functions , as an example trigonometric functions (e.g., cos, sin, tan) defined using
/// custom closures. The `context` parameter is used to provide context for parsing these functions.
pub fn parse_function<'a>(context : &'a Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,Function> {
    move |input| {
        let (input,name) = Function::parse_name(input)?;
        let (input,mut _arguments) = Function::parse_arguments_with_context(context)(input)?;

        let function = match context.functions().get(name) {
            None => todo!(), // for now , return error in future
            Some(closure) => {
                let mut func = closure();
                func.arguments.append(&mut _arguments);

                func
            }
        };

        
        Ok((input,function))
    }
}

impl Function {
    fn parse_name(input : &str) -> IResult<&str,&str> {
        let (input,mut s) = terminated(
            take_till(|c| c == '('), 
            take(1u8) // to remove extra ( at end
        )(input)?;

        s = s.trim();

        Ok((input,s))
    }
    
    fn parse_arguments(input : &str) -> IResult<&str,FunctionArguments> {
        let (input,arguments_str) = take_till(|c| c == ')')(input)?;
    
        let mut arguments = FunctionArguments::new();

        for s in arguments_str.trim().split(',') {
            let (_,key) = all_consuming(satisfies_variable_name)(s)?;
            arguments.insert(key, None);
        }

        Ok((input,arguments))
    }

    fn parse_arguments_with_context<'a>(_context : &'a Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,FunctionArguments> {
        move |input| {
            let (input,arguments_str) = take_till(|c| c == ')')(input)?;

            let mut arguments = FunctionArguments::new();

            for s in arguments_str.trim().split(',') {
                match all_consuming(satisfies_variable_name)(s) {
                    Ok((_,key)) => arguments.insert(key, None),
                    // TODO : Figure out out the fuck to handle this, maybe convert [`FunctionArguments`] into Vec<..> , for now returned error
                    Err(err) => return Err(err)
                };
            }

            Ok((input,arguments))
        }
    }
}