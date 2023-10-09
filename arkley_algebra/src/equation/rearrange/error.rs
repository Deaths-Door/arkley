use crate::Equation;

/// Represents errors that can occur when rearranging an equation.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RearrangeError {
    /// Indicates that unknown variables were found during the rearrangement process.
    /// 
    /// Provides the original equation where unknown variables were discovered.
    UnknownVariablesFound(Equation),

    /// Indicates that coefficients of the equation are not divisible, preventing
    /// successful rearrangement.
    /// 
    /// Provides the rearranged equation where non-divisible coefficients were detected.
    NonDivisibleCoefficients(Equation),
}
