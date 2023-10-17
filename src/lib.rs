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

#[cfg(feature="algebra")]
pub use arkley_algebra::*;

#[cfg(feature="units")]
pub use arkley_unit_convertor::*;

#[cfg(feature="describe")]
pub use arkley_unit_convertor::*;