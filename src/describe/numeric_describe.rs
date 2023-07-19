use crate::utils::Numeric;

use super::{FilterLevel,SubStep,Step};

/// An enumeration representing different numeric operations
#[derive(PartialEq, PartialOrd)]
pub enum NumericOperation {
    /// +
    Addition,
    /// -
    Subtraction,
    /// *
    Multiplication,
    /// /
    Division,
    /// ^ to the power of
    Power
}

/// The `DescribeNumeric` trait describes numeric math structs (e.g., decimal fractions, etc.)
pub trait DescribeNumeric<Rhs = Self> : Numeric {
    /// Describe the numeric value as a string representation
    fn describe_numeric(&self,filter_level : Option<FilterLevel>,operation : NumericOperation,other: Rhs) -> Option<Step>;
}

/*
impl DescribeNumeric for f64 {
    fn describe_numeric(&self,filter_level : Option<FilterLevel>,operation : NumericOperation,other: f64) -> Option<Step> {
        match filter_level {
            None | Some(level) if level >= FilterLevel::Intermediate =>  {
                todo!("")
            }
            _ => {
                todo!("")
            }
        }
    }
}*/
/*
impl DescribeNumeric for i64 {
    const IGNORE_LEVEL : FilterLevel = FilterLevel::Intermediate;
    fn describe_numeric(&self,filter_level : Option<FilterLevel>,operation : NumericOperation,other: i64) -> Option<Step> {
        None/*match filter_level {
            None | (Some(level) if level <= Self::IGNORE_LEVEL) => {
                let step = match operation {
                    NumericOperation::Addition => {
                        todo!("")
                    },
                    NumericOperation::Subtraction => todo!("NOT DONE YET"),
                    NumericOperation::Multiplication => todo!("NOT DONE YET"),
                    NumericOperation::Division => todo!("NOT DONE YET"),
                    NumericOperation::Power => todo!("NOT DONE YET"),
                };
        
                Some(step)
            },
            _ => None
        }*/
        /*match filter_level {
            None => None,
            Some(level) => {
                if level <= Self::IGNORE_AT_FILTER_LEVEL {
                    return None;
                }

                let step = match operation {
                    NumericOperation::Addition => {
                        
                    },
                    NumericOperation::Subtraction => todo!("NOT DONE YET"),
                    NumericOperation::Multiplication => todo!("NOT DONE YET"),
                    NumericOperation::Division => todo!("NOT DONE YET"),
                    NumericOperation::Power => todo!("NOT DONE YET"),
                }
        
                Some(step)
            }
        }*/
    }

}*/