use std::collections::HashMap;

use num_notation::{Num, Signed, Number};

use crate::{Term, Equation, Expression};

use super::*;

/// Represents an integer quadratic equation of the form `a*x^2 + b*x + c`.
///
/// This struct allows you to work with quadratic equations where `a`, `b`, and `c` can be any numeric type,
/// such as integers (`i32`, `i64`, etc.) or floating-point numbers (`f32`, `f64`).
#[derive(Clone)]
pub struct IntegerQuadratic<T> where T: Num + Clone {
    pub(super) a: T,
    pub(super) b: T,
    pub(super) c: T,
}


impl<T> IntegerQuadratic<T> where T: Num + Clone {
    /// Creates a new `IntegerQuadratic` instance with the provided coefficients.
    ///
    /// # Arguments
    ///
    /// * `a` - The coefficient for the quadratic term (`a*x^2`).
    /// * `b` - The coefficient for the linear term (`b*x`).
    /// * `c` - The constant term (`c`).
    ///
    /// # Returns
    ///
    /// A new `IntegerQuadratic` instance with the given coefficients
    pub const fn new( a: T,b: T,c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T> std::fmt::Debug for IntegerQuadratic<T> where T : std::fmt::Display + Signed + Clone{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self}")
    }
}

impl<T> std::fmt::Display for IntegerQuadratic<T> where T : std::fmt::Display + Signed + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index,(item,x)) in [(&self.a,"x^2"),(&self.b,"x"),(&self.c,"")].iter().enumerate() {
            if item.is_one() {
                f.write_str(x)?
            }

            if !item.is_zero() {
                match index == 0 {
                    true => write!(f,"{item}{x}"),
                    false => match item.is_positive() {
                        true => write!(f," + {item}{x}"),
                        false => write!(f," - {}{}",item.abs(),x)
                    }
                }?
            }
        }
        
        f.write_str(" = 0")
    }
}

impl<T> TryFrom<Term> for IntegerQuadratic<T> where T: Num + Clone + From<u8> + From<Number> {
    type Error = QuadraticError;
    fn try_from(value: Term) -> Result<Self, Self::Error> {
        let mut vec : Vec<_>  = value.variables.into_iter()
            .filter(|(_,value)| value == &2)
            .map(|(_,value)| value )
            .collect();

        match vec.len() {
            0 => Err(QuadraticError::UndefinedConcavity),
            1 => Ok(IntegerQuadratic::new(vec.pop().unwrap().into(),T::zero(),T::zero())),
            _ => Err(QuadraticError::MultipleVariablesToThePowerOf2)
        }
    }   
}

impl<T> Quadratic<Self> for IntegerQuadratic<T> where T: Num + Clone + From<u8>  {
    fn discriminant(self) -> Discriminant<Self> {
        Discriminant(self)
    }

    fn sum_of_roots(self) -> SumOfRoots<Self> {
        SumOfRoots(self)
    }

    fn product_of_roots(self) -> ProductOfRoots<Self> {
        ProductOfRoots(self)
    }

    fn axis_of_symmetry(self) -> AxisOfSymmetry<Self> {
        AxisOfSymmetry(self)
    }

    fn concavity(self) -> Concavity<Self> {
        Concavity(self)
    }

    fn roots(self) -> Roots<Self> {
        Roots(self.discriminant())
    }
}