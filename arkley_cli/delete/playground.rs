use std::process::exit;

use arkley_algebra::{parse_expression, Context, manipulation::{EvaluteWithValues, Find}, parse_equation, parse_term, Equation, Term};
use nom::{
    sequence::{preceded, pair, delimited, tuple, terminated}, 
    character::complete::{multispace0, multispace1},
    combinator::map, 
    branch::alt, 
    bytes::complete::{tag, take_until, take}, IResult
};

use crate::{pretty_errors::{new_default_editor, self}, utils};


pub fn open(context : Context<'_>) {
    let mut rl = new_default_editor();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => parse_syntax(&line,&context),
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }
}

fn parse_syntax(input : &str,context : &Context<'_>) {
    let mut parser = alt((
        parse_reserved_commands,
        parse_evaluate_command(&context),
        parse_rearrange_command(&context)
    ));

    match parser(input)   {
        Ok(_) => (),
        Err(err) => eprintln!("Please provide valid syntax! : {:?}",err),
    }
}

/// Parses things like "--command" w/o whitespace
fn parse_reserved_commands(input : &str) -> IResult<&str,()> {
    const DOC_MSG : &str = r#"
For detailed information about how to use about the playground, please refer to our documentation:
                
https://github.com/Deaths-Door/arkley/tree/main/arkley_cli/README.md"#;
    
    preceded(
        pair(
            multispace0, 
            tag("--")
        ),
        alt((
            map(tag("help"),|_| println!("{DOC_MSG}")),
            map(tag("exit"),|_| exit(0))
        ))
    )(input)        
}

/// Parses `evaluate eq/expr`
fn parse_evaluate_command<'a>(context : &'a Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str,()>  {
    move |input| {
        let (input,_) = delimited(
            multispace0,
            tag("evaluate"),
            multispace1
        )(input)?;

        alt((
            map(parse_equation(&context),|v| println!("Result: {}",v.evaluate_with_multiple_values(context.values()).find())),
            map(parse_expression(&context),|v| println!("Result: {}",v.evaluate_with_multiple_values(context.values()).find()))
        ))(input)
    }
}

fn parse_rearrange_command<'a>(context : &'a Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str,()>  {
    move |input| {        
        let (input,_) = delimited(
            multispace0,
            tag("rearrange"),
            multispace1
        )(input)?;

        let (input,eq_str) = take_until("into")(input)?;
        let (input, _) = tag("into")(input)?;
        let (input, term_str) = multispace1(input)?;

        let equation : Option<Equation> = pretty_errors::try_from_with_message_no_exit((eq_str.trim_end(),context));
        let term  : Option<Term> = pretty_errors::try_from_with_message_no_exit(term_str.trim_end());
        
        if equation.is_some() && term.is_some() {
            utils::rearrange_equation(equation.unwrap(),term.unwrap());
        }


        Ok((input,()))
    }
}
/* 
/// rearrange .. eq into term
fn parse_rearrange_command<'a>(context : &'a Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str,()>  {
    move |input| {
        let (input,v) = pair(
            parse_command(
                "rearrange", parse_equation(&context), 
                |expression| println!("Result: {}",expression.evaluate_with_multiple_values(context.values()).find())
            ) , 
            preceded(
                tag("into"), 
                parse_term
            )
        )(input)?;
        
        /**/

    todo!()
    }
}
fn parse_command<'a,P,T,F>(command : &'a str,mut parser: P,map : F) -> impl FnMut(&'a str) -> IResult<&'a str,()> 
where
    P: FnMut(&'a str) -> IResult<&'a str,T> ,
    F : Fn(T) -> ()
    {
    // evalute .. expr 
    // rearrange .. expr
    // solve .. expr 
    move |input| {
        // space .. command .. space .. arg
        let (input,_) = delimited(
            multispace0,
            tag(command),
            multispace0
        )(input)?;

        let (input,arg) = (parser)(input)?;
    
        Ok((input,(map)(arg)))
    }
}*/