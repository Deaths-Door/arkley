//TODO : Add from str for equation

// TODO : Make the parsing functions more generic so it can make my life easier for improving the explainations for errors like too little args , too many args etc unbalanced expression etc
mod tokens;
mod op;
mod term;
mod expression;
mod context;

pub use term::*;
pub use expression::*;
pub use op::*;
pub use context::*;


#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;