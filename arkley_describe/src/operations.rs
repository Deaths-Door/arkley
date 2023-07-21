/// An enumeration representing different numeric operations for description purposes.
///
/// This enum defines various arithmetic and algebraic operations that can be performed on numeric values.
/// Each variant corresponds to a specific operation:
/// - `Addition`: Represents the addition operation (+).
/// - `Subtraction`: Represents the subtraction operation (-).
/// - `Multiplication`: Represents the multiplication operation (*).
/// - `Division`: Represents the division operation (/).
/// - `Exponentiation`: Represents the exponentiation operation (^).
///
/// The `Operation` enum is used in combination with the `Describe` trait to describe mathematical
/// operations in a human-readable format. It allows you to generate step-by-step explanations
/// of arithmetic and algebraic expressions, including different operations.
/// 
/// The `Operation` enum can be used as an argument to the `describe` method of types that implement
/// the `Describe` trait. This enables generating human-readable descriptions of mathematical expressions,
/// making it useful for educational purposes, debugging, or any scenario where a step-by-step
/// explanation of numeric operations is required.
#[derive(PartialEq)]
pub enum Operation {
    /// Represents the addition operation (+).
    Addition,
    /// Represents the subtraction operation (-).
    Subtraction,
    /// Represents the multiplication operation (*).
    Multiplication,
    /// Represents the division operation (/).
    Division,
    /// Represents the exponentiation operation (^).
    Exponentiation,
}