use std::process::exit;

use arkley_algebra::{Context, parse_term};
use arkley_describe::fluent_templates::LanguageIdentifier;
use rustyline::DefaultEditor;
use nom::{
    sequence::{preceded, pair, terminated, separated_pair, delimited}, 
    character::complete::{multispace0, multispace1},
    combinator::{map, opt, map_res}, 
    branch::alt, 
    bytes::complete::{tag, take_until}, IResult
};

use crate::utils::{find_or_describe, command_evaluate, self};

pub fn open(mut context : Context<'_>) {
    let mut rl = new_default_editor();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => parse_syntax(&line,&mut context),
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }
}

fn new_default_editor() -> DefaultEditor {
    match DefaultEditor::new() {
        Ok(e) => e,
        Err(error) => {
            eprintln!("Error creating playground {error}");
            exit(1)
        },
    }
}

fn parse_syntax(input : &str,context : &Context<'_>) {
    let mut f = alt((
        parse_reserved_commands,

        parse_evaluate(context),
        parse_rearrange(context)
    ));

    match f(input)   {
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

fn parse_evaluate<'a : 'b,'b>(context : &'b Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str,()> + 'b {
    move |input| {
        let (input,locale) = parse_command_syntax("evaluate")(input)?;

        utils::command_evaluate(input, &locale, context);

        Ok((input,()))
    }
}

fn parse_rearrange<'a : 'b,'b>(context : &'b Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str,()> + 'b {
    move |input| {
        let (input,locale) = parse_command_syntax("rearrange")(input)?;

        let (input,equation) = take_until(" into")(input)?;
        utils::command_rearrange(&locale, equation.trim(), context, input.trim());

        Ok(("",()))
    }
}

// describe? lang=SYS_LANG? command ..
fn parse_command_syntax<'a>(command : &'a str) -> impl FnMut(&'a str) -> IResult<&'a str,Option<LanguageIdentifier>> {
    move |input| {
        let (input,result) = opt(
            separated_pair(
                // Some means describe none means dont describe
                tag("describe"), 
                multispace0,
                // Some means default lang value means this lang
                opt(
                    map_res(take_until(" "),|s : &str| s.trim().parse::<LanguageIdentifier>())
                )
            )
        )(input)?;
        
        let (input,_) = delimited(
            multispace0,
            tag(command),
            multispace1
        )(input)?;

        // None if describe not there , lang given? no => default to "en-US"
        let lang = result.and_then(|(_,v)| Some(v.unwrap_or("en-US".parse().unwrap())));

        Ok((input,lang))
    }
}