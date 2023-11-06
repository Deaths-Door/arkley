mod op;
mod term;
mod function;

mod tokens;
mod expression;
mod context;

use nom::{combinator::value, bytes::complete::tag};
pub use term::*;
pub use expression::*;
pub use op::*;
pub use context::*;
pub use function::*;


#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;

fn alternative<'a,T : Clone>(alternatives: &'a std::collections::HashMap<&'a str,T>) -> impl FnMut(&'a str) -> nom::IResult<&'a str,T> {
    move |input| {
        let mut last_err = Err(nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::NonEmpty }));

        for (key,t) in alternatives {
            match value(t,tag(*key))(input) {
                Ok((input,other)) => return Ok((input,(*other).clone())),
                error @ Err(_) => last_err = error,
            }
        }

        last_err.map(|(i,v)| (i,v.clone()))
    }
}