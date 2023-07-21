//use std::ops::{Add,Sub,Mul,Div,Rem,AddAssign,SubAssign,MulAssign,DivAssign};

use arkley_numerics::Numeric;

use crate::{Variables,Expression};

pub struct Term<N> where N : Numeric {
    coefficient : N,
    variables : Variables
}
/*
impl<N1,N2> Add<Expression<N2>> for Term<N1> where N1 : Numeric  , N2 : Numeric {
    type Output = Expression<N2>;

    fn add(self,other : Term<N1>) -> Self::Output {
        match self.have_same_chars(other) {
            true => Expression::Term(Term::new(self.coefficient + other.coefficient,self.variable.safe_add(other.variable))),
            false => Expression::Add(self.into(),other.into())
        }
    }
}*/