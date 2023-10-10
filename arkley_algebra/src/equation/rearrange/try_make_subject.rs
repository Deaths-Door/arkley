use std::cmp::Ordering;

use num_notation::{Signed, One};

use crate::{
    Equation, Term, Variables,
    manipulation::VariableAnalysis, ArithmeticOperation, Expression
};

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
                equation.right = Expression::new_durch(equation.right, term.coefficient.clone().into());
                term.coefficient.set_one();
            }
            else if term.coefficient.clone() % target.coefficient.clone() != 0 {
                return Err(RearrangeError::NonDivisibleCoefficients(equation));
            }
            else {
                // TODO : Move variables with the number over
                term.coefficient /= target.coefficient.clone();
                equation.right = Expression::new_durch(equation.right, target.coefficient.into())
            }
        }
        else { unimplemented!("Figure out how to make x subject for x^2 + 4x = y smth with factorization") }

        Ok(equation)
    }

    /// Determines the side of the equation to rearrange based on the count of variable occurrences.
    fn determine_side_and_rearrange(mut self,variables_to_count : Variables) -> Self {
        let lexpr_count = self.left.count_variable_occurrences(&variables_to_count);
        let rexpr_count = self.right.count_variable_occurrences(&variables_to_count);

        let (left,right) = match lexpr_count.cmp(&rexpr_count) {
            Ordering::Greater => self.left.rearrange(self.right,&variables_to_count),
            Ordering::Equal | Ordering::Less => {
                let (right,left) = self.right.rearrange(self.left,&variables_to_count);
                (left,right)
            }
        };

        self.right = right;
        self.left = left;

        self
    }
}


impl Expression {
    /// Rearrange the equation to isolate specific variables on one side.
    ///
    /// This function attempts to rearrange the equation so that specific variables, as
    /// specified by `variables_to_count`, is isolated on one side of the equation.
    ///
    /// # Arguments
    ///
    /// - `self`: A side of equation to rearrange.
    /// - `other`: Another side of equation to rearrange.
    /// - `variables_to_count`: A reference to the variables to isolate.
    ///
    /// # Returns
    ///
    /// New equations representing the rearranged equation where the specified variable
    /// is isolated on one side.
    fn rearrange(mut self,mut other : Self,variables_to_count : &Variables) -> (Self,Self) {
        match self {
            Self::Binary { operation, left, right } if operation == ArithmeticOperation::Durch => {
                // TODO : Maybe check for possible simplication eg 2/4 = 0.5 or 2/1 = 2 or smth else
                other = Expression::Binary { operation : ArithmeticOperation::Mal, left : Box::new(other), right };
                self = *left;
                self.rearrange(other,variables_to_count)
            },
            Self::Binary { operation, left, right } if operation == ArithmeticOperation::Mal => {
                // TODO : Find a better way then this to handle in future

                let lexpr_count = left.count_variable_occurrences(&variables_to_count);
                let rexpr_count = right.count_variable_occurrences(&variables_to_count);
        
                match lexpr_count.cmp(&rexpr_count) {
                    Ordering::Less => {
                        other = Expression::Binary { operation : ArithmeticOperation::Durch, left : Box::new(other), right : left};
                        self = *right
                    },
                    Ordering::Greater => {
                        other = Expression::Binary { operation : ArithmeticOperation::Durch, left : Box::new(other), right };
                        self = *left
                    },
                    Ordering::Equal => self = *left * *right,
                }

                (self,other)
            },
            Self::Nested(inner) => {
                other = other - *inner;
                self = 0.into();
                (self,other)
            }
            // Do nothing if contains variables from new subject (for now)
            Self::Term(ref term) if term.contains_all(&mut variables_to_count.keys()) => (self,other),
            Self::Term(term) => {
                let operation = match term.coefficient.is_positive() {
                    true => ArithmeticOperation::Minus,
                    false => ArithmeticOperation::Plus
                };
                other = Expression::new_binary(operation,other,term.into());

                self = 0.into();
                (self,other)
            },
            _ => todo!("NOT DONE YET")
        }
    }
}

#[cfg(test)]
mod mul_equal_tests {
    use crate::{parse_expression, Equation, RelationalOperator, Expression};

    #[inline]
    fn from_str(input: &str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    #[inline]
    fn check_expression_str(result: Equation, expected_str: &str) {
        assert_eq!(&result.to_string(), expected_str)
    }

    #[test]
    fn test_traverse_term_mut_with_term_division() {
        // 2x(7) = 3
        let equation = Equation::new(
            from_str("2x * 7"),
            RelationalOperator::Equal,
            from_str("3"),
        );

        match equation.try_make_subject('x'.into()) {
            Ok(value) => check_expression_str(value,"x = (3/7)/2"), // no simpilication is done
            Err(error) => panic!("{:?}", error),
        }
    }

    #[test]
    fn test_traverse_term_mut_with_nested_division() {
        // x(2y) = (3)
        let equation = Equation::new(
            from_str("x * 2y"),
            RelationalOperator::Equal,
            Expression::new_nested(from_str("3")),
        );

        match equation.try_make_subject('x'.into()) {
            Ok(value) => check_expression_str(value,"x = (3)/2y"), // (3) / 2y as no simplification is done to expr
            Err(error) => panic!("{:?}", error),
        }
    }

    #[test]
    fn test_traverse_term_mut_with_binary_division() {
        let equation = Equation::new(
            from_str("6y"),
            RelationalOperator::Equal,
            from_str("2 / 4"),
        );

        match equation.try_make_subject('y'.into()) {
            Ok(value) => check_expression_str(value,"y = (2/4)/6"), // no simplication is done 
            Err(error) => panic!("{:?}", error),
        }
    }
}

#[cfg(test)]
mod div_equal_tests {
    use crate::{parse_expression, Equation, RelationalOperator, Expression, Term, Variables, equation::rearrange::RearrangeError};

    #[inline]
    fn from_str(input: &str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    #[inline]
    fn check_expression_str(result: Equation, expected_str: &str) {
        assert_eq!(&result.to_string(), expected_str)
    }

    #[test]
    fn test_traverse_term_mut_with_term_multiplication() {
        let equation = Equation::new(
            from_str("2x / 7"),
            RelationalOperator::Equal,
            from_str("3"),
        );

        match equation.try_make_subject(Term::new_with_variable(2.0.into(), Variables::from([('x'.into(),1.0.into())]))) {
            Ok(value) => check_expression_str(value,"2x = 3(7)"), 
            Err(error) => panic!("{:?}", error),
        }
    }

    #[test]
    fn test_traverse_term_mut_with_nested_multiplication() {
        // x / 2y = 3
        let equation = Equation::new(
            from_str("x / 2y"),
            RelationalOperator::Equal,
            Expression::new_nested(from_str("3")),
        );
    

        match equation.try_make_subject('x'.into()) {
            Ok(value) => check_expression_str(value,"x = (3)(2y)"), 
            Err(error) => panic!("{:?}", error),
        }
    }

    #[test]
    fn test_traverse_term_mut_with_binary_multiplication() {
        let equation = Equation::new(
            from_str("6y"),
            RelationalOperator::Equal,
            from_str("2 * 4"),
        );
        
        match equation.try_make_subject(Term::new_with_variable(6.0.into(), Variables::from([('x'.into(),1.0.into())]))) {
            Ok(value) => panic!("{:?}", value), 
            Err(error) => match error {
                RearrangeError::UnknownVariablesFound(_) => {},
                _ => assert!(false)
            }

        }
    }
}