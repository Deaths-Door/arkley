use std::process::exit;

use nom::{
    sequence::{preceded, delimited}, 
    character::complete::multispace0,
    combinator::map, 
    branch::alt, 
    bytes::complete::tag, IResult
};

use crate::pretty_errors::new_default_editor;


pub fn open() {
    let mut rl = new_default_editor();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match parse_syntax(&line) {
                Err(err) => eprintln!("{err}"),
                _ => ()
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }
}

fn parse_syntax(input : &str) -> Result<(),String> {
    alt((
        parse_reserved_commands,
        map(tag("todo"),|_| ()) // place holder for now
    ))(input)
    .map(|_| Ok(()))
    .map_err(|_| String::from("Please provide valid syntax"))?
}

/// Parses things like "--command" w/o whitespace
fn parse_reserved_commands(input : &str) -> IResult<&str,()> {
    const DOC_MSG : &str = r#"
For detailed information about how to use about the playground, please refer to our documentation:
                
https://github.com/Deaths-Door/arkley/tree/main/arkley_cli/README.md"#;
    
    preceded(
        delimited(
            multispace0, 
            tag("--"), 
            multispace0),
        alt((
            map(tag("help"),|_| println!("{DOC_MSG}")),
            map(tag("exit"),|_| exit(0))
        ))
    )(input)        
}