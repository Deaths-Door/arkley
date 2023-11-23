use arkley_algebra::Context;
use arkley_describe::fluent_templates::LanguageIdentifier;
use clap::{Parser, Subcommand};

use crate::utils::ExpressionOrEquation;

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

    #[clap(about = "Evaluate a mathematical expression")]
    Evaluate {
        expression_or_equation : String,
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
    },

    #[clap(about = "Solve an equation for a specific variable")]
    Solve {
        #[arg(
            short,
            long,
            required = true,
        )]
        equation: String,
    },

    #[clap(about = "Handle quadratic equations")]
    Quadratic {
        #[clap(subcommand)]
        subcommand: QuadraticsCommands,
    },
}

#[derive(Subcommand)]
pub enum QuadraticsCommands {
    #[clap(about = "Calculate discriminant of a quadratic equation")]
    Discriminant {
        input : String
    },

    #[clap(about = "Calculate roots of a quadratic equation")]
    Roots {
        input : String
    },

    #[clap(about = "Calculate sum of roots of a quadratic equation")]
    SumOfRoots {
        input : String
    },

    #[clap(about = "Calculate product of roots of a quadratic equation")]
    ProductOfRoots {
        input : String
    },

    #[clap(about = "Calculate axis of symmetry of a quadratic equation")]
    AxisOfSymmetry {
        input : String
    },

    #[clap(about = "Determine concavity of a quadratic equation")]
    Concavity {
        input : String
    },
}