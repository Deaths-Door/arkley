use std::ops::{Add,Sub,Mul,Div,Neg,AddAssign,SubAssign,MulAssign,DivAssign};

use super::Fraction;

/// Represents a decimal number as a fraction of two integers.
/// The `Decimal` struct wraps a `Fraction<i16, i16>` to represent decimal numbers with zero precision loss for structs like [super::StandardForm]
pub struct Decimal(Fraction<i16,i16>);

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

    pub const fn new(fraction : Fraction<i16,i16>) -> Self {
        Self(fraction)
    }

    /// Converts the Decimal to a floating-point representation (f64).
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().unwrap()
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


impl AddAssign for Decimal {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl SubAssign for Decimal  {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}

impl MulAssign for Decimal  {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0
    }
}

impl DivAssign for Decimal {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0
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
            Fraction::TopHeavy(numerator,denominator) => write!(f,"{}",numerator / denominator),
            Fraction::NaN => write!(f,"NaN"),
            Fraction::PositiveInfinity => write!(f,"+∞"),
            Fraction::NegativeInfinity => write!(f,"-∞"),
        }
    }
}


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

    #[test]
    fn test_add_assign() {
        let mut a = Decimal::new(Fraction::new(1, 4));
        let b = Decimal::new(Fraction::new(3, 4));
        a += b;
        assert_eq!(a.to_f64(), 1.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Decimal::new(Fraction::new(3, 4));
        let b = Decimal::new(Fraction::new(1, 4));
        a -= b;
        assert_eq!(a.to_f64(), 0.5);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Decimal::new(Fraction::new(2, 3));
        let b = Decimal::new(Fraction::new(3, 4));
        a *= b;
        assert_eq!(a.to_f64(), 0.5);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Decimal::new(Fraction::new(2, 3));
        let b = Decimal::new(Fraction::new(3, 4));
        a /= b;
        assert_eq!(a.to_f64(), 0.8888888888888888);
    }
}