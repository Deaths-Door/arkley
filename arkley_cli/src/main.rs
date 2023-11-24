mod command;
mod playground;
mod utils;

use clap::Parser;
use command::{Command,Arguments};

fn main() {
    utils::CURRENT_EXE_DIR.get_or_init(|| std::env::current_exe().expect("Unable to get current path"));

    let command = Command::parse();
    
    match command.argument {
        Arguments::Playground => playground::open(command.context),
        Arguments::Quadratic { subcommand  , input , named } 
            => subcommand.handle_subcommands(input,named,&command.context,command.locale),
        Arguments::Evaluate { expr_eq } => utils::evaluate_handler(expr_eq, &command.locale,&command.context),
        Arguments::Rearrange { equation, target } => utils::command_rearrange(&command.locale, &equation, &command.context, &target),
        Arguments::Solve { equation } => todo!(),
    }
}