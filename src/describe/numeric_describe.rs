//use crate::Numeric;

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
pub trait DescribeNumeric<Rhs = Self> : /*Numeric*/ {
    /// Describe the numeric value as a string representation
    fn describe_numeric(&self,filter_level : FilterLevel,operation : NumericOperation,other: &Rhs) -> Option<Step>;
}

impl DescribeNumeric for f64 {
    fn describe_numeric(&self,filter_level : FilterLevel,operation : NumericOperation,other: &f64) -> Option<Step> {
        if filter_level == FilterLevel::Intermediate {
            return None;
        }

        let mut step = Step::default();
        let mut substeps : Vec<SubStep> = Vec::new();

        match operation {
            NumericOperation::Multiplication => todo!("NOT DONE YET"),
            NumericOperation::Division => todo!("NOT DONE YET"),
            NumericOperation::Power => todo!("NOT DONE YET"), 
            //check if self and other is - or + and then do operation
            _ => {
                match self.is_positive() && other.is_positive() {
                    // Addition
                    true => {
                        let c_aligned = align_numbers(&mut [self,other]);
                        let c_pairs = pair_numbers(&c_aligned);
                                                    
                        let mut s_str = String::from("");
                    
                        for (index,vec) in c_pairs.iter().enumerate() {
                        
                            if vec.iter().all(Option::is_none) {
                                continue;
                            };
                    
                            let mut c_sum = 0;
                            
                            for n in vec {
                                let cn = n.unwrap_or(0);
                                c_sum += cn;
                            };2
        
        
                        }
                    },
                    // Subtraction
                    false => todo!(".."),
                }
            }
            NumericOperation::Addition => {
                /*let (_columns,paired) = generate_column(&mut [*self,other],"+");
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
             //   substeps.reverse();*/
            },
        };
        step.add_substeps(substeps);

        Some(step)
    }
}

fn align_numbers(array : &mut [f64]) -> Vec<String>{
    array.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut longest_decimal = 0;
    let mut longest_whole = 1;
   
    for item in &mut *array {
        let string = item.to_string();
        let len = string.len() - 1;

        let whole_len : usize;

        match string.find('.') {
            None => {
                whole_len = len;
            }
            Some(index) => {
                let dec_len = len - index;
                
                whole_len = len - dec_len;

                if dec_len > longest_decimal {
                    longest_decimal = dec_len
                };
            }
        }

        if whole_len > longest_whole {
            longest_whole = whole_len;
        };
    }

    array.iter().map(|item|{
        // For some reason whole + 6 results into the perfect alignment
        format!("{:whole$.dec$}",item,whole = longest_whole + 6,dec = longest_decimal)
    }).collect()
}

type CPairs = Vec<Vec<Option<u32>>>

fn pair_numbers(vec : &Vec<String>) -> CPairs {
    let strings : Vec<String> = vec.iter().map(|item| item.chars().rev().collect()).collect();
    strings[0].chars().enumerate().map(|(index,_)|{
        vec.iter().map(|item|{
            item.chars().nth(index).unwrap().to_digit(10)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}
/*
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
}*/