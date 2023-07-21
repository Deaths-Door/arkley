use std::ops::{Add,Sub,Mul,Div,Neg,AddAssign,SubAssign,MulAssign,DivAssign};

use arkley_traits::{Gcd,Zero,Lcm};

/// Approximates the fractional representation of a floating-point number with a given tolerance.
///
/// The function uses the continued fraction algorithm to find the closest rational approximation
/// to the given floating-point `value` with a specified `tolerance`. The resulting approximation
/// is represented as a `Ratio<i64>`, where the numerator and denominator are both of type `i64`.
///
/// # Arguments
/// - `value`: The floating-point number for which to find the fractional representation.
/// - `tolerance`: The maximum tolerance allowed for the approximation. Smaller tolerance values
///               will result in more accurate approximations.
pub fn from_f64(value : f64,tolerance : f64) -> Fraction<i64,i64> {
    let mut x = value;
    let mut h1 = 1;
    let mut h2 = 0;
    let mut k1 = 0;
    let mut k2 = 1;

    while x.abs() > tolerance {
        let a = x.floor() as i64;
        let k_next = a * k1 + k2;
        let h_next = a * h1 + h2;

        x = 1.0 / (x - a as f64);
        h2 = h1;
        h1 = h_next;
        k2 = k1;
        k1 = k_next;
    }
    Fraction::new(h1,k1)
}

/// The `Fraction` struct represents a fraction with a numerator and denominator.
///
/// # Overview
///
/// `Fraction` is designed to be a precise and lossless drop-in replacement for floating-point types.
/// It provides infinite precision and guarantees no loss of information during calculations.
/// The numerator and denominator can be of different types that implements the necessary traits.
///
/// # Usage
///
/// The `Fraction` struct can be used to perform arithmetic operations on fractions,
/// such as addition, subtraction, multiplication, and division.
/// It also supports comparison operations, simplification, conversion to other types and other mathematical operations.
#[derive(Debug,PartialEq)]
pub enum Fraction<N, D> {
    /// Represents an undefined or "Not a Number" fraction.
    NaN,
    /// Represents positive infinity.
    PositiveInfinity,
    /// Represents negative infinity.
    NegativeInfinity,
    /// Represents a top-heavy fraction with a numerator of type `N` and a denominator of type `D`.
    TopHeavy(N, D),
}

impl<N,D> Fraction<N,D> {
    /// Constructs a new `Fraction` instance with the given numerator and denominator.
    ///
    /// # Safety
    ///
    /// This method does not perform any validation or simplification of the fraction.
    /// It assumes that the numerator and denominator are valid and correctly provided. 
    /// If these conditions are not method then operations like Eq maybe not function correctly so use this at your own risk
    pub const fn new_unchecked(numerator : N,denominator : D) -> Self  {
        Fraction::TopHeavy(numerator,denominator)
    }

    /// Returns an option containing a reference to the numerator of the fraction.
    ///
    /// Returns `Some` if the fraction is in the `Fraction::TopHeavy` variant, otherwise returns `None`.
    pub const fn numerator(&self) -> Option<&N>{
        match self {
            Fraction::TopHeavy(numerator,_) => Some(&numerator),
            Fraction::NaN => None,
            Fraction::PositiveInfinity => None,
            Fraction::NegativeInfinity => None,
        }
    }

    /// Returns an option containing a reference to the denominator of the fraction.
    ///
    /// Returns `Some` if the fraction is in the `Fraction::TopHeavy` variant, otherwise returns `None`.
    pub const fn denomator(&self) -> Option<&D>{
        match self {
            Fraction::TopHeavy(_,denomator) => Some(&denomator),
            Fraction::NaN => None,
            Fraction::PositiveInfinity => None,
            Fraction::NegativeInfinity => None,
        }
    }

    /// Returns a new `Fraction` instance that represents the inverse of the current fraction.
    ///
    /// If the current fraction is in the `Fraction::TopHeavy` variant, this method swaps the
    /// numerator and denominator to create the inverse fraction. For other variant types, it returns
    /// `NaN` for `NaN`, swaps `PositiveInfinity` to `NegativeInfinity`, and vice versa.
    pub fn to_inverse(self) -> Fraction<D,N> {
        match self {
            Fraction::TopHeavy(numerator,denomator) => Fraction::new_unchecked(denomator,numerator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}

impl<N,D> Fraction<N,D> where N : Zero +  Gcd + Div<N,Output = N> + PartialOrd, D : Zero + Div<N,Output = D> + Into<N> + Copy + PartialOrd { 
    /// Creates a new fraction with the given numerator and denominator.
    ///
    /// # Arguments
    ///
    /// * `numerator` - The numerator of the fraction.
    /// * `denominator` - The denominator of the fraction.
    ///
    /// # Returns
    ///
    /// A new `Fraction` instance based on the provided numerator and denominator.
    ///
    pub fn new(numerator : N,denominator : D) -> Self {
        if !denominator.is_zero(){
            return Fraction::new_unchecked_reduced(numerator,denominator);
        };

        if numerator.is_zero() {
            return Fraction::NaN;
        }

        if numerator > N::ZERO {
            Fraction::PositiveInfinity
        }
        else {
            Fraction::NegativeInfinity
        }
    }
}

impl<N,D> Fraction<N,D> where N : Div<D> , f64: TryFrom<<N as Div<D>>::Output> {
    /// Converts the fraction to its decimal representation (`f64`).
    ///
    /// # Returns
    ///
    /// - `Ok(value)`: If the conversion is successful, returns the decimal representation as `f64`.
    /// - `Err(err)`: If an error occurs during the conversion, returns the actual error.
    ///
    pub fn as_f64(self) -> Result<f64,<f64 as TryFrom<<N as Div<D>>::Output>>::Error> {
        match self {
            Fraction::TopHeavy(numerator,denominator) => (numerator / denominator).try_into(),
            Fraction::NaN => Ok(f64::NAN),
            Fraction::PositiveInfinity => Ok(f64::INFINITY),
            Fraction::NegativeInfinity => Ok(f64::NEG_INFINITY),
        }
    }
}

impl<N,D> Neg for Fraction<N,D> where N : Neg<Output = N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Fraction::TopHeavy(numerator,denominator) => Fraction::new_unchecked(-numerator,denominator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}

impl<N,D> Fraction<N,D> where N : Gcd + Div<N,Output = N>, D : Div<N,Output = D> + Into<N> + Copy {
    /// Creates a new `Fraction` with the given numerator and denominator.
    /// The fraction is reduced to its simplest form, where the numerator and denominator have no common divisors.
    ///
    /// # Arguments
    ///
    /// * `numerator`: The numerator of the fraction.
    /// * `denominator`: The denominator of the fraction.
    ///
    /// # Returns
    ///
    /// The reduced `Fraction` with the provided numerator and denominator.
    pub fn new_unchecked_reduced(numerator : N,denominator : D) -> Fraction<N,D> {
        let gcd = numerator.gcd(denominator.into());
        Fraction::TopHeavy(numerator / gcd, denominator / gcd)
    }
}

impl<N,D> Add for Fraction<N, D> where N : Add<Output = N> + Mul<Output = N>, D : Lcm + Mul + PartialEq + Into<N> {//N : Gcd + Div<N,Output = N> + Mul<D,Output = N> + Add<Output = N>, D : Div<N,Output = D> + Mul<D,Output = D> + Into<N> + PartialEq + Copy {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::TopHeavy(self_numerator + other_numerator,self_denominator);
                }
                let denominator = self_denominator.lcm(other_denominator);
                let numerator = self_numerator * (denominator / self_denominator).into() + other_numerator * (denominator / other_denominator).into();

                Fraction::new_unchecked(numerator,denominator)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<N,D> Sub for Fraction<N,D> where N : Sub<Output = N> + Mul<Output = N>, D : Lcm + PartialEq + Into<N>  {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::TopHeavy(self_numerator - other_numerator,self_denominator);
                }

                let denominator = self_denominator.lcm(other_denominator);
                let numerator = self_numerator * (denominator / self_denominator).into() - other_numerator * (denominator / other_denominator).into();

                Fraction::TopHeavy(numerator,denominator)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,
            
        }
    }
}

impl<N,D> Mul for Fraction<N,D> where N : Gcd + Mul<Output = N> + Div<Output = N>, D : Mul<Output = D> + Div<N,Output = D> + Into<N> + Copy{//where N : Mul<Output = N>,D : Lcm + Into<N> {
    type Output = Self;
    fn mul(self,other : Self) -> Self {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => { 
                let numerator : N = self_numerator * other_numerator;
                let denominator : D = self_denominator * other_denominator;

                let gcd : N = numerator.gcd(denominator.into());
                return Fraction::TopHeavy(numerator / gcd,denominator / gcd);
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,   
        }
    }
}

impl<N,D> Div for Fraction<N,D> where Self : Mul<Output = Self> ,N : Copy , D : Copy, Fraction<N, D>: From<Fraction<D, N>>{
    type Output = Self;
    fn div(self,other : Self) -> Self {
        self * other.to_inverse().into()
    }
}

impl<N,D> AddAssign for Fraction<N,D> where Self : Add<Self,Output = Self> + Copy {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<N,D> SubAssign for Fraction<N,D> where Self : Sub<Self,Output = Self> + Copy  {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<N,D> MulAssign for Fraction<N,D> where Self : Mul<Self,Output = Self> + Copy  {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<N,D> DivAssign for Fraction<N,D> where Self : Div<Self,Output = Self> + Copy  {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

macro_rules! from_ints {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Fraction<$t,u8> {
                fn from(value: $t) -> Self {
                    Fraction::new_unchecked(value, 1)
                }
            }
        )*
    }
}

impl From<f64> for Fraction<i64,i64> {
    fn from(value: f64) -> Self {
        from_f64(value,1.0e-10)
    }
}

/*
impl<'a,N,D> TryFrom<&'a str> for Fraction<N,D> where N : Numeric + From<&'a str>, D : Numeric + From<&'a str>, Fraction<N, D>: From<f64>{
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self,Self::Error> {
        match s.parse::<f64>() {
            Ok(float) => Ok(float.into()),
            Err(_) => match s {
                "NaN" => Ok(Fraction::NaN),
                "∞" | "+∞" | "inf" | "infinity" | "+inf" => Ok(Fraction::PositiveInfinity),
                "-∞" | "-inf" | "-infinity" => Ok(Fraction::NegativeInfinity),
                _ => match s.find('/') {
                    None => Err(()),
                    Some(index) => {
                        let numerator = N::try_from(&s[..index]).map_err(|_| ())?;
                        let denominator = D::try_from(&s[index + 1..]).map_err(|_| ())?;
                        Ok(Fraction::new_unchecked(numerator, denominator))
                    }
                },
            }
        }
    }
}*/

from_ints!(u8, i8, u16, i16, u32, i32, u64, i64);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        // Create fractions
        let fraction1 = Fraction::new(1, 2);
        let fraction2 = Fraction::new(3, 4);

        // Add fractions
        let result = fraction1 + fraction2;

        // Verify the result
        if let Fraction::TopHeavy(numerator, denominator) = result {
            assert_eq!(numerator, 5);
            assert_eq!(denominator, 4);
        } else {
            assert!(false, "Expected top-heavy fraction");
        }
    }

    #[test]
    fn subtraction() {
        // Create fractions
        let fraction1 = Fraction::new(2,4);
        let fraction2 = Fraction::new(1,4);

        // Add fractions
        let result = fraction1 - fraction2;

        // Verify the result
        if let Fraction::TopHeavy(numerator, denominator) = result {
            assert_eq!(numerator,1);
            assert_eq!(denominator,4);
        } else {
            assert!(false, "Expected top-heavy fraction");
        }
    }

    #[test]
    fn multiplication() {
        // Create fractions
        let fraction1 = Fraction::new(2,4);
        let fraction2 = Fraction::new(1,4);

        // Add fractions
        let result = fraction1 * fraction2;

        // Verify the result
        if let Fraction::TopHeavy(numerator, denominator) = result {
            assert_eq!(numerator,1);
            assert_eq!(denominator,8);
        } else {
            assert!(false, "Expected top-heavy fraction");
        }
    }

    #[test]
    fn divide() {
        // Create fractions
        let fraction1 = Fraction::new(1,1);
        let fraction2 = Fraction::new(1,2);

        // Add fractions
        let result = fraction1 / fraction2;

        // Verify the result
        if let Fraction::TopHeavy(numerator, denominator) = result {
            assert_eq!(numerator,2);
            assert_eq!(denominator,1);
        } else {
            assert!(false, "Expected top-heavy fraction");
        }
    }

    #[test]
    fn test_fraction_equality() {
        let fraction1: Fraction<i32, i32> = Fraction::new(2, 4);
        let fraction2: Fraction<i32, i32> = Fraction::new(1, 2);
        let fraction3: Fraction<i32, i32> = Fraction::new(4, 8);
        let fraction4: Fraction<i32, i32> = Fraction::NaN;

        assert_eq!(fraction1, fraction2);  // Fractions with equivalent values should be equal
        assert_eq!(fraction1, fraction3);  // Fractions with equivalent values should be equal
        assert_ne!(fraction1, fraction4);  // Different variants should not be equal
        assert_ne!(fraction2, fraction4);  // Different variants should not be equal
        assert_ne!(fraction3, fraction4);  // Different variants should not be equal
    }
}