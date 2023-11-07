#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,
    missing_debug_implementations,

    unsafe_code,
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

pub use core::*;
pub use arithmetics::*;


/// Module containing manipulations for algebra
pub mod manipulation;

/// Module containing `quadratics` for both algebra and numerics
pub mod quadratics;

#[cfg(feature="parse")]
mod parser;

#[cfg(feature="parse")]
pub use parser::*;

#[cfg(feature="equation")]
mod equation;

#[cfg(feature="equation")]
pub use equation::*;