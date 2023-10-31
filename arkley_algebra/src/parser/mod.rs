mod op;
mod term;
mod function;

mod tokens;
mod expression;
mod context;

pub use term::*;
pub use expression::*;
pub use op::*;
pub use context::*;
pub use function::*;


#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;

fn alternative<'a,T>(alternatives: &'a std::collections::HashMap<&'a str,fn() -> T>) -> impl FnMut(&'a str) -> nom::IResult<&'a str,T> {
    move |input| {
        let mut last_err = Err(nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::NonEmpty }));

        for (key,closure) in alternatives {
            // Same as : value(closure(),tag(*key))(input)
            match nom::bytes::complete::tag(*key)(input).map(|(i, _)| (i, closure())) { 
                ok @ Ok(_) => return ok,        
                error @ Err(_) => last_err = error 
            }
        }

        last_err
    }
}