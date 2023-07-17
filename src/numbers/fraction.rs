use crate::utils::{Zero,Lcm};

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

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
#[derive(Debug)]
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

impl<N,D> for Fraction<N,D> {
    /// Constructs a new `Fraction` instance with the given numerator and denominator.
    ///
    /// # Safety
    ///
    /// This method does not perform any validation or simplification of the fraction.
    /// It assumes that the numerator and denominator are valid and correctly provided.
    pub const fn new_unchecked(numerator : N,denominator : D) -> Self  {
        Fraction::TopHeavy(numerator,denomator)
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
    pub const fn to_inverse(&self) -> Self {
        match self {
            Fraction::TopHeavy(numerator,denomator) => Fraction::new_unchecked(denomator,numerator),
            Fraction::NaN => None,
            Fraction::PositiveInfinity => NegativeInfinity,
            Fraction::NegativeInfinity => PositiveInfinity,
        }
    }

    /// Returns a self that represents the inverse of the current fraction.
    ///
    /// If the current fraction is in the `Fraction::TopHeavy` variant, this method swaps the
    /// numerator and denominator to create the inverse fraction. For other variant types, it returns
    /// `NaN` for `NaN`, swaps `PositiveInfinity` to `NegativeInfinity`, and vice versa.
    pub const fn as_inverse(mut self) -> Self {
        self = match self {
            Fraction::TopHeavy(numerator,denomator) => Fraction::new_unchecked(denomator,numerator),
            Fraction::NaN => None,
            Fraction::PositiveInfinity => NegativeInfinity,
            Fraction::NegativeInfinity => PositiveInfinity,
        }
        self
    }
}

impl<N,D> Fraction<N,D> where N : Zero + PartialOrd, D : Zero + PartialOrd { 
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
            return Fraction::TopHeavy(numerator,denominator);
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

impl<N,D> Add for Fraction<N, D> where N : Add<Output = N> + Mul<Output = N>, D : Lcm + Mul + Into<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::TopHeavy(self_numerator + other_numerator,self_denominator);
                }

                let denominator = self_denominator.lcm(&other_denominator);
                let numerator = self_numerator * (denominator / self_denominator).into() + other_numerator * (denominator / other_denominator).into();

                Fraction::TopHeavy(numerator,denominator)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<N,D> Sub for Fraction<N,D> where N : Sub<Output = N> + Mul<Output = N>, D : Lcm + Into<N>  {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::TopHeavy(self_numerator - other_numerator,self_denominator);
                }

                let denominator = self_denominator.lcm(&other_denominator);
                let numerator = self_numerator * (denominator / self_denominator).into() - other_numerator * (denominator / other_denominator).into();

                Fraction::TopHeavy(numerator,denominator)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,
            
        }
    }
}

impl<N,D> Mul for Fraction<N,D> where N : Mul<Output = N>,D : Lcm + Into<N> {
    type Output = Self;
    fn mul(self,other : Self) -> Self {
        match (self,other) {
            (Fraction::TopHeavy(self_numerator , self_denominator),Fraction::TopHeavy(other_numerator , other_denominator)) => { 
                let lcm : D = self_denominator.lcm(&other_denominator);
                
                let factor_self = lcm / self_denominator;
                let factor_other = lcm / other_denominator;
                
                let numerator : N = self_numerator * factor_self.into() * other_numerator * factor_other.into();
        
                Fraction::TopHeavy(numerator,lcm)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,
            
        }
    }
}

impl<N,D> Div for Fraction<N,D> where Rhs : Mul<Output = Self> {
    type Output = Self;
    fn div(self,other : Self) -> Self {
        self * other.to_inverse()
    }
}

marco_rules! from_primitive_unsigned_ints! {
    ($T : ty) => {
        impl From<$T> for Fraction<$T,u8>{
            fn from(value : $T) -> Self {
                if value == 0 {
                    Fraction::NaN
                }
                else {
                    Fraction::new_unchecked(numerator,1);
                }
            }
        }
    }  
};

from_primitive_unsigned_ints!(u8);
from_primitive_unsigned_ints!(u16);
from_primitive_unsigned_ints!(u32);
from_primitive_unsigned_ints!(u64);


/// TODO TEST ALL METHODS
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
}