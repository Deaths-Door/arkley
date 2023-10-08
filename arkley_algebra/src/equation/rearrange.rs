use crate::{Equation, Term, manipulation::VariableAnalysis, Expression, ArithmeticOperation};

/*
#[derive(Debug)]
pub enum RearrangeError {
    UnknownVariblesInTerm
}

impl Equation {
    /// todo 
    pub fn with_subject(mut self,term : Term) -> Result<Self,RearrangeError> {
        let _ = term;
        
        let iter = term.variables.keys();

        if !self.contains_all(iter) {
            // TODO add more info as to whats missing maybe
            return Err(RearrangeError::UnknownVariblesInTerm);
        };

       // traverse_until_higher_precedence(&mut self.left)

        //self.left = self.left.clone().traverse_term_mut(&term.variables,&mut self);
        todo!()
    }
}

impl ArithmeticOperation {
    const fn inverse(&self) -> Self {
        match self {
            ArithmeticOperation::Plus => ArithmeticOperation::Minus,
            ArithmeticOperation::Minus => ArithmeticOperation::Plus,
            ArithmeticOperation::Mal =>  ArithmeticOperation::Durch,
            ArithmeticOperation::Durch => ArithmeticOperation::Mal,
        }
    }
} 

/*
impl Equation {
    fn traverse_term_mut(mut self,variable_keys : &[&char]) {
        match self.left {
            Expression::Binary { operation,left, right } => {
                self.right = match *right {
                    Expression::Term(term) if !term.contains_any_variable(variable_keys) => 
                        Expression::new_binary(operation.inverse(),self.right,*right),
                    Expression::Term(term) => {},
                    Expression::Nested(inner) => Expression::new_binary(operation.inverse(),self.right,*right),
                    Expression::Binary { operation, left, right } => todo!(),
                };
            },
            Expression::Term(term) if !term.contains_any_variable(variable_keys) => {
                self.right = Expression::new_plus(self.right, (-term).into());
            },
            Expression::Term(term) => {},
            Expression::Nested(_) => todo!(),
           /* Expression::Binary { operation,left, right } => {
                self.right = match &*right {
                    Expression::Term(term) if !term.contains_any_variable(variable_keys) =>
                        Expression::new_binary(operation.inverse(),self.right,(*right).clone()) ,
                    Expression::Nested(inner) => 
                        Expression::new_binary(operation.inverse(),self.right.clone(),(**inner).clone()) ,
                    _ => self.left.traverse_term_mut(variable_keys,se)
                };
            }
            Expression::Term(term) if !term.contains_any_variable(variable_keys) => {
                self.right = Expression::new_plus(equation.right.clone(), (-term).into());
                0.0.into()
            },
            _ => todo!(),*/
        }
    }
}
*/
impl Expression {
    /// Traverse an expression and apply operations to rearrange terms related to a specific variable.
    ///
    /// This method recursively traverses the expression tree and rearranges terms related to the specified
    /// variable based on the provided equation. It returns the modified expression after rearrangement. (used to overcome double mutable borrow error)
    ///
    /// # Arguments
    ///
    /// * `variable_keys`: A slice containing references to variables (char) to consider for rearrangement.
    /// * `equation`: A mutable reference to the equation containing the left and right sides for rearrangement.
    ///
    /// # Returns
    ///
    /// The modified expression after rearranging terms related to the specified variable.
    #[deprecated(note = "FIGURE OUT A BETTER WAY THEN THIS")]
    fn traverse_term_mut(self /* equation left */,variable_keys : &[&char],equation :&mut Equation) -> Self {
        /*match self {
            Expression::Binary { operation,left, right } => {
                equation.right = match &*right {
                    Expression::Term(term) if !term.contains_any_variable(variable_keys.iter()) =>
                        Expression::new_binary(operation.inverse(),equation.right.clone(),(*right).clone()) ,
                    Expression::Nested(inner) => 
                        Expression::new_binary(operation.inverse(),equation.right.clone(),(**inner).clone()) ,
                    _ => equation.left.clone().traverse_term_mut(variable_keys,equation)
                };

                *left
            }
            Expression::Term(term) if !term.contains_any_variable(variable_keys.iter()) => {
                equation.right = Expression::new_plus(equation.right.clone(), (-term).into());
                0.0.into()
            },
            _ => todo!(),
        }*/
        todo!()
    }
}

#[cfg(test)]
mod addition_tests {
    use crate::{parse_expression, Equation, RelationalOperator, Expression};

    fn from_str(input :&str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    fn check_expression_str(result : Equation,_str : &str) {
        assert_eq!(&result.to_string(),_str)
    }

    #[test]
    fn test_traverse_term_mut_with_term() {
        let mut equation = Equation::new(
            from_str("7"), 
            RelationalOperator::Equal,
            from_str("0")
        );

        let variable_keys = vec![&'x'];
        equation.left = equation.left.clone().traverse_term_mut(&variable_keys,&mut equation);

        // 2x - 7 = 3 into 2x = 3 + 7
        // Expected result: equation.right = equation.right - 2x
        check_expression_str(equation,"0 = -7");
    }

    #[test]
    fn test_traverse_term_mut_with_nested() {
        let mut equation = Equation::new(
            from_str("x + 2y"), 
            RelationalOperator::Equal, 
            Expression::new_nested(from_str("3"))
        );

        let variable_keys = vec![&'x'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys,&mut equation);

        check_expression_str(equation,"x = (3) - 2y"); // (3) - 2y as no simpilification is done to expr
    }

    #[test]
    fn test_traverse_term_mut_with_binary() {
        let mut equation = Equation::new(
            from_str("6y"),
            RelationalOperator::Equal, 
            from_str("2 + 4"), 
        );

        let variable_keys = vec![&'y'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys,&mut equation);

        check_expression_str(equation," 6y = 2 + 4");
    }
}

#[cfg(test)]
mod subtraction_tests {
    use crate::{parse_expression, Equation, RelationalOperator, Expression};

    fn from_str(input: &str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    fn check_expression_str(result: Equation, expected_str: &str) {
        assert_eq!(&result.to_string(), expected_str)
    }

    #[test]
    fn test_traverse_term_mut_with_term_subtraction() {
        let mut equation = Equation::new(
            from_str("2x + 7"),
            RelationalOperator::Equal,
            from_str("3"),
        );

        let variable_keys = vec![&'x'];
        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        // 2x + 7 = 3 into 2x = 3 - 7
        // Expected result: equation.right = equation.right + 2x
        check_expression_str(equation, "2x = 3 - 7");
    }

    #[test]
    fn test_traverse_term_mut_with_nested_subtraction() {
        let mut equation = Equation::new(
            from_str("x + 2y"),
            RelationalOperator::Equal,
            Expression::new_nested(from_str("3")),
        );

        let variable_keys = vec![&'x'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        check_expression_str(equation, "x = (3) - 2y"); // (3) - 2y as no simplification is done to expr
    }

    #[test]
    fn test_traverse_term_mut_with_binary_subtraction() {
        let mut equation = Equation::new(
            from_str("6y"),
            RelationalOperator::Equal,
            from_str("2 - 4"),
        );

        println!("{:?}", equation.left);
        let variable_keys = vec![&'y'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        check_expression_str(equation, "6y = 2 - 4");
    }
}

#[cfg(test)]
mod div_tests {
    use crate::{parse_expression, Equation, RelationalOperator, Expression};

    fn from_str(input: &str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    fn check_expression_str(result: Equation, expected_str: &str) {
        assert_eq!(&result.to_string(), expected_str)
    }

    #[test]
    fn test_traverse_term_mut_with_term_division() {
        let mut equation = Equation::new(
            from_str("2x * 7"),
            RelationalOperator::Equal,
            from_str("3"),
        );

        let variable_keys = vec![&'x'];
        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        // 2x * 7 = 3 into 2x = 3 / 7
        // Expected result: equation.right = equation.right * 2x
        check_expression_str(equation, "2x = 3/7");
    }

    #[test]
    fn test_traverse_term_mut_with_nested_division() {
        // x(2y) = (3)
        let mut equation = Equation::new(
            from_str("x * 2y"),
            RelationalOperator::Equal,
            Expression::new_nested(from_str("3")),
        );

        let variable_keys = vec![&'x'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        check_expression_str(equation, "x = (3) / 2y"); // (3) / 2y as no simplification is done to expr
    }

    #[test]
    fn test_traverse_term_mut_with_binary_division() {
        let mut equation = Equation::new(
            from_str("6y"),
            RelationalOperator::Equal,
            from_str("2 / 4"),
        );

        println!("{:?}", equation.left);
        let variable_keys = vec![&'y'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        check_expression_str(equation, "6y = 2 / 4");
    }
}

#[cfg(test)]
mod mul_tests {
    use crate::{parse_expression, Equation, RelationalOperator, Expression};

    fn from_str(input: &str) -> Expression {
        parse_expression(input).unwrap().1.unwrap()
    }

    fn check_expression_str(result: Equation, expected_str: &str) {
        assert_eq!(&result.to_string(), expected_str)
    }

    #[test]
    fn test_traverse_term_mut_with_term_multiplication() {
        let mut equation = Equation::new(
            from_str("2x / 7"),
            RelationalOperator::Equal,
            from_str("3"),
        );

        let variable_keys = vec![&'x'];
        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        // 2x / 7 = 3 into 2x = 3 * 7
        // Expected result: equation.right = equation.right / 2x
        check_expression_str(equation, "2x = 3(7)");
    }

    #[test]
    fn test_traverse_term_mut_with_nested_multiplication() {
        let mut equation = Equation::new(
            from_str("x / 2y"),
            RelationalOperator::Equal,
            Expression::new_nested(from_str("3")),
        );

        let variable_keys = vec![&'x'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        check_expression_str(equation, "x = (3)(2y)"); // (3) * 2y as no simplification is done to expr
    }

    #[test]
    fn test_traverse_term_mut_with_binary_multiplication() {
        let mut equation = Equation::new(
            from_str("6y"),
            RelationalOperator::Equal,
            from_str("2 * 4"),
        );

        println!("{:?}", equation.left);
        let variable_keys = vec![&'y'];

        equation.left = equation.left.clone().traverse_term_mut(&variable_keys, &mut equation);

        check_expression_str(equation, "6y = 2(4)");
    }
}
*/