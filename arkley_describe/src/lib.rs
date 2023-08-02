#![doc = include_str!("../README.md")]

#![forbid(
        missing_docs,
        unsafe_code,
        unused_mut,
        unused_results,
        unused_allocation,
        unused_must_use,
        unreachable_patterns,
        trivial_casts,
        unsafe_op_in_unsafe_fn,
        overflowing_literals,
)]

mod substep;
mod step;
mod filter;
mod describe;
mod utils;

pub use self::substep::*;
pub use self::step::*;
pub use self::filter::*;
pub use self::describe::*;