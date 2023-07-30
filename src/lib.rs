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

/// # Arkley Traits
///
/// This module provides traits for Arkley traits crate.
/// For more information, refer to the [README.md](https://docs.rs/arkley_traits).
pub mod traits {
    pub use arkley_traits::*;
}

/// # Arkley Numerics
///
/// This module provides numbers for Arkley traits crate.
/// For more information, refer to the [README.md](https://docs.rs/arkley_numerics).
pub mod numerics {
    pub use arkley_numerics::*;
}