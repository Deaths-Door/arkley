use crate::{Term,Expression};

impl std::ops::Sub for Term {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        if self.is_combinable_with(&other) {
            let coefficient = self.coefficient - other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new_with_variable(coefficient,variables));
        }
        Expression::new_minus(self.into(),other.into())
    }
}


impl std::ops::Sub for Expression {
    type Output = Expression;

    fn sub(self,other : Expression) -> Self::Output {
        Expression::new_minus(self,other).combine_terms()
    }
}

impl std::ops::Sub<Term> for Expression {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        Expression::new_minus(self,other.into()).combine_terms()
    }
}

#[cfg(feature="function")]
use crate::Function;

#[cfg(feature="function")]
impl std::ops::Sub<Function<'_>> for Function<'_> {
    type Output = Expression; 
    fn sub(self, rhs: Function<'_>) -> Self::Output {
        match self.name() == rhs.name() {
            true => 0.0.into(),
            false => Expression::new_minus(
                self.into(), 
                rhs.into()
            ),
        }
    }
}

#[cfg(feature="function")]
impl std::ops::Sub<Term> for Function<'_> {
    type Output = Expression; 
    fn sub(self, rhs: Term) -> Self::Output {
        Expression::new_minus(self.into(),rhs.into())
    }
}

#[cfg(feature="function")]
impl std::ops::Sub<Function<'_>> for Expression {
    type Output = Expression; 
    fn sub(self, rhs: Function<'_>) -> Self::Output {
        if let Expression::Function { name } = self {
            return match name == rhs.name() {
                true => 0.0.into(),
                false => Expression::new_minus(
                    self.into(), 
                    rhs.into()
                ),
            }
        };

        Expression::new_minus(self.into(),rhs.into())
    }
}


#[cfg(test)]
mod term {
    use super::*;

    use num_notation::Number;
    use crate::Variables;

    #[test]
    fn subtract_terms_with_same_variables() {
        // 5x
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(1.0))]));

        // 2.5x
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));

        // 5x - 2.5x
        let result = term1.clone() - term2.clone();


        // 2.5x
        let expected_term = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(1.0))]));
        let expected_expression = Expression::new_term(expected_term);

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_different_variables() {

        // 5x
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(1.0))]));

        // 5y
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('y', Number::Decimal(1.0))]));

        // 5x - 5y
        let result = term1.clone() - term2.clone();

        // 5x - 5y
        let expected_expression = Expression::new_minus(term1.into(), term2.into());

        assert_eq!(result, expected_expression);
    }

    #[test]
    fn subtract_terms_with_same_variables_and_different_powers() {
        // 5x^3
        let term1 = Term::new_with_variable(Number::Decimal(5.0), Variables::from([('x', Number::Decimal(3.0))]));

        // 2.5x^2
        let term2 = Term::new_with_variable(Number::Decimal(2.5), Variables::from([('x', Number::Decimal(2.0))]));

        // 5x^3 - 2.5x^2
        let result = term1.clone() - term2.clone();

        let expected_expression = Expression::new_minus(term1.into(),term2.into());

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
    fn combine_terms_subtract_nested() {
        let expr1 = Expression::new_minus(create_term_with_variable(5.0, 'x', 1.0).into(),create_term_with_variable(3.0, 'x', 1.0).into());
        let expr2 : Expression = create_term_with_variable(2.0, 'y', 1.0).into();

        let result = expr1 - expr2;

        check_expression_str(result, "2x - 2y");
    }
}