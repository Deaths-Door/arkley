mod command;
mod playground;

use clap::Parser;
use command::{Command,Arguments};

fn main() {
    let command = Command::parse();
    match command.argument {
        Arguments::Playground => playground::open(command.context),
    }
}