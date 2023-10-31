use nom::{
    IResult, 
    multi::many_till,
    character::complete::{char, multispace0}, 
    sequence::{delimited, terminated}, 
    combinator::{map, opt}, 
    bytes::complete::{take_till, take}
};

use crate::{Function, FunctionArguments, parse_expression, Context, satisfies_variable_name};

/* 
/*
// TODO : Add ability to parse user defined functions by parsing around context struct (maybe)
// TODO : Improve doc 
// TODO : allow more formats to be parsed of functions (which exist i think)
// can use to parse custom fn defination + context can be used to map more complex functions like cos , sin / tan with custom closue
pub fn parse_function_definition<'a>(context : &Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,Function> {
    move |input| {
        let (mut input,name) = Function::parse_name(input)?;

        let function = match context.functions().get(name) {
            Some(value) => value(),
            None => {
                let (_input,arguments) = Function::parse_arguments(input)?;
                let (_input,_) = delimited(multispace0, char('='),multispace0)(_input)?;
                let (_input,expression) = parse_expression(_input, context)?;   
                input = _input;

                Function::new_default(name, expression, arguments)     
            }
        };

        Ok((input,function))   
    }
}

pub fn parse_function<'a>(context : &Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,Function> {
    move |input: &str|{
        let (input,name) = Function::parse_name(input)?;
        let (input,arguments) = Function::parse_arguments(input)?;

        Ok((input,function))    
    }
}*/

impl Function {
    fn parse_name(input : &str) -> IResult<&str,&str> {
        let (input,mut s) = terminated(
            take_till(|c| c == '('), 
            take(1u8) // to remove extra ( at end
        )(input)?;

        s = s.trim();

        Ok((input,s))
    }

    //  space .. name ... opt(comma) 
    fn parse_arguments(input : &str) -> IResult<&str,FunctionArguments> {
        // gets the arguments as a str eg f(x,y) => (x,y) => x,y
        let (mut input,arguments_str) = take_till(|c| c == ')')(input)?;

        let mut args = FunctionArguments::new();

        for string in arguments_str.trim().split(',') {
            match string.len() == 1 {
                true => {
                    let (i,key) = satisfies_variable_name(input);
                    args.insert(key, None);
                    input = i;
                },
                false => {

                }
            }
        }

        todo!()
        /*let parser = many_till(
            delimited(
                multispace0, 
                super::satisfies_variable_name,
                opt(char(','))
            ),
            char(')')
        );

        map(parser,|(variables,_)| variables.into_iter().map(|c| (c,None)).collect())(input)*/
    }
}
*/