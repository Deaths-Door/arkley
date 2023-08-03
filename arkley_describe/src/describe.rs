use crate::{FilterLevel,Step, SubStep};

use crate::utils::*;

/// Represents a list of steps to solve a mathematical problem.
pub type SolutionSteps  = Vec<Step>;

/// Represents a generic trait for describing operations.
/// The associated type `Output` specifies the return type of the `describe` method.
pub trait Describe<Rhs = Self>: Sized {
    /// The output type returned by the `describe` method (is [Method] for arkley).
    type Output;

    /// Describes the addition operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the addition.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the addition operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_add(self,other : Rhs,filter_level : FilterLevel) -> Option<Self::Output>;

    // Describes the subtraction operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the subtraction operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_sub(self, other: Rhs, filter_level: FilterLevel) -> Option<Self::Output>;

    /// Describes the multiplication operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the multiplication.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the multiplication operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_mul(self, other: Rhs, filter_level: FilterLevel) -> Option<Self::Output>;

    /// Describes the division operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the division.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    ///
    /// # Returns
    ///
    /// An `Option<Self::Output>` representing the description of the division operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_div(self, other: Rhs, filter_level: FilterLevel) -> Option<Self::Output>;
}

impl Describe<f64> for f64 {
    type Output = SolutionSteps;

    fn describe_add(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        todo!()
    }

    fn describe_sub(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        todo!()
    }

    fn describe_mul(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> { 
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        const DESCRIPTION : &str =  "Multiply each digit in the second number with the digits in the first number, and write the results below each digit in the second number.";

        let mut solution = SolutionSteps::new();

        let mut step1 = Step::new(DESCRIPTION.to_string());
       
        let x_neg = self.is_negative();
        let y_neg = other.is_negative();

        let mut either_neg : bool = false;
        
        if x_neg && x_neg {
            step1.insert_to_description("\nSince both are negative we can simpilfy them into positive");
        }
        else if x_neg || y_neg {
            either_neg = true;
            step1.insert_to_description("\nSince we have a negative number lets ignore the sign for now.");
        }

        let (x,y) = swap_if_greater(self.abs(),other.abs());

        let (padding,longest_decimal)  = figure_alignment(x,y);

        let c_aligned = into_column(x,y,"+",padding,longest_decimal);

        let c_aligned_joined = c_aligned.join("");

        let space_index = c_aligned[0].find(' ').unwrap();

        let _c_zero = &c_aligned[0];
        let _c_one = &c_aligned[1];

        // So only valid numbers are there in loop
        let x_str = &_c_zero[space_index.._c_zero.len() - 3].trim_start_matches(SPACE);
        let y_str = &_c_one[space_index + 2.._c_zero.len() - 3].trim_start_matches(SPACE);

        // to take into account decimal points in numbers for the factor scaling so 10 to the power of index - (encounter as i32)
        let mut x_dec_encounted = false;
        let mut y_dec_encounted = false;

        // for sums of mul
        let mut sum = String::new();

        for  (y_index,y_ch) in y_str.chars().rev().enumerate() {
            if y_ch == '.' {
                y_dec_encounted = true;
                continue;
            }

            let yd = y_ch.to_digit(BASE).unwrap();

            if yd == 0 {
                if y_index == y_str.len() - 1 {
                    continue;    
                }
                
                let format = format!("Now we can skip multiplying {x_str} with 0 as {x_str} * 0 = 0");
                let substep = SubStep::new(format);
                step1.add_substep(substep);
                continue;
            }

            let y_factor = 10_u32.pow(y_index as u32 - y_dec_encounted as u32);

            let y_digit = yd * y_factor;

            for (x_index,x_ch) in x_str.chars().rev().enumerate() {
                if x_ch == '.' {
                    x_dec_encounted = true;
                    continue;
                }

                let xd = x_ch.to_digit(BASE).unwrap();

                if xd == 0 {
                    let format = format!("Now we can skip multiplying 0 with {y_str} as 0 * {y_str} = 0");
                    let substep = SubStep::new(format);
                    step1.add_substep(substep);
                    continue;
                }

                let x_factor = 10_u32.pow(x_index as u32 - x_dec_encounted as u32);
                
                let x_digit = xd * x_factor;

                let product = y_digit * x_digit;

                sum += &align(product as f64,padding,longest_decimal);

                let description = format!(r"Multiply ${y_digit} \times ${x_digit} which is {product}\nNow write the product down below");

                let latex = align_latex_end(&format!("{c_aligned_joined}{sum}"));

                let mut substep = SubStep::new(description);
                substep.set_latex(latex);

                step1.add_substep(substep);
            }
        }

        solution.push(step1);

        const START_ADD : &str = "Now lets add theh results of the mul together";        

        let mut step2 = Step::new(START_ADD.to_string());
        describe_add(&mut step2,sum);

        solution.push(step2);

        if either_neg {
            const INCLUDE_NEG : &str = "We previously omitted the negative sign, but now we've included it into the sum. ";
            let step3 = Step::new(INCLUDE_NEG.to_string());
            solution.push(step3);
        }
        
        Some(solution)
    }

    fn describe_div(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        todo!()
    }
}

fn describe_add(step : &mut Step,column : String) {
    let c_aligned : Vec<_> = column.split(r"\\&").map(|s| s.trim_start_matches(SPACE)).collect();
    
    let c_pairs : Vec<_> = c_aligned[0].chars().enumerate().map(|(index,_)|{
        c_aligned.iter().map(|item|{
                item.chars().nth(index).unwrap_or(' ').to_digit(BASE)
        }).collect::<Vec<_>>()
    })
    .collect();

    let mut previous_carry = 0;

    let mut sum_str = String::new();
    let mut carry_str = String::from("0");

    for pair in c_pairs.iter().rev() {
        let mut description = String::from("Calculate ");

        let mut c_sum = previous_carry;

        for &n in pair.iter().flatten() {
            c_sum += n;
            description += &format!("{n} + ");
        }

        if c_sum == 0 {
            description += &format!("\nSince sum is 0 we can skip this column");
            continue;
        }

        if previous_carry != 0 {
            description += &format!("{previous_carry} = {c_sum}\n{previous_carry} came from the previous carry forward");
        }

        else if c_sum >= 10 {
            description += &format!("\nSince sum is greater than 10 we carry 1 forward");
        }
        
        let c_rem = c_sum % 10;
        sum_str.insert_str(0,&c_rem.to_string());

        previous_carry = c_sum / 10;
        carry_str.insert_str(0,&previous_carry.to_string());

        let _latex = format!(r"& {carry_str} \\ {column} {SEPERATOR} & {sum_str} \\");
        let latex = align_latex_end(&_latex);

        let mut substep = SubStep::new(description);
        substep.set_latex(latex);

        step.add_substep(substep);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_mul_unsigned_float() {
        let solution = 42_f64.describe_mul(32_f64,FilterLevel::Beginner).unwrap();
        println!("{:?}",solution);
    }
}