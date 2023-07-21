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

mod substep;
mod step;
mod method;
mod filter;

pub use self::substep::*;
pub use self::step::*;
pub use self::method::*;
pub use self::filter::*;

use std::fmt::Arguments;

/// Represents a generic trait for describing operations.
/// The associated type `Output` specifies the return type of the `describe` method.
pub trait Describe<Rhs = Self> {
    /// The output type returned by the `describe` method.
    /// By default, it is `Step`, but implementations can override it.
    type Output;
    /// Describes the operation between the current instance and the right-hand side `Rhs`,
    /// with optional additional arguments.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to the object on which the method is called.
    /// - `other`: The right-hand side argument of the numeric operation.
    /// - `additional_args`: An optional `Arguments` type representing any number of additional
    ///   arguments that can be passed during the description. This allows for flexibility in
    ///   handling additional arguments of different types and quantities.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the operation as a `Step`.
    /// If the operation can be described successfully or is described at all, the method returns `Some(step)`,
    /// otherwise, it returns `None`
    fn describe(&self,other : Rhs,additional_args: Option<Arguments>) -> Option<Self::Output>;
}