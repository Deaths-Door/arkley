use thiserror::Error;

use crate::{Equation, Term};

/// Represents errors that can occur when rearranging an equation.
#[derive(Debug,Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RearrangeError {
    /// Indicates that unknown variables were found during the rearrangement process.
    /// 
    /// Provides the original equation where unknown variables were discovered.
    #[error("Given variable was not found in {}",.0)]
    UnknownVariablesFound(Equation),

    /// Indicates that the resulting term cannot be converted into the target term
    /// 
    /// Provides the rearranged equation and the target term
    #[error("Resulting equation {} can not be rearranged into {}",.0,.1)]
   
    ImpossibleSolution(Equation,Term),
}