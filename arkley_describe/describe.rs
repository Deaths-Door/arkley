use crate::{FilterLevel,DescribeOperation,Step};

/// Represents a generic trait for describing operations.
/// The associated type `Output` specifies the return type of the `describe` method.
pub trait Describe<Rhs = Self> : Sized {
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
    fn describe(self,other : Rhs,filter_level : Option<FilterLevel>,operation: DescribeOperation) -> Option<Self::Output>;
}

impl Describe<f64> for f64 {
    type Output = Step;

    fn describe(self,other : f64,filter_level : Option<FilterLevel>,operation: DescribeOperation) -> Option<Self::Output> {
        match filter_level.map(|level| level > FilterLevel::Intermediate).unwrap_or(true) {
            false => None,
            true => {
                let (a,b) = if self >= other { (self, other) } else { (other, self) };
                let step = match operation {
                    DescribeOperation::Multiplication => crate::describe_mul_f64(a,b),
                    DescribeOperation::Division => todo!("NOT DONE YET"),
                    DescribeOperation::Addition if a.is_positive() && b.is_positive() => crate::describe_add_f64(a,b),
                    DescribeOperation::Subtraction if a.is_negative() && b.is_negative() => crate::describe_add_f64(a,b),
                    DescribeOperation::Addition | DescribeOperation::Subtraction => todo!("describe substraction"),
                    _ => todo!("...")
                };

                Some(step)
            }
        }
    }
}

macro_rules! impl_describe_for_integers {
    ($($t:ty),*) => {
        $(
            impl Describe<$t> for $t {
                type Output = Step;
    
                fn describe(
                    self,
                    other: $t,
                    filter_level: Option<FilterLevel>,
                    operation: DescribeOperation,
                ) -> Option<Self::Output> {
                    (self as f64).describe(other as f64,filter_level,operation)
                }
            }
        )*
    };
}

impl_describe_for_integers!(u8,u16,u32,u64,i8,i16,i32,i64,f32);