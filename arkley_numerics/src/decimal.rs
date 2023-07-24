use std::ops::{Add,Sub,Mul,Div,Neg};

use super::Fraction;

/// Represents a decimal number as a fraction of two integers.
/// The `Decimal` struct wraps a `Fraction<i32>` to represent decimal numbers with zero precision loss for structs like [super::StandardForm]
#[derive(Debug,PartialEq,PartialOrd)]
pub struct Decimal(Fraction<i32>);

impl Decimal {
    /// Creates a new `Decimal` instance from a given `Fraction<i16, i16>`.
    ///
    /// # Arguments
    ///
    /// * `fraction`: The `Fraction<i16, i16>` to be used as the internal representation of the `Decimal`.
    ///
    /// # Returns
    ///
    /// A new `Decimal` instance with the provided `fraction` as the internal representation.
    pub const fn new(fraction : Fraction<i32>) -> Self {
        Self(fraction)
    }

    
    /// Converts the Decimal to a floating-point representation (f64).
    pub fn to_f64(&self) -> f64 {
        match self.0 {
            Fraction::Proper(numerator,denominator) => {
                let n : f64 = numerator as f64;
                let d : f64 = denominator as f64;
                n / d
            } ,
            Fraction::NaN => f64::NAN,
            Fraction::PositiveInfinity => f64::INFINITY,
            Fraction::NegativeInfinity => f64::NEG_INFINITY,
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
       Decimal::new(self.0 + other.0)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Decimal::new(self.0 - other.0)
    }
}


impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Decimal::new(self.0 * other.0)
    }
}

impl Div for Decimal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Decimal::new(self.0 / other.0)
    }
}

impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Decimal::new(-self.0)
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Fraction::Proper(numerator,denominator) => write!(f,"{}",numerator / denominator),
            Fraction::NaN => write!(f,"NaN"),
            Fraction::PositiveInfinity => write!(f,"+∞"),
            Fraction::NegativeInfinity => write!(f,"-∞"),
        }
    }
}

macro_rules! impl_ints {
    (eq; $($t:ty),*) => {
        $(
            impl PartialEq<$t> for Decimal {
                fn eq(&self,other: &$t) -> bool {
                    self == other
                }
            }
        )*
    };

    (ord; $($t:ty),*) => {
        $(
            impl PartialOrd<$t> for Decimal {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    let rhs : Fraction<i32> = (*other).into();
                    self.0.partial_cmp(&rhs)
                }
            }
        )*
    };

    (add; $($t:ty),*) => {
        $(
            impl Add<$t> for Decimal {
                type Output = Self;

                fn add(self, other: $t) -> Self::Output {
                    Decimal::new(self.0 + other)
                }
            }
        )*
    };

    (sub; $($t:ty),*) => {
        $(
            impl Sub<$t> for Decimal {
                type Output = Self;

                fn sub(self, other: $t) -> Self::Output {
                    Decimal::new(self.0 - other)
                }
            }
        )*
    };

    (div; $($t:ty),*) => {
        $(
            impl Div<$t> for Decimal {
                type Output = Self;

                fn div(self, other: $t) -> Self::Output {
                    Decimal::new(self.0 / other)
                }
            }
        )*
    };

    (mul; $($t:ty),*) => {
        $(
            impl Mul<$t> for Decimal {
                type Output = Self;

                fn mul(self, other: $t) -> Self::Output {
                    Decimal::new(self.0 * other)
                }
            }
        )*
    };

    (operations; $($t:ty),*) => {
        $(
            impl_ints!(add; $t);
            impl_ints!(sub; $t);
            impl_ints!(mul; $t);
            impl_ints!(div; $t);
        )*
    }
}

impl_ints!(eq; i8, i16, i32);
impl_ints!(ord; i8, i16, i32);
impl_ints!(operations; i8, i16, i32);


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_addition() {
        let a = Decimal::new(Fraction::new(1, 4));
        let b = Decimal::new(Fraction::new(3, 4));
        let result = a + b;
        assert_eq!(result.to_f64(), 1.0);
    }

    #[test]
    fn test_subtraction() {
        let a = Decimal::new(Fraction::new(3, 4));
        let b = Decimal::new(Fraction::new(1, 4));
        let result = a - b;
        assert_eq!(result.to_f64(), 0.5);
    }

    #[test]
    fn test_multiplication() {
        let a = Decimal::new(Fraction::new(2, 3));
        let b = Decimal::new(Fraction::new(3, 4));
        let result = a * b;
        assert_eq!(result.to_f64(), 0.5);
    }

    #[test]
    fn test_division() {
        let a = Decimal::new(Fraction::new(2, 3));
        let b = Decimal::new(Fraction::new(3, 4));
        let result = a / b;
        assert_eq!(result.to_f64(), 0.8888888888888888);
    }

    #[test]
    fn test_negation() {
        let a = Decimal::new(Fraction::new(3, 4));
        let result = -a;
        assert_eq!(result.to_f64(), -0.75);
    }
}