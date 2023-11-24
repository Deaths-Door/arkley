use std::process::exit;

use arkley_algebra::Context;
use rustyline::DefaultEditor;
use nom::{
    sequence::{preceded, pair}, 
    character::complete::multispace0,
    combinator::map, 
    branch::alt, 
    bytes::complete::tag, IResult
};

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

fn parse_syntax(input : &str,context : &mut Context<'_>) {
    let mut f = alt((
        parse_reserved_commands,
        // TODO : Remove this
        parse_reserved_commands
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

/* 
fn parse_context<'a : 'b,'b : 'l,'l>(context : &'b mut Context<'l>) -> impl FnMut(&'a str) -> IResult<&'a str,()> + 'b {
    move |input| {
        // TODO : Implement this weird af function
        /*match context.parse_tags()(input) {
            res => return res,
            _ => context.parse_tags()(input)
        }
        fold_many1(alt_context,|| (),|acc,_| acc)(input)*/
        Ok((input,()))
    }
}*/