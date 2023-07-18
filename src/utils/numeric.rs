use std::ops::{Add,Sub,Mul,Div,Neg};

use super::{Abs,Lcm,Power,Zero};

/// The `Numeric` trait represents types that can be used as numeric values.
/// Used to restrict the types N and D in the Decimal struct and other related structures, you can use the Numeric trait as a generic type constraint.
pub trait Numeric : Abs + Lcm + Zero + Power<Self> + Add<Self> +  Sub<Self> + Mul<Self> +  Div<Self> + Neg {}

macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {}
        )*
    };
}

impl_numeric!(i8, i16, i32, i64);