use crate::SubStep;

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
    let (c_aligned, padding) = align_numbers(array, "*");
    let column = c_aligned.join("\n");

    let substeps: Vec<SubStep> = Vec::new();

    for (index,outer_ch) in c_aligned.iter().rev().enumerate() {
         
        for ch in &c_aligned[index + 1..] {
            println!("{ch}");
        }
    }

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
}