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
    fn describe_numeric(&self,filter_level : FilterLevel,operation : NumericOperation,other: Rhs) -> Option<Step>;
}

static B_NUMERIC_OP_ANFANG: &str = "Start from the rightmost column (ones place) and add the digits:";

impl DescribeNumeric for f64 {
    fn describe_numeric(&self,filter_level : FilterLevel,operation : NumericOperation,other: f64) -> Option<Step> {
        if filter_level == FilterLevel::Intermediate {
            return None;
        }

        let mut step = Step::default();

        match operation {
            NumericOperation::Addition => {
                let (_columns,paired) = generate_column(&mut [*self,other],"+");
                let point_index = _columns[0].find('.').unwrap();
                let columns = _columns.join("\n");
                let substep = SubStep::new(B_NUMERIC_OP_ANFANG.to_string(),_columns.join("\n"));
                step.add_substep(substep); 
                
                let mut carries : Vec<u32> = Vec::new();
                let mut carries_str = String::new();

                let mut substeps : Vec<SubStep> = Vec::new();

                for (index,list) in paired.iter().enumerate() {
                    if index == point_index || list.iter().all(Option::is_none) {
                        continue;
                    }
                    let mut c_sum = 0;
                    let numbers : Vec<String> = list.iter().map(|item| {
                        let n = item.unwrap_or(0);
                        c_sum += n;
                        n.to_string()
                    }).collect();
                
    
                    let c_rem = c_sum % 10;
                    carries.push(c_rem);
                    
                    let info = {
                        let optional = match carries.get(carries.len() - 1) {
                            None => String::new(),
                            Some(n) if *n == 0 => String::new(),
                            Some(n) => {
                                format!("+ {} (carried forward)",n)
                            }
                        };
    
                        format!("Sum : {} {} = {}",numbers.join(" + "),optional,c_sum) 
                    };
                    
                    let maths = format!("{:>padding$}\n{}\n{:>padding$}",
                        carries_str,
                        columns,
                        c_rem,
                        padding = numbers[0].len()
                    );

                    carries_str += " ";
                    carries_str += &c_rem.to_string();
                    
                    let substep = SubStep::new(info,maths);
                    substeps.push(substep);
                }

                substeps.reverse();
                step.add_substeps(substeps);
            },
            NumericOperation::Subtraction => todo!("NOT DONE YET"),
            NumericOperation::Multiplication => todo!("NOT DONE YET"),
            NumericOperation::Division => todo!("NOT DONE YET"),
            NumericOperation::Power => todo!("NOT DONE YET"),
        };

        Some(step)
    }
}

fn generate_column(array : &mut [f64],operation : &str) -> (Vec<String>,Vec<Vec<Option<u32>>>) {
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

    let paired = {
        let vec_strings : Vec<String> = vec.iter().map(|s| s.chars().rev().collect()).collect();
        vec_strings[0].chars().enumerate().map(|(index,string)|{
            vec.iter().map(|s|{
                s.chars().nth(index).unwrap().to_digit(10)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    };

    vec.push("-".repeat(max_length));
    (vec,paired)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a test function for the `describe_numeric` method
    #[test]
    fn test_describe_numeric_addition() {
        let step = 2.5.describe_numeric(FilterLevel::Beginner, NumericOperation::Addition, 3.5).unwrap();
        for substep in &step.0 {
            println!("Info: {}", substep.info);
            println!("Maths:\n{}", substep.latex);
            println!("-----------------------------");
        }
    }
}
