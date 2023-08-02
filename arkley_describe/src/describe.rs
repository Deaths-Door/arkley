use crate::{FilterLevel,Step, SubStep};

use crate::utils::*;

/// Represents a generic trait for describing operations.
/// The associated type `Output` specifies the return type of the `describe` method.
pub trait Describe<Rhs = Self>: Sized {
    /// The output type returned by the `describe` method (with is [crate::Step] for arkley).
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
    type Output = Step;

    fn describe_add(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        todo!()
    }

    fn describe_sub(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        todo!()
    }

    fn describe_mul(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> { 
        if filter_level != FilterLevel::Beginner {
            return None;
        }
        const TITLE : &str = "Column Multiplication";
        const DESCRIPTION : &str =  "Multiply each digit in the second number with the digits in the first number, and write the results below each digit in the second number.";

        let mut step = Step::new(TITLE.to_string(),DESCRIPTION.to_string());
       
        let x_neg = self.is_negative();
        let y_neg = other.is_negative();

        let either_neg : bool;
        
        if x_neg && x_neg {
            either_neg = false;
            step.insert_to_description("\nSince both are negative we can simpilfy them into positive");
        }
        else if x_neg || y_neg {
            either_neg = true;

            step.insert_to_description("\nSince we have a negative number lets ignore the sign for now.");
        }

        let (x,y) = swap_if_greater(self.abs(),other.abs());

        let (padding,longest_decimal)  = figure_alignment(x,y);

        let c_aligned = into_column(x,y,"+",padding,longest_decimal);

        let c_aligned_joined = c_aligned.join("");

        let space_index = c_aligned[0].find(' ').unwrap();

        let _c_zero = &c_aligned[0];
        let _c_one = &c_aligned[1];

        // So only valid numbers are there in loop
        let x_str = &_c_zero[space_index.._c_zero.len() - 3].trim_start_matches(' ');
        let y_str = &_c_one[space_index + 2.._c_zero.len() - 3].trim_start_matches(' ');

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

            const BASE : u32 = 10;

            let yd = y_ch.to_digit(BASE).unwrap();//.unwrap_or(0);

            if yd == 0 {
                if y_index == y_str.len() - 1 {
                    continue;    
                }
                
                let format = format!("Now we can skip multiplying {x_str} with 0 as {x_str} * 0 = 0");
                let substep = SubStep::new(format);
                step.add_substep(substep);
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
                    step.add_substep(substep);
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

                step.add_substep(substep);
            }
        }

        todo!("ADDDD");

        if either_neg {
            const INCLUDE_NEG : &str = "We previously omitted the negative sign, but now we've included it into the sum.";
            let substep = SubStep::new(INCLUDE_NEG.to_string());
            step.add_substep(substep);
        }
        
        Some(step)
    }

    fn describe_div(self, other: f64, filter_level: FilterLevel) -> Option<Self::Output> {
        todo!()
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_mul_unsigned_float() {
        let step = 42_f64.describe_mul(32_f64,FilterLevel::Beginner);

        for substep in step.unwrap().substeps() {
            println!("Info = {}",substep.description());
            println!("Latex = {}",substep.latex().clone().unwrap_or(String::from("NO LATEX")));
        }
    }
}