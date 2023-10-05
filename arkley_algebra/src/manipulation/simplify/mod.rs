use crate::{Expression, ArithmeticOperation};

mod expression;

pub use expression::*;

/// A trait for types that can be simplified.
///
/// This trait defines a `simplify` method that allows implementing types to provide
/// custom logic for simplifying instances of themselves. The `simplify` method should
/// return a simplified result of the implementing type.
///
/// The `Output` associated type specifies the return type of the `simplify` method.
/// By default, it is set to `Self`, indicating that if an implementing type doesn't
/// specify a different return type, it will return an instance of itself.
pub trait Simplify<Output = Self> {
    /// Simplify the implementing type.
    ///
    /// This method should return a simplified instance of the implementing type. 
    /// 
    /// *Note* : This method is for simplifying the structure for example by removing unneccessary parentheses
    ///
    /// # Returns
    ///
    /// The simplified result of the implementing type.
    fn simplify_structure(self) -> Output;
}