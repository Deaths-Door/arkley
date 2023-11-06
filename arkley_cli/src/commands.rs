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
    pub locale : Option<LanguageIdentifier>
}

#[derive(Subcommand)]
pub enum Arguments {
    /// A more 'code' like experience
    #[clap(about = "Interactive math playground")]
    Playground,

    // TODO : Add custom [`arkley_algebra::Context`] for function , tag definition and also allow giving values for them
    #[clap(about = "Evaluate a mathematical expression")]
    Evaluate {
        #[arg(
            short = 'e',
            long = "expression",
            required = true,
        )]
        expression_or_equation : String,

        #[arg(skip)]
        // TODO : Add parsers for it and allow it to be passed via cli
        context : Context<'static>
    },

    #[clap(about = "Rearrange an equation to isolate a variable")]
    Rearrange {
        #[arg(
            short,
            long,
            required = true,
        )]
        /// Parse into `arkley_algebra::Equation` using `TryFrom<&str>`
        equation: String,

        #[arg(
            short,
            long,
            required = true,
        )]
        /// Parse into `arkley_algebra::Term` using `TryFrom<&str>`
        target : String,
        
        #[arg(skip)]
        // TODO : Add parsers for it and allow it to be passed via cli
        context : Context<'static>
    },

    #[clap(about = "Solve an equation for a specific variable")]
    Solve {
        #[arg(
            short,
            long,
            required = true,
        )]
        equation: String,
        
        #[arg(skip)]
        // TODO : Add parsers for it and allow it to be passed via cli
        context : Context<'static>
    },

    #[clap(about = "Handle quadratic equations")]
    Quadratic {
        #[clap(subcommand)]
        subcommand: QuadraticsCommands,
    },
}

#[derive(Subcommand)]
pub enum QuadraticsCommands {

}