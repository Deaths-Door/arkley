use crate::{DescribeOperation,SubStep,Step};

pub(super) fn describe_add_f64(x : f64,y : f64) -> Step {
    let (_c_aligned,padding,_) = align(x,y,"+");
    let c_aligned = _c_aligned.iter().map(|s| s.as_str()).collect();
    let substeps = describe_add_numbers(&c_aligned,padding);
    let mut step = Step::new("Start from left to right".to_string());
    step.add_substeps(substeps);
    step
}

fn describe_add_numbers(c_aligned : &Vec<&str>,padding : usize) -> Vec<SubStep> {        
    let c_pairs = into_num_pairs(&c_aligned);
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

        if c_sum == 0 {
            info += &format!("\nSince sum is 0 we can skip this column")
        }

        if previous_carry != 0 {
            info += &format!("{} (carried forward)",previous_carry);
        }

        previous_carry = c_sum / 10;

        info += &format!(" = {}",c_sum);

        if c_sum >= 10 {
            info += &format!("\nSince sum is greater than 10 we carry 1 forward");
        }
        /*else if c_sum == 0{
            info += &format!("\nSince sum is less then 10 we dont carry forward anything")
        }*/

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

pub fn into_num_pairs(vec :&Vec<&str>) -> Vec<Vec<Option<u32>>> {
    vec[0].chars().enumerate().map(|(index,_)|{
        vec.iter().map(|item|{
                item.chars().nth(index).unwrap().to_digit(10)
            }).collect::<Vec<_>>()
    })
    .collect()
}  


/*
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

*/