#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,
    missing_debug_implementations,

    unsafe_code,
    unused_variables,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,

    unstable_features,
    unsafe_op_in_unsafe_fn,

    trivial_casts,
    overflowing_literals,
    non_snake_case
)]

mod core;
mod arithmetics;

/// Module contains manipulations for algebra
pub mod manipulation;

pub use core::*;
pub use arithmetics::*;

#[cfg(feature="parse")]
mod parser;

#[cfg(feature="parse")]
pub use parser::*;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;