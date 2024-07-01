use crate::{Term,Expression};

impl std::ops::Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        if self.is_combinable_with(&other) {
            return self.force_add_terms(other).into();
        }

        Expression::new_plus(self,other)
    }
}


impl std::ops::Add for Expression {
    type Output = Expression;

    fn add(self,other : Expression) -> Self::Output {
        Expression::new_plus(self,other).combine_terms()
    }
}


impl std::ops::Add<Term> for Expression {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        Expression::new_plus(self,other).combine_terms()
    }
}

#[cfg(test)]
mod term {
    use super::*;
    use test_case::test_case;
    
    #[test_case("2.5x","3.5x","6x")]
    #[test_case("2.5x","3.5y","2.5x + 3.5y")]
    #[test_case("2.5x^2","3.5x^3","2.5x^2 + 3.5x^3")]
    fn adddition_tests(input1 : &str,input2 : &str,expected: &str) {
        assert_eq!(
            (Term::try_from(input1).unwrap() + Term::try_from(input2).unwrap()).to_string().replace(" ",""),
            expected.replace(" ","")
        )
    }
}

#[cfg(test)]
mod expr {
    use super::*;
    use test_case::test_case;

    #[test_case("2x","1x - 3y","3x - 3y")]
    #[test_case("2x","3x","5x")]
    #[test_case("3x","3y","3x + 3y")]   
    #[test_case("x^2", "2x", "x^2 + 2x")]
    #[test_case("3y^2", "-y", "3y^2 - y")]
    #[test_case("5", "-2", "3")]
    #[test_case("-a", "7", "7 - a")]
    #[test_case("2x + 3", "y^2 - 4y", "2x + y^2 - 4y")]
    #[test_case("-a + b^2", "5c - 1", "-a + b^2 + 5c - 1")]
    #[test_case("-2a", "-3a", "-5a")]
    #[test_case("4a", "-a", "3a")]
    #[test_case("x/2", "y/3", "(3x + 2y) / 6")]
    #[test_case("1/a", "-2/b", "b - 2a / ab")]
    #[test_case("0", "x^3", "x^3")]
    #[test_case("7y", "0", "7y")]
    fn adddition_tests(input1 : &str,input2 : &str,expected: &str) {
        assert_eq!(
            (Expression::try_from(input1).unwrap() + Expression::try_from(input2).unwrap()).to_string().replace(" ",""),
            expected.replace(" ","")
        )
    }
}