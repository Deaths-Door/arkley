mod commands;
mod playground;
mod pretty_errors;

use clap::Parser;



fn main() {
    let command = commands::Command::parse();

    match command.argument {
        commands::Arguments::Playground => playground::open(),
        commands::Arguments::Evaluate { expression_or_equation, context } => todo!(),
        commands::Arguments::Rearrange { equation, target, context } => todo!(),
        commands::Arguments::Solve { equation, context } => todo!(),
        commands::Arguments::Quadratic { subcommand } => todo!(),
    }
}