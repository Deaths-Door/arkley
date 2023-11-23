mod commands;
mod playground;
mod pretty_errors;
mod utils;

use clap::Parser;
use commands::{Command,Arguments};

fn main() {
    let command = Command::parse();
    match command.argument {
        Arguments::Playground => playground::open(command.context),
        Arguments::Evaluate { expression_or_equation } 
            => Command::command_evaluate(&command.locale,&expression_or_equation, &command.context),        
        Arguments::Rearrange { equation, target } 
            => Command::command_rearrange(command.locale, &equation, &command.context, &target),
        Arguments::Solve { equation } => todo!(),
        Arguments::Quadratic { subcommand } => subcommand.handle(&command.context,&command.locale),
    }
}