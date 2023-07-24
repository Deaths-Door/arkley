use crate::{DescribeOperation,SubStep,Step};

pub(crate) struct DescribeOperationWithIntergers;

impl DescribeOperationWithIntergers {
    pub(crate) fn new(operation: DescribeOperation,x : f64,y : f64) -> Step {
        let (a,b) = if x >= y { (x, y) } else { (y, x) };
        match operation {
            DescribeOperation::Multiplication => Self::describe_mul_f64(a,b),
            DescribeOperation::Division => todo!("NOT DONE YET"),
            _ => todo!("...")
        }
    }

    fn describe_mul_f64(x : f64,y : f64) -> Step {
        const BASE : u32 = 10;

        let mut step = Step::new("lets start (TODO GIVE BETTER STARTING INSTRUCTION)".to_string());

        let (c_aligned,padding,longest_decimal) = Self::align(x,y,"*");

        let mut c_aligned_updated = c_aligned.join("\n");

        let x_space_index = c_aligned[0].rfind(' ').unwrap();
        let y_space_index = c_aligned[0].rfind(' ').unwrap();

        // So only valid numbers are there in loop
        let x_str = &c_aligned[0][x_space_index + 1..];
        let y_str = &c_aligned[1][y_space_index + 1..];

        // to take into account decimal points in numbers for the factor scaling so 10 to the power of index - (encounter as i32)
      //  let mut x_dec_encounted = false;
       // let mut y_dec_encounted = false;

        //multiplication
        for (y_index,ch_y) in y_str.chars().rev().enumerate() {
            if ch_y == '.' {
                continue;
            }

            let _yd = ch_y.to_digit(BASE).unwrap_or(0);

            if _yd == 0 {
                if y_index == y_str.len() - 1 {
                    continue;    
                }
                
                let format = format!("Now we can skip multiplying {x_str} with 0 as {x_str} * 0 = 0");
                let substep = SubStep::new(format);
                step.add_substep(substep);
                continue;
            }

            let y_factor = 10_i32.pow(y_index as u32);

            let y_digit = _yd * (y_factor as u32);

            for (x_index,ch_x) in x_str.chars().rev().enumerate() {
                if ch_x == '.' {
                    continue;
                }
                
                let _xd = ch_x.to_digit(BASE).unwrap();

                if _xd == 0 {
                    let format = format!("Now we can skip multiplying 0 with {y_str} as 0 * {y_str} = 0");
                    let substep = SubStep::new(format);
                    step.add_substep(substep);
                    continue;
                }

                let x_factor = 10_i32.pow(x_index as u32);
            
                let x_digit = _xd * x_factor as u32;

                let product = y_digit * x_digit;

                let info = format!("Multiply {y_digit} * {x_digit} which is {product}\nNow write the product down below");

                c_aligned_updated = format!("{}\n{:width$.dec$}",c_aligned_updated.as_str(),product,width = padding,dec = longest_decimal);
                
                let latex = Some(c_aligned_updated.clone());
                let substep : SubStep = SubStep::new_with_latex(info,latex);
                step.add_substep(substep);
            }
        }

        let seperator = "-".repeat(padding);
        let last_dash_index = c_aligned_updated.rfind('-').unwrap();
        let mut addition_numbers : Vec<_> = c_aligned_updated[last_dash_index + 2..].lines().rev().collect();
        
        addition_numbers.push(&seperator);
        let substeps = Self::describe_add_numbers(&addition_numbers,padding,longest_decimal);
        step.add_substeps(substeps);
        
        step
    }

    fn describe_add_numbers(c_aligned : &Vec<&str>,padding : usize,longest_decimal : usize) -> Vec<SubStep> {        
        let c_pairs = Self::into_num_pairs(&c_aligned);
        let column = c_aligned.join("\n");

        let mut previous_carry = 0;
    
        let mut carry_str = String::from("0");
        let mut sum_str = String::new();
    
        let mut substeps : Vec<SubStep> = Vec::new();

        for pair in c_pairs.iter().rev() {
            if pair.iter().all(Option::is_none) {
                continue;
            }

            let mut c_sum = previous_carry;
    
            let mut info = String::from("Sum : ");
    
            for &num in pair.iter().flatten() {
                c_sum += num;
                info += &format!("{num} + ");
            }
    
            if previous_carry != 0 {
                info += &format!("{} (carried forward)",previous_carry);
            }
    
            previous_carry = c_sum / 10;
    
            info += &format!(" = {}",c_sum);
    
            if c_sum >= 10 {
                info += &format!("\nSince sum is greater than 10 we carry 1 forward");
            }
            else if c_sum == 0{
                info += &format!("\nSince sum is less then 10 we dont carry forward anything")
            }
    
            let c_rem = c_sum % 10;
    
            sum_str.insert_str(0,&c_rem.to_string());
    
            carry_str.insert_str(0,&previous_carry.to_string());
    
            let _temp_carry_str = match carry_str.parse::<u8>().unwrap() {
                0 => "",
                _ => &carry_str
            };//carry_str.parse::<u64>().unwrap();

            let latex = format!("{:>p$}\n{column}\n---------\n{:>p$}",_temp_carry_str,sum_str,p = padding);
    
            let mut substep = SubStep::new(info);
            substep.set_latex(latex);
            substeps.push(substep);
        }
    
        substeps
    }

    
    fn align(x : f64,y : f64,op_str : &str) -> (Vec<String>,usize,usize) {
        let x_str = x.to_string();
        let y_str = y.to_string();

        // For some reason + 6 results into the perfect alignment
        let padding = x_str.len().max(y_str.len()) + 6;

        let mut longest_decimal : usize = 0 ;

        let is_x_int = (x as i64) as f64 == x;
        let is_y_int = (y as i64) as f64 == y;

        let mut closure = |string : &str|{
            let index = string.find('.').unwrap();
            let dec_len = string.len() - index - 1;

            if dec_len > longest_decimal {
                longest_decimal = dec_len
            };
        };

        match (is_x_int,is_y_int) {
            // both are whole numbers so nothing
            (true,true) => { },
            // y is f64
            (true,false) => closure(&y_str),
            // x is 64
            (false,true) => closure(&x_str),
            // neither are whole numbers
            (false,false) => {
                closure(&y_str);
                closure(&x_str);
            }
        };

        let padded_x = format!("{:width$.dec$}", x, width = padding,dec = longest_decimal);
       
        // -2 to take into account offset by adding op_str and whitespace
        let padded_y = format!("{} {:width$.dec$}",op_str,y,width = padding - 2,dec = longest_decimal);

        let seperator = "-".repeat(padding);

        return (vec![padded_x,padded_y,seperator],padding,longest_decimal);
    }

    fn into_num_pairs(vec :&Vec<&str>) -> Vec<Vec<Option<u32>>> {
        vec[0].chars().enumerate().map(|(index,_)|{
            vec.iter().map(|item|{
                    item.chars().nth(index).unwrap().to_digit(10)
                }).collect::<Vec<_>>()
        })
        .collect()
    }  
}

#[cfg(test)]
mod test {
    use super::*;

    use std::panic;
    #[test]
    fn init_mul_unsigned_float() {
        let vec = DescribeOperationWithIntergers::new(DescribeOperation::Multiplication,345.0,40.0);
            for substep in vec.substeps() {
                println!("Info = {}",substep.information());
                println!("Latex = \n{}",substep.latex().clone().unwrap_or("NO latex".to_string()));
            }
        
    }
}


/*
fn align_numbers(array : &mut [f64],op_str : &str) -> (Vec<String>,usize) {
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

    let mut vec : Vec<String> = array.iter().map(|item|{
        // For some reason whole + 6 results into the perfect alignment
        format!("{:whole$.dec$}",item,whole = longest_whole + 6,dec = longest_decimal)
    }).collect();
    let last = vec.last_mut().unwrap();
    last.replace_range(0..1,op_str);

    let paddin = vec[0].len();
    (vec,paddin)
}

fn pair_numbers(vec : &Vec<String>) -> Vec<Vec<Option<u32>>> {
    let strings : Vec<String> = vec.iter().map(|item| item.chars().rev().collect()).collect();
    strings[0].chars().enumerate().map(|(index,_)|{
        vec.iter().map(|item|{
            item.chars().nth(index).unwrap().to_digit(10)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

pub(super) fn describe_unsigned_float_addition(array : &mut [f64]) -> Vec<SubStep> {
    let (c_aligned,padding) = align_numbers(array,"+");
    let c_pairs = pair_numbers(&c_aligned);

    let column = c_aligned.join("\n");

    let mut previous_carry = 0;

    let mut carry_str = String::from("0");
    let mut sum_str = String::new();

    let mut substeps : Vec<SubStep> = Vec::new();
    
    for (index,vec_item) in c_pairs.iter().rev().enumerate() {
        if vec_item.iter().all(Option::is_none) {
            continue;
        };
          
        let mut c_sum = previous_carry;

        let mut info = String::from("Sum : ");

        for &num in vec_item.iter().flatten() {
            c_sum += num;
            info += &format!("{num} + ");
        }

        if previous_carry != 0 {
            info += &format!("{} (carried forward)",previous_carry);
        }

        previous_carry = c_sum / 10;

        info += &format!(" = {}",c_sum);

        if c_sum >= 10 {
            info += &format!("\nSince sum is greater than 10 we carry 1 forward");
        }

        let c_rem = c_sum % 10;

        sum_str.insert_str(0,&c_rem.to_string());

        carry_str.insert_str(0,&previous_carry.to_string());

        let latex = format!("{:>p$}\n{column}\n---------\n{:>p$}",carry_str,sum_str,p = padding);

        
        substeps.push(SubStep::new(info,latex));
    }                                 

    substeps
}

pub(super) fn describe_unsigned_float_multiplication(array: &mut [f64]) -> Vec<SubStep> {
    let substeps: Vec<SubStep> = Vec::new();

 /*   let (c_aligned, padding) = align_numbers(array, "*");
    let column = c_aligned.join("\n");
*/

    /*for (index,outer_ch) in c_aligned.iter().rev().enumerate() {
         
        for ch in &c_aligned[index + 1..] {
            println!("{ch}");
        }
    }*/

    substeps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_add_unsigned_float() {
        let mut array = [1.0,9.5,4.8];
        let vec = describe_unsigned_float_addition(&mut array);
        println!("{:#?}",vec);

        for substep in vec {
            println!("Info = {}",substep.information());
            println!("Latex = \n{}",substep.latex());
        }
    }

    #[test]
    fn init_mul_unsigned_float() {
        let mut array = [34.0,40.0];
        let vec = describe_unsigned_float_multiplication(&mut array);
        println!("{:#?}",vec);

        for substep in vec {
            println!("Info = {}",substep.information());
            println!("Latex = \n{}",substep.latex());
        }
    }
}*/