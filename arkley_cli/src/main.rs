mod command;
mod playground;
mod utils;

use clap::Parser;
use command::{Command,Arguments};

fn main() {
    let command = Command::parse();
    match command.argument {
        Arguments::Playground => playground::open(command.context),
        Arguments::Quadratic { subcommand  , input , named } 
            => subcommand.handle_subcommands(input,named,&command.context,command.locale),
    }
}