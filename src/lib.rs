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

#[doc = include_str!("../arkley_traits/README.md")]
pub mod traits {
    pub use arkley_traits::*;
}
/*

arkley_numerics = { path = "arkley_numerics",optional = true }

[features]
numeric = ["dep:arkley_numerics"]

#[cfg(feature = "numeric")]
#[doc = include_str!("../arkley_numerics/README.md")]
pub mod numeric {
    pub use arkley_numerics::*;
}*/