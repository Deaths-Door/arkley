use crate::{Term,Expression};

impl std::ops::Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        if self.is_combinable_with(&other) {
            return self.force_add_terms(other).into();
        }

        Expression::new_plus(self.into(),other.into())
    }
}


impl std::ops::Add for Expression {
    type Output = Expression;

    fn add(self,other : Expression) -> Self::Output {
        Expression::new_plus(self,other).combine_terms()
    }
}

#[cfg(test)]
mod term {
    use super::*;

    use num_notation::Number;
    use crate::Variables;

    #[test]
    fn add_terms_with_same_variables() {
        // 2.5x
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));
        //3.5x
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(1.0))]));

        // 2.5x + 3.5x = 6x
        let result = term1.clone() + term2.clone();

        // 6x
        let expected_term = Term::new_with_variable(Number::Decimal(6.0), Variables::from([('x', Number::Decimal(1.0))]));
        
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn add_terms_with_different_variables() {
        //2.5x
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));

        //3.5y
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('y', Number::Decimal(1.0))]));

        // 2.5x + 3.5y
        let result = term1.clone() + term2.clone();

        // 2.5x + 3.5y
        let expected_expression = Expression::new_plus(term1.into(), term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn add_terms_with_same_variables_and_different_powers() {
        // 2.5x^2
        let term1 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 3.5x^3
        let term2 = Term::new_with_variable(Number::Decimal(3.5), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2 + 3.5x^2
        let result = term1.clone() + term2.clone();

        // 2.5x^2 + 3.5x^2
        let expected_expression = Expression::new_plus(term1.into(),term2.into());

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
    fn combine_terms_complex() {
        // 2x
        let expr1 = Expression::new_term(create_term_with_variable(2.0,'x',1.0));
        // 1x - 3y
        let expr2 = Expression::new_minus(
            Expression::new_term(create_term_with_variable(1.0,'x',1.0)),
            Expression::new_term(create_term_with_variable(3.0,'y',1.0)),
        );

        // Call the combine_terms function to combine like terms
        // 2x + 1x - 3y
        let result = expr1 + expr2;

        check_expression_str(result,"3x - 3y");
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

}