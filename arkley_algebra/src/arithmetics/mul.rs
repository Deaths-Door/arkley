use crate::{Term,Expression};

impl std::ops::Mul for Term {
    type Output = Expression;

    fn mul(self,other : Term) -> Self::Output {
        let mut variables = self.variables;
        for (var,exponent) in other.variables {
            variables.entry(var)
                .and_modify(|e| *e += exponent.clone())
                .or_insert(exponent);
        };

        let coefficient = self.coefficient * other.coefficient;
        Expression::new_term(Term::new_with_variable(coefficient,variables))
    }
}

#[cfg(test)]
mod term {
    use super::*;

    use num_notation::Number;
    use crate::Variables;
    
    #[test]
    fn multiply_terms() {
        // 2x
        let term1 = Term::new_with_variable(Number::Decimal(2.0), Variables::from([('x', Number::Decimal(1.0))]));

        //3x^2
        let term2 = Term::new_with_variable(Number::Decimal(3.0), Variables::from([('x', Number::Decimal(2.0))]));

        // 2x * 3x^2
        let result = term1.clone() * term2.clone();

        // 6x^3
        let expected_term = Term::new_with_variable(Number::Decimal(6.0), Variables::from([('x', Number::Decimal(3.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn multiply_terms_with_same_variables_and_different_powers() {
        // 2.5x^2
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 3.5x^3
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2 * 3.5x^3
        let result = term1.clone() * term2.clone();

        // 2.5x^2 * 3.5x^3 = 8.75x^5
        let expected_term = Term::new_with_variable(Number::Decimal(8.75), Variables::from([('x', Number::Decimal(5.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }
}

#[cfg(test)]
mod expr {
    use super::*;

    use num_notation::Number;
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
    fn combine_terms_with_mul() {
        let expr1 : Expression = Term::new(Number::Decimal(1.0)).into();
        let expr2 : Expression = Expression::new_mal(
            Expression::new_term(create_term_with_variable(3.0, 'x', 1.0)),
            Expression::new_plus(
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
                Expression::new_term(create_term_with_variable(2.0, 'x', 1.0)),
            )
        );

        
        let result = expr1 - expr2;

        check_expression_str(result,"1 + 3x(4x)");
    }

    #[test]
    fn mul_expression_by_term_addition() {
        // Test multiplying an expression containing addition by a term
        let expression = Expression::new_plus(
            create_term_with_variable(2.0, 'x', 1.0).into(),
            create_term_with_variable(3.0, 'y', 1.0).into(),
        );

        check_expression_str(expression.clone(), "2x + 3y");

        // Create a term to multiply with the expression
        let term_to_multiply = create_term_with_variable(2.0, 'z', 1.0);

        // Multiply the expression by the term
        let result = expression * term_to_multiply;

        check_expression_str(result, "4xz + 6yz");
    }

    #[test]
    fn mul_expression_by_term_subtraction() {
        // Test multiplying an expression containing subtraction by a term
        let expression = Expression::new_minus(
            create_term_with_variable(5.0, 'x', 1.0).into(),
            create_term_with_variable(3.0, 'y', 1.0).into(),
        );

        check_expression_str(expression.clone(), "5x - 3y");

        // Create a term to multiply with the expression
        let term_to_multiply = create_term_with_variable(2.0, 'z', 1.0);

        // Multiply the expression by the term
        let result = expression * term_to_multiply;

        check_expression_str(result, "10xz - 6yz");
    }

    #[test]
    fn mul_expression_by_term_nested() {
        // Test multiplying a nested expression by a term
        let inner_expression = Expression::new_plus(
            create_term_with_variable(2.0, 'x', 1.0).into(),
            create_term_with_variable(3.0, 'y', 1.0).into(),
        );

        let expression = Expression::new_minus(
            create_term_with_variable(5.0, 'z', 1.0).into(),
            Expression::new_nested(inner_expression),
        );

        check_expression_str(expression.clone(), "5z - (2x + 3y)");

        // Create a term to multiply with the expression
        let term_to_multiply = create_term_with_variable(2.0, 'w', 1.0);

        // Multiply the expression by the term
        let result = expression * term_to_multiply;

        check_expression_str(result, "10wz - 4wx + 6wy");
    }
}