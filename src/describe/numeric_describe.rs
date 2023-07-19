use crate::utils::Numeric;

use super::{FilterLevel,SubStep,Step};

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
    /// ^ to the power of
    Power
}

/// The `DescribeNumeric` trait describes numeric math structs (e.g., decimal fractions, etc.)
pub trait DescribeNumeric<Rhs = Self> : Numeric {
    /// Describe the numeric value as a string representation
    fn describe_numeric(self,filter_level : FilterLevel,operation : NumericOperation,other: Rhs) -> Option<Step>;
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

fn generate_column(array : &mut [f64],operation : &str) -> String {
    array.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    let mut longest_decimal = None;
    for item in &mut *array {
        let string = item.to_string();
        let str_len = string.len() - 1;
        if let Some(index) = string.find('.'){
            let dec_len = str_len - index;
            match longest_decimal {
                None => {
                    longest_decimal = Some(dec_len);
                }
                Some(length) => {
                    if dec_len > length {
                        longest_decimal = Some(dec_len)
                    }
                }
            };
        }
    }
    
    let mut max_length = 0;
    let mut formatter = |n : &f64,w : usize| -> String {
        let string = format!("{:whole$.dec$}",n,whole = w,dec = longest_decimal.unwrap_or(0));
        let len = string.len();
        if len > max_length {
            max_length = len
        };
        string
    };
    
    let last_array_index = array.len() - 1;
    
    let mut vec : Vec<String> =  array[..last_array_index].iter().map(|n| formatter(n,12)).collect();
    let last = &array[last_array_index];
    let last_item = format!("{operation}{}",formatter(last,11));
    vec.push(last_item);
    format!("{}\n{}",vec.join("\n"),"-".repeat(max_length))
}