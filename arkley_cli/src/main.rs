mod commands;
mod playground;
mod pretty_errors;
mod utils;

use clap::Parser;
use commands::{Command,Arguments};

fn main() {
    let command = Command::parse();

    match command.argument {
        Arguments::Playground => playground::open(),
        Arguments::Evaluate { expression_or_equation, context } 
            => Command::command_evaluate(command.locale,&expression_or_equation, context),        
        Arguments::Rearrange { equation, target, context } 
            => Command::command_rearrange(command.locale, &equation, context, &target),
        Arguments::Solve { equation, context } => todo!(),
        Arguments::Quadratic { subcommand } => todo!(),
    }
}