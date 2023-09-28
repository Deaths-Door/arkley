use std::ops::{Add,Sub,Mul,Div,Neg};
use std::collections::BTreeMap;
use num_notation::{
    Number,
    fraction::Signed
};
use crate::{Expression,Term,Variables,ArithmeticOperation};


impl Mul<Term> for Expression {
    type Output = Self;

    fn mul(self,other : Term) -> Self::Output {
        let expr = match self {
            Expression::Nested(inner) => *inner * other,
            Expression::Term(term) => term * other,
            // if operation == ArithmeticOperation::Durch as 3x * (3/x) can be more simpily done as (3x/1) * (3/x) then other solution
            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Durch => {
                let lexpr = *left * other;
                Expression::Binary { operation,left : Box::new(lexpr), right } // to avoid unnesscary .combine_terms()
            },
            //  if operation == ArithmeticOperation::Mal as things like 3x(4x * 3) need to be 'evaluted' inside before mal with outside 
            Expression::Binary { operation,.. /*, left , right*/ } if operation == ArithmeticOperation::Mal => todo!(),//(*left * *right) * other,

            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Plus  => {
                let lexpr = *left * other.clone();
                let rexpr = *right * other;
                Expression::new_binary(operation,lexpr,rexpr) // to avoid unnesscary .combine_terms()
            },
            Expression::Binary { operation , left , right } /*if operation == ArithmeticOperation::Minus*/ => {
                let lexpr = *left * other.clone();
                let rexpr = -(*right * other);
                //let rexpr = if let Expression::Term(term) = *right { -term * other } else { *right * other };
                Expression::new_binary(operation,lexpr,rexpr) // to avoid unnesscary .combine_terms()
            },
        };

        expr.combine_terms()
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self,other : Expression) -> Self::Output {
        let expr = match self {
            Expression::Nested(inner) => *inner * other,
            Expression::Term(term) => other * term,
            // if operation == ArithmeticOperation::Durch as 3x * (3/x) can be more simpily done as (3x/1) * (3/x) then other solution
            Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Durch => {
                let lexpr = *left * other;
                let rexpr = right;
                Expression::Binary { operation,left : Box::new(lexpr),right : rexpr } // to avoid unnesscary .combine_terms()
            },
            //  if operation == ArithmeticOperation::Mal as things like 3x(4x * 3) need to be 'evaluted' inside before mal with outside 
         //   Expression::Binary { operation , left , right } if operation == ArithmeticOperation::Mal => todo!(),//(*left * *right) * other,

            /*
            // for addition / subtraction mir ist egal 
            Expression::Binary { operation , left , right } => {
                let lexpr = *left * other.clone();
                let rexpr = *right * other;
                Expression::new_binary(operation,lexpr,rexpr) // to avoid unnesscary .combine_terms()
            },*/
            _ => todo!()

        };

        expr.combine_terms()
    }
}