#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,
    unsafe_code,
    unused_variables,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    overflowing_literals,
)]


mod expression;
mod term;
mod operation;

pub use self::expression::*;
pub use self::term::*;
pub use self::operation::*;