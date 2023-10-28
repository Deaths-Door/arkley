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
impl std::ops::Sub<Term> for Function  {
    type Output = Expression; 
    fn sub(self, rhs: Term) -> Self::Output {
        Expression::new_minus(self.into(),rhs.into())
    }
}


#[cfg(feature="function")]
use crate::Function;

#[cfg(feature="function")]
impl std::ops::Sub<Function> for Function  {
    type Output = Expression; 
    fn sub(self, rhs: Function ) -> Self::Output {
        match self.same(&rhs) && self.arguments_empty(&rhs) {
            true => 0.0.into(),
            false => Expression::new_minus(
                self.into(), 
                rhs.into()
            ),
        }
    }
}

#[cfg(feature="function")]
impl std::ops::Sub<Function > for Expression {
    type Output = Expression; 
    fn sub(self, rhs: Function ) -> Self::Output {
        match self {
            Self::Function(ref func) if func.arguments_empty(&rhs) => match func.same(&rhs) {
                true => Expression::new_minus(0.0.into(), self.into()),
                false => Expression::new_minus(self.into(),rhs.into()),
            },
            Self::Function(_) => Expression::new_minus(self.into(),rhs.into()),
            _ => Expression::new_minus(self.into(),rhs.into())
        }
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
    use std::collections::BTreeMap;

    use super::*;

    use num_notation::Number;
    use crate::{Variables, parse_expression};
    
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

    fn cos(arg : Expression) -> Function {
        Function::new_default("cos", 1.into(),BTreeMap::from([('x',arg.into())]))
    }

    fn sin(arg : Expression) -> Function {
        Function::new_default("sin", 1.into(),BTreeMap::from([('x',arg.into())]))
    }
    
    #[test]
    fn with_functions() {

        let _expr = parse_expression("2x ",&(Default::default())).unwrap().1;
        let cos = cos(_expr);

        let expr = parse_expression("3x",&(Default::default())).unwrap().1;
        let result = expr - cos;

        check_expression_str(result, "3x - cos(2x)");
    }

    #[test]
    fn func_sub_func() {
        let cos1 = cos((-1).into());
        let cos2 = cos(1.into());
        let result = cos1 - cos2;

        check_expression_str(result, "cos(-1) - cos(1)");
    }

    #[test]
    fn cos_x_and_sin_x() {
        let cos_x = cos('x'.into());
        let sin_x = sin('x'.into());
        let result = cos_x - sin_x;

        check_expression_str(result, "cos(x) - sin(x)");
    }

    #[test]
    fn funcition_complex() {
        let sin_x = sin('x'.into());
        
        let cos_1 = cos(1.into());
        let sin_1 = sin(1.into());

        let expr = parse_expression("2x - 7x - 5y",&(Default::default())).unwrap().1;

        let lexpr = Expression::new_plus(expr, sin_1.into()); // 2x - 7x - 5y + sin(1)
        let rexpr = Expression::new_mal(cos_1.into(), sin_x.into()); // cos(1)sin(x)

        // -5x - 5y + sin(1) - (cos(1) * sin(x))
        let result = lexpr - rexpr;

        check_expression_str(result, "sin(1) - 5x - 5y - cos(1)sin(x)");
    }

}