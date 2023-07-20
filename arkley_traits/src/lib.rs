#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,
    unsafe_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unused_results,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    overflowing_literals,
)]

mod gcd;
mod lcm;
mod numeric;
mod abs;
mod power;
mod zero;
mod log;

pub use self::gcd::*;
pub use self::lcm::*;
pub use self::numeric::*;
pub use self::abs::*;
pub use self::power::*;
pub use self::zero::*;
pub use self::log::*;
