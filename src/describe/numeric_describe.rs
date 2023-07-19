use crate::utils::Numeric;

use super::{FilterLevel,SubStep,Step};

// An enumeration representing different numeric operations
#[derive(PartialEq, PartialOrd)]
pub enum NumericOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power
}

// The `DescribeNumeric` trait extends the `Numeric` trait
pub trait DescribeNumeric<Rhs = Self> : Numeric {
    fn describe_numeric(&self,filter_level : Option<FilterLevel>,operation : NumericOperation,other: Rhs) -> Option<Step>;
}

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