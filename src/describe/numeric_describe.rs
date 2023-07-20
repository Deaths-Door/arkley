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
        let mut substeps : Vec<SubStep> = Vec::new();

        match operation {
            NumericOperation::Addition => {
                let (_columns,paired) = generate_column(&mut [*self,other],"+");
                let point_index = _columns[0].find('.').unwrap();
                let columns = _columns.join("\n");
                
                let mut carries : Vec<u32> = Vec::new();
                let mut carries_str = String::new();
                
                let mut sum_str = String::new();

                let padding = _columns[_columns.len() - 2].len();

                for (index,list) in paired.iter().enumerate() {
                    if index == point_index {
                        sum_str += ".";
                        continue;
                    };
                    
                    if list.iter().all(Option::is_none) {
                        continue;
                    }

                    let mut info = String::from("Sum : ");

                    let mut c_sum = 0;

                    for item in list {
                        let cn = item.unwrap_or(0);
                        c_sum += cn;
                        info += &format!("{} +",cn);
                    };

                    let carry = carries.last().unwrap_or(&0);
                    c_sum += carry;

                    if carry != &0 {
                        info += &format!("{} (carried forward)",carry);
                    };

                    info += &format!(" = {}",c_sum);

                    let c_rem = c_sum % 10;

                    sum_str += &c_rem.to_string();

                    if c_sum >= 10 {
                        info += &format!("\n Since sum is greater than 10 we carry 1 forward");
                        carries.push(c_rem);
                    };

                    let c_div = c_sum / 10;

                    carries_str.insert_str(0,&format!(" {}",c_div)); 
                
                    let latex = format!("{:>p$}\n{columns}\n{:>p$}",carries_str,sum_str,p = padding);

                    let substep = SubStep::new(info,latex);
                    substeps.push(substep);
                };

                let substep = SubStep::new(B_NUMERIC_OP_ANFANG.to_string(),_columns.join("\n"));
                step.add_substep(substep); 
             //   substeps.reverse();
            },
            NumericOperation::Subtraction => todo!("NOT DONE YET"),
            NumericOperation::Multiplication => todo!("NOT DONE YET"),
            NumericOperation::Division => todo!("NOT DONE YET"),
            NumericOperation::Power => todo!("NOT DONE YET"),
        };
        step.add_substeps(substeps);

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
        let step = 26.5.describe_numeric(FilterLevel::Beginner, NumericOperation::Addition, 3.5).unwrap();
        for substep in &step.0 {
            println!("Info: {}", substep.info);
            println!("Maths:\n{}", substep.latex);
            println!("-----------------------------");
        }
    }
}
