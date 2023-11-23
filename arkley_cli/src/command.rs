use arkley_algebra::Context;
use arkley_describe::fluent_templates::LanguageIdentifier;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Command {
    /// For which 'sub_command' to use
    #[clap(subcommand)]
    pub argument : Arguments,

    /// Whether should `describe` command in given locale
    #[arg(skip = None)]
    pub locale : Option<LanguageIdentifier>,

    // Add it be be able to be loaded from a file
    #[arg(skip)]
    // TODO : Add parsers for it and allow it to be passed via cli
    pub context : Context<'static>
}

#[derive(Subcommand)]
pub enum Arguments {
    /// A more 'code' like experience
    #[clap(about = "Interactive math playground")]
    Playground,
}