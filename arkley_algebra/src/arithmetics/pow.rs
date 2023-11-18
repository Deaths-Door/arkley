use crate::{Term,Expression,Function};
use num_notation::{Pow, Zero, One};

// Follow indices rule (a^m)^n so a^n * m*n  

impl Pow<Term> for Term {
    type Output = Expression;

    fn pow(self,other : Term) -> Self::Output {
        if other.variables.is_empty() {
            if other.coefficient.is_one() {
                return self.into();
            }

            if other.coefficient.is_zero() {
                return 1u8.into();
            }

            let coefficient = self.coefficient.pow(other.coefficient.clone());

            let variables = self.variables.into_iter()
                .map(|(k,v)| (k,v * other.coefficient.clone()))
                .collect();

            return Term::new_with_variable(coefficient, variables).into()
        };

        let iter = self.variables.clone().into_iter()
            .map(|(k,v)| Expression::new_pow(k,other.clone() * v));

        let coefficient = Expression::new_pow(self, other.clone());
        
        iter.fold(coefficient,|acc,e| acc * e )
    }
}

impl Pow<Function> for Function  {
    type Output = Expression; 
    fn pow(self, rhs: Function) -> Self::Output {
        Expression::new_pow(self, rhs)
    }
}
impl Pow<Term> for Function  {
    type Output = Expression; 
    fn pow(self, rhs: Term) -> Self::Output {
        Expression::new_pow(self, rhs)
    }
}
impl Pow<Function> for Expression {
    type Output = Expression; 
    fn pow(self, rhs: Function) -> Self::Output {
        Expression::new_pow(self, rhs)
    }
}

impl Pow<Term> for Expression {
    type Output = Expression;

    fn pow(self,other : Term) -> Self::Output {
        todo!()
    }
}

impl Pow<Self> for Expression {
    type Output = Expression;

    fn pow(self,other : Expression) -> Self::Output {
        todo!()
    }
}
/*
impl Pow for Expression {
    type Output = Expression;

    fn pow(self,other : Expression) -> Self::Output {
        Expression::new_plus(self,other).combine_terms()
    }
}

#[cfg(feature="function")]
impl Pow<Function> for Expression {
    type Output = Expression; 
    fn pow(self, rhs: Function ) -> Self::Output {
        match self {
            Self::Function(ref func) if func.arguments_empty(&rhs) => match func.same(&rhs) {
                true => Expression::new_mal(2.0, self),
                false => Expression::new_plus(self,rhs),
            },
            Self::Function(_) => Expression::new_plus(self,rhs),
            _ => Expression::new_plus(self,rhs)
        }
    }
}*/