use num_notation::Number;
use std::ops::{Add,Sub,Mul,Div};


use crate::{Expression,Term,ArithmeticOperation};

type AddSubTermPairs = (Term,ArithmeticOperation);

impl Expression {        
    /// Collects all terms of addition (+) or subtraction (-) variants into 'vec' along with 'parent_op'.
    ///
    /// # Returns
    ///
    /// An optional Expression representing the result of combining terms from nested (Nested), multiplication (*),
    /// and division (/) variants.
    fn collect_terms(self,vec : &mut Vec<AddSubTermPairs>) -> Option<Expression> {
        match self {
            Expression::Term(term) => {
                let op = vec.last().map(|(_, operation)| operation.clone()).unwrap_or(ArithmeticOperation::Plus);
                vec.push((term,op));
                None
            },
            Expression::Binary { operation , left , right} if operation == ArithmeticOperation::Plus || operation == ArithmeticOperation::Minus => {
                let lexpr = left.collect_terms(vec);
                let rexpr = right.collect_terms(vec);

                match (lexpr,rexpr) {
                    (None,None) => None,
                    (Some(expr),None) | (None,Some(expr)) => Some(expr),
                    (Some(expr1),Some(expr2)) => Some(Expression::new_binary(operation,expr1,expr2))
                }        
            },
            Expression::Binary { operation , left , right} => {
                let lexpr = left.combine_terms(true); //idk should work
                let rexpr = right.combine_terms(true); // idk should work
                Some(Expression::new_binary(operation,lexpr,rexpr))
            },
            Expression::Nested(inner) => Some(Expression::new_nested(inner.combine_terms(true))) //idk should work
        }
    }

    /// Groups terms based on addition (+) or subtraction (-) operations.
    ///
    /// # Returns
    ///
    /// A vector containing grouped term-operation pairs.
    fn group_terms(vec : &[AddSubTermPairs],is_addition : bool) -> Vec<AddSubTermPairs> {
        let mut grouped_terms : Vec<AddSubTermPairs> = Vec::new();

        for (term,op) in vec {
            let mut is_combined = || -> bool {
                for (grouped_term,_) in &mut grouped_terms {
                    if term.is_combinable_with(grouped_term) {
                        match is_addition {
                            true => grouped_term.coefficient += term.coefficient.clone(),
                            false => grouped_term.coefficient -= term.coefficient.clone(),
                        }

                        return true
                    }
                } 

                return false
            };
        

            if !is_combined() {
                grouped_terms.push((term.clone(),op.clone()));
            }
        }

        grouped_terms
    }

    /// Reconstructs the expression based on grouped terms and an optional nested expression.
    ///
    /// # Returns
    ///
    /// The reconstructed expression.
    fn reconstruct_expression(grouped_terms : &[AddSubTermPairs],nested_expr : Option<Expression>) -> Self {
        let mut expression : Expression = Term::new(Number::Decimal(0.0)).into();

        for (term,sign) in grouped_terms {
            if term.coefficient == 0.0 {
                continue
            }

            let term_expr : Expression = (*term).clone().into();

            expression = Expression::new_binary(sign.clone(),expression,term_expr);
        }

        if let Some(nested) = nested_expr {
            expression = Expression::new_binary(ArithmeticOperation::Plus,expression,nested);
        }
        
        expression
    }
    
    /// Combines terms within the expression.
    ///
    /// # Returns
    ///
    /// The expression with combined terms.
    fn combine_terms(self,is_addition : bool) -> Self {
        let mut vterms = Vec::new();
        let nested_expr = self.collect_terms(&mut vterms);

        let grouped_terms = Self::group_terms(&vterms,is_addition);

        Self::reconstruct_expression(&grouped_terms,nested_expr)
    }
}

impl Add for Expression {
    type Output = Expression;

    fn add(self,other : Expression) -> Self::Output {
        Expression::new_plus(self,other).combine_terms(true)
    }
}

impl Sub for Expression {
    type Output = Expression;

    fn sub(self,other : Expression) -> Self::Output {
        Expression::new_minus(self,other).combine_terms(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Variables;

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
    fn combine_terms() {
        let expression = Expression::new_plus(
            Expression::new_term(create_term_with_variable(2.0,'x',1.0)),
            Expression::new_minus(
                Expression::new_term(create_term_with_variable(1.0,'x',1.0)),
                Expression::new_term(create_term_with_variable(3.0,'y',1.0)),
            )
        );

        check_expression_str(expression.clone(),"2x + x - 3y");

        // Call the combine_terms function to combine like terms
        let result = expression.combine_terms(true);

        check_expression_str(result,"3x + 3y");
    }

    #[test]
    fn combine_terms_add_same_variables() {
        let expr1 : Expression = create_term_with_variable(2.0, 'x', 1.0).into();
        let expr2 : Expression = create_term_with_variable(3.0, 'x', 1.0).into();

        let result = expr1 + expr2;

        check_expression_str(result,"5x");
    }

    #[test]
    fn combine_terms_add_different_variables() {
        let expr1 : Expression = create_term_with_variable(3.0, 'x', 1.0).into();
        let expr2 : Expression = create_term_with_variable(3.0, 'y', 1.0).into();

        let result = expr1 + expr2;
        check_expression_str(result, "3x + 3y");
    }

    #[test]
    fn combine_terms_subtract_same_variables() {
        let expr1 : Expression = create_term_with_variable(2.0, 'x', 1.0).into();
        let expr2 : Expression = create_term_with_variable(5.0, 'x', 1.0).into();

        let result = expr1 - expr2;

        check_expression_str(result,"-3x");
    }

    #[test]
    fn combine_terms_subtract_different_variables() {
        let expr1 : Expression = create_term_with_variable(3.0, 'x', 1.0).into();
        let expr2 : Expression = create_term_with_variable(2.0, 'y', 1.0).into();

        let result = expr1 - expr2;

        check_expression_str(result, "3x - 2y");
    }

    #[test]
    fn combine_terms_with_mul() {
        let expr1 : Expression = Term::new(Number::Decimal(1.0)).into();
        let expr2 : Expression = Expression::new_mal(
            Expression::new_term(create_term_with_variable(3.0, 'x', 1.0)),
            Expression::new_minus(
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
            )
        );

        
        let result = expr1 - expr2;

        check_expression_str(result,"1 + 3x(4x)");
    }
}