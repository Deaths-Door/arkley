use std::cmp::Ordering;

use num_notation::One;

use crate::{Equation, Term, manipulation::VariableAnalysis, Variables, Expression};

use super::RearrangeError;

impl Equation {
    /// Makes the specified term the subject of the equation, rearranging it accordingly.
    ///
    /// This method rearranges the equation to make the given `term` the subject. It ensures that
    /// the equation remains balanced and valid. If any unknown variables are encountered during
    /// the process, an error of type `RearrangeError::UnknownVariablesFound` with `self` is returned.
    ///
    /// # Parameters
    ///
    /// - `term`: The term to be made the subject of the equation.
    ///
    /// # Returns
    ///
    /// Returns a new equation with the specified term as the subject or an error if unknown
    /// variables are encountered.
    pub fn try_make_subject(self,target : Term) -> Result<Self,RearrangeError> {
        if !self.contains_all(&mut target.variables.keys()) {
            return Err(RearrangeError::UnknownVariablesFound(self));
        };

        // TODO : Handle case where making it a subject is not possible
        let mut equation = self.determine_side_and_rearrange(target.variables);

        if let Expression::Term(term) = &mut equation.left {
            if term.coefficient == target.coefficient {}
            else if target.coefficient.is_one() && !term.coefficient.is_one() {
                equation.right = equation.right / term.coefficient.clone();
                term.coefficient.set_one();
            }
            // TODO : Move variables with the number over
            else if term.coefficient.clone() % target.coefficient.clone() != 0 {
                return Err(RearrangeError::NonDivisibleCoefficients(equation));
            }
            else {
                term.coefficient /= target.coefficient.clone();
                equation.right = Expression::new_durch(equation.right, target.coefficient.into())
            }
        }
        else { unimplemented!("Figure out how to make x subject for x^2 + 4x = y smth with factorization or check how to do this") }

        Ok(equation)
    }

    /// Determines the side of the equation to rearrange based on the count of variable occurrences.
    fn determine_side_and_rearrange(mut self,variables_to_count : Variables) -> Self {
        let lexpr_count = self.left.count_variable_occurrences(&variables_to_count);
        let rexpr_count = self.right.count_variable_occurrences(&variables_to_count);

        let (left,right) = match lexpr_count.cmp(&rexpr_count) {
            Ordering::Greater => self.left.rearrange(
                self.right,
                &variables_to_count,|_,rexpr| rexpr.contains_any_variable(&mut variables_to_count.keys())),
            Ordering::Equal | Ordering::Less => {
                let (right,left) = self.right.rearrange(
                    self.left,
                    &variables_to_count,
                    |_,rexpr| rexpr.contains_any_variable(&mut variables_to_count.keys()) );
                (left,right)
            }
        };

        self.right = right;
        self.left = left;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parse_equation;

    macro_rules! impl_test {
        (make_subject => $({ $var:expr => $count : expr,$eq : expr,$subject : expr,$expected : expr } ),* ) => {
            $(
                concat_idents::concat_idents!(fn_name = make_subject_,$var,_,$count {
                    #[test]
                    fn fn_name() {
                        let equation = parse_equation($eq).unwrap().1.unwrap();
                        let result = equation.try_make_subject(Term::from($subject));
                        assert!(result.is_ok());
                        assert_eq!(&result.unwrap().to_string(), $expected);
                    }
                });       
            )*    
        }
    }


    impl_test!(make_subject => 
        { a => 0 ,  "3b - 2a = 12", 'a', "a = (3/2)b - 6" },
        { x => 0,  "2x + 3 = 7",'x', "x = 2" },
        { x => 1,  "2(x + 3) - 4 = 10 - x",'x', "x = 2.5" },

        { y => 0, "3y - 5 = 1", 'y', "y = 2" },
        { z => 0, "2z + 3 = 1",'z',"z = -1" },

        { p => 0 , "3p + 2q = 12",'p', "p = 4 - q"},

        { q => 0 , "3p + 2q = 12",'q', "q = 6 - (3/2)p"},
        { q => 1, "3p + 2q = 12",'q', "q = 6 - (3/2)p" },
        { q => 2,  "2qy + 3 = 1",'q', "q = (1/3)/2y"},

        { b => 0, "3b - 2a = 12", 'b', "b = (4/3) + (2/3)a" }
    );

    /*
    impl_test!(make_y_the_subject_trivial_equation, "y = y",'y', "Infinite solutions");
    impl_test!(variable_on_both_sides, "2x = x + 3",'x', "x = 3");
    impl_test!(no_solution, "2x + 3 = 1",'x', "No solution");
    impl_test!(multiple_solutions, "2x - 4 = 2x - 6",'x', "Infinite solutions");
    impl_test!(variable_with_complex_expression, "2(x + 3) - 4 = 10 - x",'x', "x = 2.5");
    impl_test!(equation_with_constants_only, "3 = 4",'x', "No variable 'x' in the equation");
    impl_test!(empty_input, "",'x', "No equation provided");
    impl_test!(complex_variable, "2a + 3 = 7", 'a', "a = 2");
    impl_test!(negative_coefficient, "-2x = 6",'x', "x = -3");
    impl_test!(multiple_terms_variable_on_one_side, "3x - 2y = 12",'x', "x = 4 + 2/3y");
    impl_test!(complex_expression_with_parentheses, "(x + 3) = 7",'x', "x = 4");
    impl_test!(equation_with_whitespace, "  2x  +  3  =  7  ",'x', "x = 2");
    impl_test!(no_variable_x_in_equation, "5 = 7",'x', "No variable 'x' in the equation");;*/
}
