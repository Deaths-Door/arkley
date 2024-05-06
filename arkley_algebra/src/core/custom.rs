use std::hash::{Hash, Hasher};

use dyn_clone::DynClone;
use dyn_eq::DynEq;
use dyn_hash::DynHash;

/// The `CustomizableExpression` trait provides a powerful mechanism for extending the functionality of
/// base expressions defined in the `crate::Expression` type. This allows you to create specialized
/// expression types tailored to your specific domain or use case.
/// By implementing `CustomizableExpression`, concrete expression types gain the ability to:
/// - **Integrate Custom Functions:** Define and incorporate user-defined functions within expressions.
/// - **Incorporate Specialized Mathematical Functions:** Add support for specialized mathematical functions like trigonometric functions (sin, cos, tan) or other domain-specific functions.
/// - **Perform Integration and Differentiation:** Extend the expression type with methods for integration and differentiation operations.
/// - **Implement Additional Features:** The flexibility of this approach allows for the introduction of various other functionalities beyond the listed examples.
/// This trait serves as a contract that concrete expression types can adhere to in order to achieve the desired level of functionality. 
pub trait CustomizableExpression : DynClone + DynEq + DynHash + std::fmt::Debug {    
    /// Negates the expression, and is same as apply [Neg] on `Self`
    fn negate(self : Box<Self>) -> Box<dyn CustomizableExpression>;
}

// For future reference https://users.rust-lang.org/t/trait-object-with-generic-parameter/34101/3
dyn_eq::eq_trait_object!(CustomizableExpression);
dyn_hash::hash_trait_object!(CustomizableExpression);