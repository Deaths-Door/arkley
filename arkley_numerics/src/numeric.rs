//use std::ops::{Add,Sub,Mul,Div,Neg};

//use arkley_traits::{Abs,Lcm,Power,Zero,Log};

use arkley_describe::{
    Describe,
    Step,
    FilterLevel
};

/// An enumeration representing different numeric operations
#[derive(PartialEq)]
pub enum NumericOperation {
    /// +
    Addition,
    /// -
    Subtraction,
    /// *
    Multiplication,
    /// /
    Division,
}

/// A wrapper trait that combines the numeric operations and description functionality.
pub trait Numeric<Rhs = Self> : Describe<NumericOperation,Output = Step> {
    /// Describes the numeric operation and generates a step-by-step explanation.
    ///
    /// This method takes another numeric value `other`, an optional `filter_level` to control
    /// the level of detail in the description, and the `operation` to be performed.
    ///
    /// # Parameters
    ///
    /// - `other`: Another numeric value of the same type to perform the operation with.
    /// - `filter_level`: An optional `FilterLevel` to control the level of detail in the description.
    /// - `operation`: The `NumericOperation` to be performed, such as addition, subtraction, etc.
    ///
    /// # Returns
    ///
    /// An `Option<Step>` containing the step-by-step description of the numeric operation.
    ///
    fn describe_numeric(&self,other : Self,filter_level : Option<FilterLevel>,operation : NumericOperation) -> Option<Step> {
        self.describe(other,filter_level,operation)
    }
}
/*
macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {}
        )*
    };
}

impl_numeric!(i8, i16, i32, i64,f32,f64);*/