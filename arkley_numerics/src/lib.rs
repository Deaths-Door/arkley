#![doc = include_str!("../README.md")]

#![forbid(
        missing_docs,
        unsafe_code,
        unused_imports,
        //unused_variables,
        unused_mut,
        unused_results,
        unused_allocation,
        unused_must_use,
        unreachable_patterns,
        trivial_casts,
        unsafe_op_in_unsafe_fn,
        overflowing_literals,
)]

mod fraction;
mod decimal;

#[cfg(feature="describe")]
mod describe;
//mod standardform;

pub use self::fraction::*;
pub use self::decimal::*;
//pub use self::standardform::*;

#[cfg(feature="describe")]
pub use self::describe::*;