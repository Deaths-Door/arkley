use arkley_numerics::Numeric;

use crate::Term;

pub enum Expression<N> where N : Numeric {
    Term(Term<N>),
    Add(Box<Expression<N>>, Box<Expression<N>>),
    Subtract(Box<Expression<N>>, Box<Expression<N>>),
    Multiply(Box<Expression<N>>, Box<Expression<N>>),
    Divide(Box<Expression<N>>, Box<Expression<N>>),
}