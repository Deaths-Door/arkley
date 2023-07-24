use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};

use crate::*;

/// A trait representing the core arithmetic operations for a type.
/// 
/// This trait requires the implementor to support basic arithmetic operations with other instances of the same type or a compatible type (`Rhs`). 
/// The result of these operations will be of the same type (`Output`) as the implementor.
/// 
/// This trait is mainly used to simplify trait bounds when working with arithmetic operations and their related functionalities.
pub trait ArithmeticCore<Rhs = Self,Output = Self> : 
    Add<Rhs,Output = Output> + 
    Sub<Rhs,Output = Output> + 
    Mul<Rhs,Output = Output> + 
    Div<Rhs,Output = Output> + 
    AddAssign<Rhs> + 
    SubAssign<Rhs> + 
    MulAssign<Rhs> + 
    DivAssign<Rhs> + 
    Power<Rhs,Output = Output> + 
    Abs + 
    Gcd + 
    Zero + 
    Lcm + 
{}

macro_rules! impl_core {
    ($($t:ty),*) => {
        $(
            impl ArithmeticCore for $t {}
        )* 
    }
}

impl_core!(i8,i16,i32,i64);
impl_core!(f32,f64);
