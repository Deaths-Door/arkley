mod commands;
mod playground;
mod pretty_errors;

use arkley_describe::fluent_templates;
use clap::Parser;
use commands::{Command,Arguments};

fluent_templates::static_loader! {
    static LOCALES = {
        // For now given this full path
        locales: r"C:\Users\Aarav Aditya Shah\Documents\GitHub\project-codebases\rust\arkley\arkley_algebra\translations",
        fallback_language: "en-US",
    };
}

fn main() {
    let command = Command::parse();

    match command.argument {
        Arguments::Playground => playground::open(),
        Arguments::Evaluate { expression_or_equation, context } 
            if ['=','<','>'].into_iter().any(|c| expression_or_equation.contains(c))=> {
            
            },
        Arguments::Evaluate { expression_or_equation, context } => todo!(),        
        Arguments::Rearrange { equation, target, context } => todo!(),
        Arguments::Solve { equation, context } => todo!(),
        Arguments::Quadratic { subcommand } => todo!(),
    }
}