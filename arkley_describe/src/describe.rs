use crate::FilterLevel;

/// Represents a generic trait for describing operations.
/// The associated type `Output` specifies the return type of the `describe` method.
pub trait Describe<T,Rhs = Self> : Sized {
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
    fn describe(&self,other : Rhs,filter_level : Option<FilterLevel>,operation: NumericOperation) -> Option<Self::Output>;
}

impl Describe<f64> for f64 {
    type Output = Step;

    fn describe(&self,other : f64,filter_level : Option<FilterLevel>,operation: NumericOperation) -> Option<Self::Output> {
        match filter_level.map(|level| level > FilterLevel::Intermediate).unwrap_or(true) {
            false => None,
            true => {
                match operation {
                    NumericOperation::Multiplication => todo!("NOT DONE YET"),
                    NumericOperation::Division => todo!("NOT DONE YET"),
                    _ => {

                    }
                }
            }
        }
    }
}