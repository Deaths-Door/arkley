use crate::{Term,Expression};

impl std::ops::Sub for Term {
    type Output = Expression;

    fn sub(self,other : Term) -> Self::Output {
        if self.is_combinable_with(&other) {
            let coefficient = self.coefficient - other.coefficient;
            let variables = self.variables;
            return Expression::new_term(Term::new_with_variables(coefficient,variables));
        }
        Expression::new_minus(self,other)
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
        Expression::new_minus(self,other).combine_terms()
    }
}

#[cfg(test)]
mod term {
    use super::*;
    use test_case::test_case;
    
    #[test_case("5x","2.5x","2.5x")]
    #[test_case("5x","5y"," 5x - 5y")]
    #[test_case("5x^3","2.5x^2","5x^3 - 2.5x^2")]
    fn subtraction_tests(input1 : &str,input2 : &str,expected: &str) {
        assert_eq!(
            (Term::try_from(input1).unwrap() - Term::try_from(input2).unwrap()).to_string().replace(" ",""),
            expected.replace(" ","")
        )
    }
}

#[cfg(test)]
mod expr {
    use super::*;
    use test_case::test_case;

    #[test_case("2x","5x","-3x")]
    #[test_case("3x","2y","3x-2y")]
    #[test_case("5x-3x","2y", "2x - 2y")]    
    fn subtraction_tests(input1 : &str,input2 : &str,expected: &str) {
        assert_eq!(
            (Expression::try_from(input1).unwrap() - Expression::try_from(input2).unwrap()).to_string().replace(" ",""),
            expected.replace(" ","")
        )
    }
}