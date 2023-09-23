use std::ops::{Add,Sub,Mul,Div,Neg};

use num_notation::Number;

use crate::{Term,ArithmeticOperation,Variables};
/*
impl Expression {
    fn combine_terms(self) -> Self {
        let mut vterms = Vec::new();
        self.collect_terms(&mut vterms,None);

        let mut grouped_terms : Vec<(Term,Option<ArithmeticOperation>)> = Vec::new();

        for (term,sign) in vterms {
            let mut is_combined = || -> bool {
                for (grouped_term,_) in &mut grouped_terms {
                    if term.variables == grouped_term.variables {
                        grouped_term.coefficient += term.coefficient.clone();
                        return true
                    }
                } 

                return false
            };
        

            if !is_combined() {
                grouped_terms.push((term,sign));
            }
        }

        Self::reconstruct_expression(&grouped_terms)
    }

    fn reconstruct_expression(grouped_terms : &[(Term,Option<ArithmeticOperation>)]) -> Self {
        let mut expression : Expression = Term::new(Number::Decimal(0.0)).into();

       for (term,sign) in grouped_terms {
            if term.coefficient == 0.0 {
                continue
            }

            let term_expr : Expression = (*term).clone().into();

            expression = match sign {
                None => term_expr,
                Some(old_sign) => Expression::new_binary(old_sign.clone(),expression,term_expr)
            }
        }

        // to remove the extra 0 + added at beginning
        /*if let Expression::Binary { right,.. } = expression {
            expression = *right;
        }*/

        expression
    }

    fn collect_terms(&self, vec: &mut Vec<(Term,Option<ArithmeticOperation>)>,parent_operation : Option<ArithmeticOperation>) {
        match self {
            Expression::Term(term) => vec.push((term.clone(),parent_operation)),
            Expression::Nested(_) => todo!("MAYBE RETURN Option<Expression> for nested variants and then construct it all together"),
            Expression::Binary { operation , left , right } => {
                left.collect_terms(vec,Some(operation.clone()));
                right.collect_terms(vec,Some(operation.clone()))
            }
        }
    }
}*/


impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {     
        match self {
            Expression::Term(term) => Expression::new_term(-term),
            Expression::Nested(inner) => Expression::new_nested(-*inner),
            Expression::Binary { operation , left , right } => Expression::new_binary(operation.negate_if_plus_or_minus(),-*left,-*right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a Term with a single variable.
    fn create_term_with_variable(coeff: f64, var: char, exp: f64) -> Term {
        let mut variables = Variables::new();
        variables.insert(var, Number::Decimal(exp));
        Term::new_with_variable(Number::Decimal(coeff), variables)
    }    

    fn check_expression_str(expression : Expression,_str : &str) {
        assert_eq!(&expression.to_string(),_str)
    }

    #[test]
    fn test_neg() {
        // x + x - (x)
        let expression = Expression::new_plus(
            create_term_with_variable(1.0,'x',1.0).into(),
            Expression::new_minus(
                create_term_with_variable(1.0,'x',1.0).into(),
                Expression::new_nested(create_term_with_variable(1.0,'x',1.0).into())
            )
        );

        let negated_expression = -expression;

        // x - x + (-x)
        let expected_expression = Expression::new_minus(
            create_term_with_variable(-1.0,'x',1.0).into(),
            Expression::new_plus(
                create_term_with_variable(-1.0,'x',1.0).into(),
                Expression::new_nested(create_term_with_variable(-1.0,'x',1.0).into())
            )
        );

        assert_eq!(negated_expression, expected_expression);
    }

    #[test]
    fn test_combine_terms() {
        let expression = Expression::new_plus(
            Expression::new_term(create_term_with_variable(2.0,'x',1.0)),
            Expression::new_minus(
                Expression::new_term(create_term_with_variable(1.0,'x',1.0)),
                Expression::new_term(create_term_with_variable(3.0,'y',1.0)),
            )
        );

        check_expression_str(expression.clone(),"2x + x - 3y");

        // Call the combine_terms function to combine like terms
        let result = expression.combine_terms();

        // Define the expected result
        let expected_expression = Expression::new_minus(
            Expression::new_term(create_term_with_variable(3.0,'x',1.0)),
            Expression::new_term(create_term_with_variable(3.0,'y',1.0)),
        );

        check_expression_str(expected_expression.clone(),"3x - 3y");

        // Check if the expression matches the expected result
        assert_eq!(result, expected_expression);
    }

    #[test]
    fn test_combine_terms_same_variables() {
        // Test combining terms with the same variables
        let expression = Expression::new_plus(
            Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
            Expression::new_term(create_term_with_variable(3.0, 'x', 1.0)),
        );

        check_expression_str(expression.clone(), "2x + 3x");

        // Call the combine_terms function to combine like terms
        let result = expression.combine_terms();

        // Define the expected result
        let expected_expression = Expression::new_term(create_term_with_variable(5.0, 'x', 1.0));

        check_expression_str(expected_expression.clone(), "5x");

        // Check if the expression matches the expected result
        assert_eq!(result, expected_expression);
    }

    #[test]
    fn test_combine_terms_different_variables() {
        // Test combining terms with different variables
        let expression = Expression::new_minus(
            Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
            Expression::new_term(create_term_with_variable(3.0, 'y', 1.0)),
        );

        check_expression_str(expression.clone(), "2x - 3y");

        // Call the combine_terms function to combine like terms
        let result = expression.combine_terms();

        // Since the terms have different variables, they should not be combined
        let expected_expression = Expression::new_minus(
            Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
            Expression::new_term(create_term_with_variable(3.0, 'y', 1.0)),
        );

        check_expression_str(expected_expression.clone(), "2x - 3y");

        // Check if the expression matches the expected result
        assert_eq!(result, expected_expression);
    }

    #[test]
    fn combine_terms_with_mul() {
        // Test combining terms with different variables
        let expression = Expression::new_plus(
            Term::new(Number::Decimal(1.0)).into(),
            Expression::new_mal(
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
                Expression::new_term(create_term_with_variable(3.0, 'x', 1.0)),
            )
        );

        check_expression_str(expression.clone(), "1 + 2x(3x)");

        // Call the combine_terms function to combine like terms
        let result = expression.combine_terms();

        // Since the terms have different variables, they should not be combined
        let expected_expression =  Expression::new_plus(
            Term::new(Number::Decimal(1.0)).into(),
            Expression::new_mal(
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
                Expression::new_term(create_term_with_variable(3.0, 'x', 1.0)),
            )
        );

        check_expression_str(expected_expression.clone(),"1 + 2x(3x)");

        // Check if the expression matches the expected result
        assert_eq!(result, expected_expression);
    }
}
/*
#[derive(Clone)]
#[cfg_attr(test,derive(PartialEq))]
pub enum Expression {
    /// Represents a basic unit in a mathematical expression.
    Term(Term),

    /// Represents the addition of two terms.
    ///
    /// The `Plus` variant is a binary operator (+) that takes two `Term` values as its operands.
    Plus(Box<Expression>,Box<Expression>),

    /// Represents the subtraction of two terms.
    ///
    /// The `Minus` variant is a binary operator (-) that takes two `Term` values as its operands.
    Minus(Box<Expression>,Box<Expression>),

    /// Represents the multiplication of two terms.
    ///
    /// The `Mal` variant is a binary operator (*) that takes two `Term` values as its operands.
    Mal(Box<Expression>,Box<Expression>),

    /// Represents the division of two terms.
    ///
    /// The `Durch` variant is a binary operator (/) that takes two `Term` values as its operands.
    Durch(Box<Expression>,Box<Expression>),

    /// Represents a more complex expression that contains nested expressions that contain `()` 
    Nested(Box<Expression>),
}

impl Expression {
    pub(crate) fn combine_terms(&mut self) {
        let mut terms : Vec<(Term,Option<bool>)> = Vec::new();
        self.collect_terms(&mut terms,None);

        let mut grouped_terms: Vec<(Term,Option<bool>)> = Vec::new();

        for (term,sign) in terms {
            let mut combined = false;
            for (grouped_term,_) in &mut grouped_terms {
                if term.variables == grouped_term.variables {
                    grouped_term.coefficient += term.coefficient.clone();
                    combined = true;
                    break;
                }
            }
            if !combined {
                grouped_terms.push((term,sign)); // Clone the term to keep the original
            }
        }
        /*
impl Expression {
    fn collect_terms(&self, vec: &mut Vec<(Term,Option<ArithmeticOperation>)>) {
        match self {
            Expression::Term(term) => vec.push((term,None)),
            Expression::Nested(inner)
           /* Expression::Term(term) => {
                terms.push((term.clone(),parent_sign));
            },
            Expression::Plus(right, left) => {
                right.collect_terms(terms,Some(true));
                left.collect_terms(terms,Some(true));
            },
            Expression::Minus(right, left) => {
                right.collect_terms(terms,Some(false));
                left.collect_terms(terms,Some(false));
            },
            Expression::Nested(inner) => {
                inner.collect_terms(terms,None);
            }

            //Not done as i dont think i neeed to do anything here
            Expression::Mal(_, _) | Expression::Durch(_, _) => {}*/
        }
    }
}*/

        // Reconstruct the expression.
        // We use `None` here because creating `let mut new_expression = Expression::new_term(Term::new(Number::Decimal(0.0)));`
        // would result in an expression like `0 + ...`. To avoid this, we "ignore" zero coefficients and aim to simplify the
        // reconstructed expression. 
        // In the future, consider updating the implementation of the `Display` trait to remove all terms with zero coefficients from resulting string
        // Or create a new function to remove all 0s from the expression tree to further improve expression simplification.
        let mut new_expression = None;

        for (term,sign) in grouped_terms {
            if term.coefficient != 0.0 {
                let term_expression = Expression::new_term(term.clone());

                new_expression = match new_expression {
                    None => Some(term_expression),
                    Some(expr) => match sign {
                        None => Some(term_expression),
                        Some(is_add) => Some(match is_add {
                            true => Expression::new_plus(expr,term_expression),
                            false => Expression::new_minus(expr,term_expression)
                        })
                    }
                }
            }
        }

        *self = new_expression.unwrap();
    }

    fn collect_terms(&self, terms: &mut Vec<(Term,Option<bool>)>,parent_sign : Option<bool>) {
        match self {
            Expression::Term(term) => {
                terms.push((term.clone(),parent_sign));
            },
            Expression::Plus(right, left) => {
                right.collect_terms(terms,Some(true));
                left.collect_terms(terms,Some(true));
            },
            Expression::Minus(right, left) => {
                right.collect_terms(terms,Some(false));
                left.collect_terms(terms,Some(false));
            },
            Expression::Nested(inner) => {
                inner.collect_terms(terms,None);
            }

            //Not done as i dont think i neeed to do anything here
            Expression::Mal(_, _) | Expression::Durch(_, _) => {}
        }
    }
}

/*
impl Mul<Term> for Expression {
    type Output = Self;

    fn mul(self,other : Term) -> Expression {
        match self {
            Expression::Term(term) => (other * term).into(),
            Expression::Nested(inner) => (*inner * other).into(),
            //right * other *left + other  into expression .combine_all_terms.... is answer
            Expression::Plus(right, left) | Expression::Minus(right, left) => 
            _ => todo!()
            //  Expression::Mal(right, left)| Expression::Durch(right, left) => {
        }

        // (2x)(2x +  ..1 + y) for + and - yes
        // (2x)()
    }
}*/
/*
macro_rules! primitives! {
    
};*/*/