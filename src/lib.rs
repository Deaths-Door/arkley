#![doc = include_str!("../README.md")]

#![forbid(
        missing_docs,
    
        unsafe_code,
        
       // unused_imports,
       // unused_variables,
        unused_mut,
        unused_results,
        unused_allocation,
        unused_must_use,
    
        unreachable_patterns,
    
        trivial_casts,
    
        unsafe_op_in_unsafe_fn,
        
        overflowing_literals,
)]

/// `Utilities` module for common mathematical operations.
///
/// This module provides several traits and utility functions for common mathematical operations,
/// such as calculating the least common multiple (LCM) and greatest common divisor (GCD) of values.
/// These utilities can be used across different parts of the calculator to perform various
/// mathematical calculations.
pub mod utils;

/// `Numbers` module provides various number representations and calculations.
///
/// This module provides several structs and enums that represent common mathematical numbers such as fractions, standard form, and decimals,
pub mod numbers;

/// The `describe` module provides functionalities for describing operations and filtering levels.
///
/// This module contains several components, including the `FilterLevel` enum that represents different
/// levels of filtering for numeric descriptions. It also includes the `SubStep` and `Step` structs, which
/// are used to construct detailed descriptions of numeric operations.
pub mod describe;