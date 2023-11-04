use crate::{
    FilterLevel,
    Step,
    SubStep,
    SupportedLanguages,
    utils::*
};

use fluent_templates::LanguageIdentifier;

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
    fn describe_add(self,other : Rhs,filter_level : FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output>;

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
    fn describe_sub(self, other: Rhs, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output>;

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
    fn describe_mul(self, other: Rhs, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output>;

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
    fn describe_div(self, other: Rhs, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output>;
}

impl Describe<f64> for f64 {
    type Output = SolutionSteps;

    fn describe_add(self, other: f64, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output> {
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        todo!()
    }

    fn describe_sub(self, other: f64, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output> {
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        todo!()
    }

    fn describe_mul(self, other: f64, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output> { 
        if filter_level != FilterLevel::Beginner {
            return None;
        }

        todo!()
    }

    fn describe_div(self, other: f64, filter_level: FilterLevel,lang : &SupportedLanguages) -> Option<Self::Output> {
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