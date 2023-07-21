use std::ops::{Add,Sub,Mul,Div,Neg,Rem,AddAssign,SubAssign,MulAssign,DivAssign};

use crate::utils::{Lcm,Gcd,Zero,Numeric,Abs,Power,Log};

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
}

impl<N,D> Fraction<N,D> where N : Div<D> + Copy , D : Copy , f64: From<<N as Div<D>>::Output>{
    
    /// Converts the fraction to a decimal representation (`f64`).
    ///
    /// # Returns
    ///
    /// The decimal representation of the fraction as an `f64`.
    pub fn as_f64(&self) -> f64 {
        match self {
            Fraction::TopHeavy(numerator,denominator) => (*numerator / *denominator).into(),
            Fraction::NaN => f64::NAN,
            Fraction::PositiveInfinity => f64::INFINITY,
            Fraction::NegativeInfinity => f64::NEG_INFINITY,
        }
    }
}

impl<N,D> Fraction<N,D> where N : Copy , D : Copy {
    /// Returns a new `Fraction` instance that represents the inverse of the current fraction.
    ///
    /// If the current fraction is in the `Fraction::TopHeavy` variant, this method swaps the
    /// numerator and denominator to create the inverse fraction. For other variant types, it returns
    /// `NaN` for `NaN`, swaps `PositiveInfinity` to `NegativeInfinity`, and vice versa.
    pub const fn to_inverse(&self) -> Fraction<D,N> {
        match self {
            Fraction::TopHeavy(numerator,denomator) => Fraction::new_unchecked(*denomator,*numerator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}

impl<N,D> Abs for Fraction<N,D> where N : Abs , D : Abs  {
    fn absolute(self) -> Self {
        match self {
            Fraction::TopHeavy(numerator,denominator) => Fraction::new_unchecked(numerator.absolute(),denominator),
            Fraction::NaN => Fraction::NaN,
            _ => Fraction::PositiveInfinity,
        }
    }
}
/*
impl<N,D> Numeric for Fraction<N,D> where Self : Abs + Lcm + Zero + Power<Self> + Add<Self> +  Sub<Self> + Mul<Self> +  Div<Self> + Neg {}


impl<N,D> Gcd for Fraction<N,D> where Self : Zero + Rem<Output = Self> + Sized + Copy  {}
impl<N,D> Lcm for Fraction<N,D> where Self : Gcd + Div<Output = Self> + Mul<Output = Self>  {}

impl<N,D>  Zero for Fraction<N,D> where Self : PartialEq + Sized , u8 : Into<N> + Into<D> {
    const ZERO : Self = Fraction::new_unchecked(0.if into(),1.into());
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}*/


impl<N,D> Neg for Fraction<N,D> where N : Numeric , D : Numeric {
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

impl<N,D> std::fmt::Display for Fraction<N,D> where N : Numeric + std::fmt::Display, D : Numeric + std::fmt::Display{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Fraction::TopHeavy(numerator,denominator) => write!(f,"{}/{}",numerator,denominator),
            Fraction::NaN => write!(f,"{}","NaN"),
            Fraction::PositiveInfinity => write!(f,"{}","∞"),
            Fraction::NegativeInfinity => write!(f,"{}","-∞"),
        }
    }
}

impl<N,D> Add for Fraction<N, D> where N : Add<Output = N> + Mul<Output = N>, D : Lcm + Mul + PartialEq + Into<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::TopHeavy(self_numerator + other_numerator,self_denominator);
                }

                let denominator = self_denominator.lcm(other_denominator);
                let numerator = self_numerator * (denominator / self_denominator).into() + other_numerator * (denominator / other_denominator).into();

                Fraction::TopHeavy(numerator,denominator)
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

/*
impl<N,D> Neg for Fraction<N,D> where N : Neg<Output = N> , D : Copy {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Fraction::TopHeavy(numerator,denominator) => Fraction::new_unchecked(-numerator,denominator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}*/

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