use std::ops::{Add,Sub,Mul,Div,Neg};

use arkley_traits::{
    ArithmeticCore,
    Abs,
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
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Fraction<T> where T : ArithmeticCore {
    /// Represents an undefined or "Not a Number" fraction.
    NaN,
    /// Represents positive infinity.
    PositiveInfinity,
    /// Represents negative infinity.
    NegativeInfinity,
    /// Represents a proper fraction with a numerator and a denominator.
    Proper(T, T),
}

impl<T> Fraction<T> where T : ArithmeticCore {
    /// Constructs a new `Fraction` instance with the given numerator and denominator.
    ///
    /// # Safety
    ///
    /// This method does not perform any validation or simplification of the fraction.
    /// It assumes that the numerator and denominator are valid and correctly provided. 
    /// `Note` : If these conditions are not method then operations like PartialEq or PartialOrd maybe not function correctly so use this at your own risk. However if operation like + - * / or ^ is performed on self then this will be fixed
    pub const fn new_unchecked(numerator : T,denominator : T) -> Self  {
        Fraction::Proper(numerator,denominator)
    }

     /// Returns an option containing a reference to the numerator of the fraction.
    ///
    /// Returns `Some` if the fraction is in the `Fraction::Proper` variant, otherwise returns `None`.
    pub const fn numerator(&self) -> Option<&T>{
        match self {
            Fraction::Proper(numerator,_) => Some(&numerator),
            Fraction::NaN => None,
            Fraction::PositiveInfinity => None,
            Fraction::NegativeInfinity => None,
        }
    }

    /// Returns an option containing a reference to the denominator of the fraction.
    ///
    /// Returns `Some` if the fraction is in the `Fraction::Proper` variant, otherwise returns `None`.
    pub const fn denominator(&self) -> Option<&T>{
        match self {
            Fraction::Proper(_,denomator) => Some(&denomator),
            Fraction::NaN => None,
            Fraction::PositiveInfinity => None,
            Fraction::NegativeInfinity => None,
        }
    }
}

impl<T> Fraction<T> where T : ArithmeticCore + Copy {
    /// Returns a new `Fraction` instance that represents the inverse of the current fraction.
    ///
    /// If the current fraction is in the `Fraction::Proper` variant, this method swaps the
    /// numerator and denominator to create the inverse fraction. For other variant types, it returns
    /// `NaN` for `NaN`, swaps `PositiveInfinity` to `NegativeInfinity`, and vice versa.
    pub fn as_inverse(&self) -> Fraction<T> {
        match self {
            Fraction::Proper(numerator,denominator) => Fraction::new_unchecked(*denominator,*numerator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}

impl<T> Fraction<T> where T : ArithmeticCore + PartialOrd {
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
    pub fn new(numerator : T,denominator : T) -> Self {
        if !denominator.is_zero(){
            return Fraction::new_unchecked_reduced(numerator,denominator);
        };

        if numerator.is_zero() {
            return Fraction::NaN;
        }

        if numerator > T::zero() {
            Fraction::PositiveInfinity
        }
        else {
            Fraction::NegativeInfinity
        }
    }
}

impl<T> Fraction<T> where T : ArithmeticCore + PartialOrd {
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
    pub fn new_unchecked_reduced(numerator : T,denominator : T) -> Fraction<T> {
        let gcd = numerator.gcd(denominator);
        let n = if denominator < T::zero() {
            -numerator / gcd
        } else {
            numerator / gcd
        };

        let d = if denominator < T::zero() {
            -denominator / gcd
        } else {
            denominator / gcd
        };
        Fraction::Proper(n,d)
    }
}

impl<T> Abs for Fraction<T> where T : ArithmeticCore {
    // Required method
    fn absolute(self) -> Self {
        match self {
            Fraction::Proper(numerator,denominator) => Fraction::new_unchecked(numerator.absolute(),denominator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity | Fraction::NegativeInfinity  => Fraction::PositiveInfinity,
        }   
    }
}

impl<T> std::fmt::Display for Fraction<T> where T : ArithmeticCore + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Fraction::Proper(numerator,denominator) => {
                let ns = numerator.to_string();
                let ds = denominator.to_string();

                let nr = ns.parse::<f64>();
                let dr = ds.parse::<f64>();

                let string = match (nr,dr) {
                    (Ok(_),Ok(_)) => format!("{}/{}",ns,ds),
                    (Ok(_),Err(_)) => format!("{}/({})",ns,ds),
                    (Err(_),Ok(_)) => format!("({})/{}",ns,ds),
                    _ => format!("({})/({})",ns,ds)
                };

                write!(f,"{}",string)
            },
            Fraction::NaN => write!(f,"NaN"),
            Fraction::PositiveInfinity => write!(f,"+∞"),
            Fraction::NegativeInfinity => write!(f,"-∞"),
        }
    }
}

impl<T> Neg for Fraction<T> where T: ArithmeticCore {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Fraction::Proper(numerator,denominator) => Fraction::new_unchecked(-numerator,denominator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}

impl<T> Add for Fraction<T> where T : ArithmeticCore + PartialEq + PartialOrd{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::Proper(self_numerator , self_denominator),Fraction::Proper(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::Proper(self_numerator + other_numerator,self_denominator);
                }
    
                Fraction::new_unchecked_reduced(self_numerator * other_denominator + other_numerator * self_denominator,self_denominator * other_denominator)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<T> Sub for Fraction<T> where T : ArithmeticCore + PartialEq + PartialOrd{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::Proper(self_numerator , self_denominator),Fraction::Proper(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::Proper(self_numerator - other_numerator,self_denominator);
                }

                Fraction::new_unchecked_reduced(self_numerator * other_denominator - other_numerator * self_denominator,self_denominator * other_denominator)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<T> Mul for Fraction<T> where T : ArithmeticCore + PartialOrd {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::Proper(self_numerator , self_denominator),Fraction::Proper(other_numerator , other_denominator)) =>              
                Fraction::new_unchecked_reduced(self_numerator * other_numerator,self_denominator * other_denominator),
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<T> Div for Fraction<T> where T : ArithmeticCore + PartialOrd {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.as_inverse()
    }
}

macro_rules! impl_ints {
    (form; $($t:ty),*) => {
        $(
            impl From<$t> for Fraction<$t> {
                fn from(value: $t) -> Self {
                    Fraction::new_unchecked(value, 1)
                }
            }
        )*
    };;
}

impl_ints!(form; i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_unchecked(){
        let fraction1 = Fraction::new_unchecked(1, 2);
        let fraction2 = Fraction::new_unchecked(3, 4);
        let fraction3 = Fraction::new_unchecked(6,8);

        assert_eq!(fraction1,Fraction::Proper(1,2));
        assert_eq!(fraction2,Fraction::Proper(3,4));
        assert_ne!(fraction3,fraction1);
    }

    #[test]
    fn new_unchecked_reduced(){
        let fraction1 = Fraction::new_unchecked_reduced(4,8);
        let fraction2 = Fraction::new_unchecked_reduced(3, 4);
        let fraction3 = Fraction::new_unchecked_reduced(6,8);

        assert_eq!(fraction1,Fraction::Proper(1,2));
        assert_eq!(fraction2,Fraction::Proper(3,4));
        assert_ne!(fraction3,Fraction::NaN);
    }

    #[test]
    fn test_new_fraction() {
        // Test case 1: Create a new fraction with a reduced numerator and denominator
        let fraction1 = Fraction::new(2, 4);
        assert_eq!(fraction1, Fraction::Proper(1, 2));

        // Test case 2: Create a new fraction with an unreduced numerator and denominator
        let fraction2 = Fraction::new(5, 15);
        assert_eq!(fraction2, Fraction::Proper(1, 3));

        // Test case 3: Create a new fraction with a negative numerator and denominator
        let fraction3 = Fraction::new(-10, -20);
        assert_eq!(fraction3, Fraction::Proper(1, 2));

        // Test case 4: Create a new fraction with a zero numerator and non-zero denominator
        let fraction4 = Fraction::new(0, 5);
        assert_eq!(fraction4, Fraction::Proper(0, 1));

        // Test case 5: Create a new fraction with a non-zero numerator and zero denominator
        let fraction5 = Fraction::new(7, 0);
        assert_eq!(fraction5, Fraction::PositiveInfinity);

        // Test case 6: Create a new fraction with both numerator and denominator as zero
        let fraction6 = Fraction::new(0, 0);
        assert_eq!(fraction6, Fraction::NaN);

        // Test case 7: Create a new fraction with a negative numerator and zero denominator
        let fraction7 = Fraction::new(-3, 0);
        assert_eq!(fraction7, Fraction::NegativeInfinity);

        let fraction8 = Fraction::new(10, 4);
        assert_eq!(fraction8,Fraction::Proper(5, 2));
        let fraction9 = Fraction::new(3, 4);
        assert_eq!(fraction9,Fraction::Proper(3, 4));
    }

    #[test]
    fn addition() {
        // Create fractions
        let fraction1 = Fraction::new(1, 2);
        let fraction2 = Fraction::new(3, 4);

        // Add fractions
        let result = fraction1 + fraction2;

        // Verify the result
        if let Fraction::Proper(numerator, denominator) = result {
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
        if let Fraction::Proper(numerator, denominator) = result {
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
        if let Fraction::Proper(numerator, denominator) = result {
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
        if let Fraction::Proper(numerator, denominator) = result {
            assert_eq!(numerator,2);
            assert_eq!(denominator,1);
        } else {
            assert!(false, "Expected top-heavy fraction");
        }
    }

    #[test]
    fn test_fraction_equality() {
        let fraction1: Fraction<i32> = Fraction::new(2, 4);
        let fraction2: Fraction<i32> = Fraction::new(1, 2);
        let fraction3: Fraction<i32> = Fraction::new(4, 8);
        let fraction4: Fraction<i32> = Fraction::NaN;

        assert_eq!(fraction1, fraction2);  // Fractions with equivalent values should be equal
        assert_eq!(fraction1, fraction3);  // Fractions with equivalent values should be equal
        assert_ne!(fraction1, fraction4);  // Different variants should not be equal
        assert_ne!(fraction2, fraction4);  // Different variants should not be equal
        assert_ne!(fraction3, fraction4);  // Different variants should not be equal
    }

    #[test]
    fn neg(){
        let fraction1 = Fraction::new(1,2);
        let result = -Fraction::new(1,2);
        assert_ne!(fraction1,result);
        assert_eq!(result,Fraction::new(-1,2));
    }

    #[test]
    fn test_abs() {
        let fraction1 = Fraction::new(-1, 2);
        let abs_fraction1 = fraction1.absolute();
        assert_eq!(abs_fraction1, Fraction::new(1, 2));

        let fraction2 = Fraction::new(3, 4);
        let abs_fraction2 = fraction2.absolute();
        assert_eq!(abs_fraction2, fraction2); // Absolute value of a positive fraction is the fraction itself
    }

    /*#[test]
    fn test_from_f64() {
        // Test cases and expected results as Fraction values
        let test_cases = [
            (0.25, Fraction::new(1, 4), 1e-8),
            (0.333333, Fraction::new(1, 3), 1e-8),
            (0.5, Fraction::new(1, 2), 1e-8),
            (0.75, Fraction::new(3, 4), 1e-8),
            (1.0, Fraction::new(1, 1), 1e-8),
            (1.25, Fraction::new(5, 4), 1e-8),
            (1.5, Fraction::new(3, 2), 1e-8),
            (1.75, Fraction::new(7, 4), 1e-8),
            (2.0, Fraction::new(2, 1), 1e-8),
            (2.25, Fraction::new(9, 4), 1e-8),
            (2.5, Fraction::new(5, 2), 1e-8),
            (2.75, Fraction::new(11, 4), 1e-8),
            (3.0, Fraction::new(3, 1), 1e-8),
        ];
    
        for (input,expected_output,_) in test_cases.iter() {
            let result = Fraction::from(*input);
            assert_eq!(result, *expected_output);
        }
    }*/
    
  /*  
      #[test]
    fn neg_f64(){
        let fraction1 = Fraction::new(1.0,2.0);
        let result = -Fraction::new(1.0,2.0);
        assert_ne!(fraction1,result);
        assert_eq!(result,Fraction::new(-1.0,2.0));
    }*/
}