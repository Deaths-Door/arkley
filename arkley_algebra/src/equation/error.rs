use crate::{Equation, Term};

/// Represents errors that can occur when rearranging an equation.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RearrangeError {
    /// Indicates that unknown variables were found during the rearrangement process.
    /// 
    /// Provides the original equation where unknown variables were discovered.
    UnknownVariablesFound(Equation),

    /// Indicates that the resulting term cannot be converted into the target term
    /// 
    /// Provides the rearranged equation and the target term
    ImpossibleSolution(Equation,Term),
}