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

#[cfg(all(feature="default",feature = "build"))]
compile_error!("The 'default' feature, which is the crate itself, and the 'build' feature, which should be used at compile-time to generate resources, cannot be enabled simultaneously.");

#[cfg(feature="build")]
mod build;

#[cfg(feature="build")]
pub use build::*;

#[cfg(feature="default")]
mod describe;

#[cfg(feature="default")]
pub use describe::*;

#[cfg(feature="default")]
mod filter;

#[cfg(feature="default")]
pub use filter::*;

#[cfg(feature="default")]
pub use fluent_templates;
