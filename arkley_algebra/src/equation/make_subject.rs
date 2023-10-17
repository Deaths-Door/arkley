use std::{cmp::Ordering, collections::BTreeMap};

use crate::{
    Equation, Term, Variables, Expression, gcd,
    manipulation::VariableAnalysis, 
};

use super::RearrangeError;

impl Equation {
    /// Makes the specified term the subject of the equation, rearranging it accordingly.
    ///
    /// This method rearranges the equation to make the given `term` the subject. It ensures that
    /// the equation remains balanced and valid. If any unknown variables are encountered during
    /// the process, an error of type `RearrangeError::UnknownVariablesFound` with `self` is returned.
    ///
    ///  # Note
    ///  Try to pass a 'simplified' (structured) equation if possible to make the process much quickers
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
        let mut equation = self.determine_side_and_rearrange(&target.variables);
        
        match equation.left {
            Expression::Term(ref mut term) => {
                let gcd_coefficient = gcd(term.coefficient.clone(),target.coefficient.clone());

                let vars_to_move : Variables;

                #[cfg(nightly)]
                {
                    vars_to_move = term.variables.extract_if(|k,v| !target.variables.contains_key(k)).collect();
                }

                #[cfg(not(nightly))]
                {
                    vars_to_move = term.variables.iter()
                        .filter(|(k,_)| !target.variables.contains_key(k))
                        .map(|(k,v)| (*k,v.clone()))
                        .collect();

                    term.variables.retain(|k,_| !vars_to_move.contains_key(k));
                }

                let to_divide_with =  Term::new_with_variable(term.coefficient.clone() / gcd_coefficient.clone(),vars_to_move);

                equation.right = equation.right / to_divide_with;

                if term.coefficient != target.coefficient {
                    term.coefficient = gcd_coefficient;
                }

                if term.variables != target.variables {
                    return Err(RearrangeError::ImpossibleSolution(equation, target));   
                }

                Ok(equation)
            },
            _ => unimplemented!("Figure out how to make x subject for x^2 + 4x = y smth with factorization or check how to do this")
        }
    }

    /// Determines the side of the equation to rearrange based on the count of variable occurrences.
    fn determine_side_and_rearrange(mut self,variables_to_count : &Variables) -> Self {
        let lexpr_count = self.left.count_variable_occurrences(&variables_to_count);
        let rexpr_count = self.right.count_variable_occurrences(&variables_to_count);

        let (left,right) = match lexpr_count.cmp(&rexpr_count) {
            Ordering::Greater => self.left.rearrange(
                self.right,
                variables_to_count,|_,rexpr| rexpr.contains_any_variable(&mut variables_to_count.keys())),
            Ordering::Equal | Ordering::Less => {
                let (right,left) = self.right.rearrange(
                    self.left,
                    variables_to_count,
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

                        let result = result.unwrap();

                        println!("{:?}",result);
                        assert_eq!(&result.to_string(), $expected);
                    }
                });       
            )*    
        }
    }


    impl_test!(make_subject => 
        { x => 0,  "2x + 3 = 7",'x', "x = 2" },
        { x => 1,  "2 * (x + 3) - 4 = 10 - x",'x', "x = 2.5" },

        { y => 0, "3y - 5 = 1", 'y', "y = 2" },
        { z => 0, "2z + 3 = 1",'z',"z = -1" },

        { p => 0 , "3p + 2q = 12",'p', "p = (12 - 2q)/3" }, /* or p = (-2/3)q + 4  , for this behaviour make new fn */

        { q => 0 , "3p + 2q = 12",'q', "q = (12 - 3p)/2"},
        { q => 1,  "2qy + 3 = 1",'q', "q = (-1)/y"},

        { a => 0 ,  "3b - 2a = 12", 'a', "a = (3b + 12)/2" },
        { a => 1 ,  "2a + 3 = 7", 'a', "a = 2" },


        { b => 0, "3b - 2a = 12", 'b', "b = (2a + 12)/3" } /* or b = (2/3)a + 4  , for this behaviour make new fn */
    );

    /*
    impl_test!(make_y_the_subject_trivial_equation, "y = y",'y', "Infinite solutions");
    impl_test!(variable_on_both_sides, "2x = x + 3",'x', "x = 3");
    impl_test!(no_solution, "2x + 3 = 1",'x', "No solution");
    impl_test!(multiple_solutions, "2x - 4 = 2x - 6",'x', "Infinite solutions");
    impl_test!(variable_with_complex_expression, "2(x + 3) - 4 = 10 - x",'x', "x = 2.5");
    impl_test!(equation_with_constants_only, "3 = 4",'x', "No variable 'x' in the equation");
    impl_test!(empty_input, "",'x', "No equation provided");
    impl_test!(complex_variable, );
    impl_test!(negative_coefficient, "-2x = 6",'x', "x = -3");
    impl_test!(multiple_terms_variable_on_one_side, "3x - 2y = 12",'x', "x = 4 + 2/3y");
    impl_test!(complex_expression_with_parentheses, "(x + 3) = 7",'x', "x = 4");
    impl_test!(equation_with_whitespace, "  2x  +  3  =  7  ",'x', "x = 2");
    impl_test!(no_variable_x_in_equation, "5 = 7",'x', "No variable 'x' in the equation");;*/
}
