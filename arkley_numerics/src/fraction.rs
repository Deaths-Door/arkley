use std::ops::{Add,Sub,Mul,Div,Neg};

use std::cmp::Ordering;

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
#[derive(Debug,PartialEq,Clone)]
pub enum Fraction<T> where T : ArithmeticCore + Clone {
    /// Represents an undefined or "Not a Number" fraction.
    NaN,
    /// Represents positive infinity.
    PositiveInfinity,
    /// Represents negative infinity.
    NegativeInfinity,
    /// Represents a proper fraction with a numerator and a denominator.
    Proper(T, T),
}

impl<T> Fraction<T> where T : ArithmeticCore + Clone {
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

impl<T> Fraction<T> where T : ArithmeticCore + Clone {
    /// Returns a new `Fraction` instance that represents the inverse of the current fraction.
    ///
    /// If the current fraction is in the `Fraction::Proper` variant, this method swaps the
    /// numerator and denominator to create the inverse fraction. For other variant types, it returns
    /// `NaN` for `NaN`, swaps `PositiveInfinity` to `NegativeInfinity`, and vice versa.
    pub fn as_inverse(&self) -> Fraction<T> {
        match self {
            Fraction::Proper(numerator,denominator) => Fraction::new_unchecked(denominator.clone(),numerator.clone()),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity => Fraction::NegativeInfinity,
            Fraction::NegativeInfinity => Fraction::PositiveInfinity,
        }
    }
}

impl<T> Fraction<T> where T : ArithmeticCore + PartialOrd + Clone {
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

impl<T> Fraction<T> where T : ArithmeticCore + PartialOrd + Clone {
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
        let gcd = numerator.clone().gcd(denominator.clone());
        let n = if denominator < T::zero() {
            -numerator / gcd.clone()
        } else {
            numerator / gcd.clone()
        };

        let d = if denominator < T::zero() {
            -denominator / gcd
        } else {
            denominator / gcd
        };

        Fraction::Proper(n,d)
    }
}

impl<T> Abs for Fraction<T> where T : ArithmeticCore + Clone{
    // Required method
    fn absolute(self) -> Self {
        match self {
            Fraction::Proper(numerator,denominator) => Fraction::new_unchecked(numerator.absolute(),denominator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity | Fraction::NegativeInfinity  => Fraction::PositiveInfinity,
        }   
    }
}

impl<T> std::fmt::Display for Fraction<T> where T : ArithmeticCore + std::fmt::Display + Clone{
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

impl<T> Neg for Fraction<T> where T: ArithmeticCore + Clone{
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

impl<T> Add for Fraction<T> where T : ArithmeticCore + PartialEq + PartialOrd + Clone{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::Proper(self_numerator , self_denominator),Fraction::Proper(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::new_unchecked(self_numerator + other_numerator,self_denominator);
                }
                
                let n1 = self_numerator * other_denominator.clone();
                let n2 = other_numerator * self_denominator.clone();
                let n = n1 + n2;

                let d = self_denominator * other_denominator;

                Fraction::new_unchecked_reduced(n,d)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<T> Sub for Fraction<T> where T : ArithmeticCore + PartialEq + PartialOrd + Clone{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::Proper(self_numerator , self_denominator),Fraction::Proper(other_numerator , other_denominator)) => {                
                if self_denominator == other_denominator {
                    return Fraction::Proper(self_numerator - other_numerator,self_denominator);
                }

                let n1 = self_numerator * other_denominator.clone();
                let n2 = other_numerator * self_denominator.clone();
                let n = n1 - n2;

                let d = self_denominator * other_denominator;

                Fraction::new_unchecked_reduced(n,d)
            }
            (Fraction::NaN,_) | (_,Fraction::NaN) => Fraction::NaN,
            (Fraction::PositiveInfinity,_) | (_,Fraction::PositiveInfinity) => Fraction::PositiveInfinity,
            (Fraction::NegativeInfinity,_) | (_,Fraction::NegativeInfinity) => Fraction::NegativeInfinity,    
        }
    }
}

impl<T> Mul for Fraction<T> where T : ArithmeticCore + PartialOrd + Clone{
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

impl<T> Div for Fraction<T> where T : ArithmeticCore + PartialOrd + Clone {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.as_inverse()
    }
}

impl<T> PartialOrd for Fraction<T> where T: ArithmeticCore + PartialOrd + Clone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = match (self, other) {
            (Fraction::NaN, Fraction::NaN) => Ordering::Equal,
            (Fraction::NaN, _) => Ordering::Less,
            (_, Fraction::NaN) => Ordering::Greater,

            (Fraction::PositiveInfinity, Fraction::PositiveInfinity) => Ordering::Equal,
            (Fraction::PositiveInfinity, Fraction::NegativeInfinity) => Ordering::Greater,
            (Fraction::NegativeInfinity, Fraction::PositiveInfinity) => Ordering::Less,
            (Fraction::NegativeInfinity, Fraction::NegativeInfinity) => Ordering::Equal,

            (Fraction::PositiveInfinity, Fraction::Proper(_, _)) => Ordering::Greater,
            (Fraction::NegativeInfinity, Fraction::Proper(_, _)) => Ordering::Less,

            (Fraction::Proper(_, _), Fraction::PositiveInfinity) => Ordering::Less,
            (Fraction::Proper(_, _), Fraction::NegativeInfinity) => Ordering::Greater,
            (
                Fraction::Proper(n1,d1),
                Fraction::Proper(n2,d2),
            ) => match d1.is_zero() && d2.is_zero() {
                true => Ordering::Equal,
                false => {
                    let lhs = n1.clone() * d2.clone();
                    let rhs = n2.clone() * d1.clone();

                    lhs.partial_cmp(&rhs).unwrap()
                }
            } /*{
                if d1.is_zero() && d2.is_zero() {
                    Ordering::Equal
                }
                else {
                    let lhs_value = *n1 * *d2;
                    let rhs_value = *n2 * *d1;

                    if lhs_value == rhs_value {
                        Ordering::Equal
                    } else if lhs_value < rhs_value {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }*/
        };
        
        Some(ordering)
    }
}

macro_rules! primitives {
    (form => $($t:ty),*) => {
        $(
            impl<T> From<$t> for Fraction<T> where T : ArithmeticCore + Clone, $t : Into<T> {
                fn from(value: $t) -> Self {
                    Fraction::new_unchecked(value.into(), 1.into())
                }
            }
        )*
    };

    (try_from => $($t:ty),*) => {
        $(
            impl TryFrom<&str> for Fraction<$t> {
                type Error = std::num::ParseIntError;
                fn try_from(value : &str) -> Result<Self, Self::Error> {
                    match value.find('/') {
                        None => value.parse::<$t>().and_then(|number| Ok(number.into())),
                        Some(index) => {
                            let n : $t = value[..index].parse::<$t>()?;
                            let d : $t = value[index + 1..].parse::<$t>()?;
                            Ok(Fraction::new(n,d))
                        } 
                    }
                }
            }
        )*
    };

    (add; $($t:ty),*) => {
        $(
            impl<T> Add<$t> for Fraction<T> where T : ArithmeticCore + PartialEq + PartialOrd + Clone, $t : Into<Self> {
                type Output = Self;

                fn add(self, other: $t) -> Self::Output {
                    let rhs : Self = other.into();
                    self + rhs
                }
            }
        )*
    };

    (sub; $($t:ty),*) => {
        $(
            impl<T> Sub<$t> for Fraction<T> where T : ArithmeticCore + PartialEq + PartialOrd + Clone, $t : Into<Self> {
                type Output = Self;

                fn sub(self, other: $t) -> Self::Output {
                    let rhs : Self = other.into();
                    self - rhs
                }
            }
        )*
    };

    (div; $($t:ty),*) => {
        $(
            impl<T> Div<$t> for Fraction<T> where T : ArithmeticCore + PartialOrd + Clone, $t : Into<Self> {
                type Output = Self;

                fn div(self, other: $t) -> Self::Output {
                    let rhs : Self = other.into();
                    self / rhs
                }
            }
        )*
    };

    (mul; $($t:ty),*) => {
        $(
            impl<T> Mul<$t> for Fraction<T> where T : ArithmeticCore + PartialOrd + Clone, $t : Into<Self> {
                type Output = Self;

                fn mul(self, other: $t) -> Self::Output {
                    let rhs : Self = other.into();
                    self * rhs
                }
            }
        )*
    };

    (operations => $($t:ty),*) => {
        $(
            primitives!(add; $t);
            primitives!(sub; $t);
            primitives!(mul; $t);
            primitives!(div; $t);
        )*
    };

    (eq => $($t:ty),*) => {
        $(
            impl<T> PartialEq<$t> for Fraction<T> where T : ArithmeticCore + Clone, $t : Into<Self> {
                fn eq(&self,other: &$t) -> bool {
                    let rhs : Self = (*other).into();
                    *self == rhs
                }
            }
        )*
    };

    (ord => $($t:ty),*) => {
        $(
            impl<T> PartialOrd<$t> for Fraction<T> where
                Self : PartialOrd,
                T: ArithmeticCore + Clone,
                $t : Into<Fraction<T>> {
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    let rhs : Self = (*other).into();
                    self.partial_cmp(&rhs)
                }
            }
        )*
    };

}

primitives!(form => i8,i16,i32,i64);
primitives!(try_from => i8,i16,i32,i64);
primitives!(operations => i8, i16, i32, i64);
primitives!(eq => i8,i16,i32,i64);
primitives!(ord => i8,i16,i32,i64);

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
    fn new_fraction() {
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
    fn fraction_equality() {
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
    fn abs() {
        let fraction1 = Fraction::new(-1, 2);
        let abs_fraction1 = fraction1.absolute();
        assert_eq!(abs_fraction1, Fraction::new(1, 2));

        let fraction2 = Fraction::new(3, 4);
        let abs_fraction2 = fraction2.absolute();
        assert_eq!(abs_fraction2, fraction2); // Absolute value of a positive fraction is the fraction itself
    }

    #[test]
    fn partial_ord_integers_vs_fractions() {
        assert!(Fraction::new(1,1) >= Fraction::new(1,2));

        // Test integers vs fractions using PartialOrd
        let integer_values : [i32;4] = [1, 3, -2, 5];
        let fraction_values = [
            Fraction::new(1, 2),
            Fraction::new(-3, 4),
            Fraction::new(5, 1),
            Fraction::new(-6, 2),
        ];
        let results = [
            Some(Ordering::Less), // 1/2 is less than 1
            Some(Ordering::Less), // -3/4 is less than 3
            Some(Ordering::Greater), // 5/1 is greater than -2
            Some(Ordering::Less)   // -6/2 is less than 5
        ];

        for x in 0..4 {
            println!("f1 = {:?}",fraction_values[x]);
            println!("i2 = {:?}",integer_values[x]);
            println!("-------------");

            assert_eq!(fraction_values[x].partial_cmp(&integer_values[x]), results[x]);
        }
    }

    #[test]
    fn partial_eq_integers_vs_fractions() {
        // Test integers vs fractions using PartialEq
        let integer_values = [2, -3, 5, -1, 7];
        let fraction_values = [
            Fraction::new(2, 1),
            Fraction::new(-3, 1),
            Fraction::new(5, 1),
            Fraction::new(-1, 1),
            Fraction::new(7, 1),
        ];

        for x in 0..5 {
            assert_eq!(fraction_values[x],integer_values[x]);
        }
    }

    // Test for i8
    #[test]
    fn try_from_i8() {
        let input = "42";
        let result = Fraction::<i8>::try_from(input);
        assert!(result.is_ok());

        // Add more test cases specific to i8 if needed
    }

    // Test for i16
    #[test]
    fn try_from_i16() {
        let input = "12345";
        let result = Fraction::<i16>::try_from(input);
        assert!(result.is_ok());

        // Add more test cases specific to i16 if needed
    }

    // Test for i32
    #[test]
    fn try_from_i32() {
        let input = "100/25";
        let result = Fraction::<i32>::try_from(input);
        assert!(result.is_ok());

        // Add more test cases specific to i32 if needed
    }

    // Test for i64
    #[test]
    fn try_from_i64() {
        let input = "500/125";
        let result = Fraction::<i64>::try_from(input);
        assert!(result.is_ok());

        // Add more test cases specific to i64 if needed
    }

    // Additional Test Cases
    #[test]
    fn try_from_negative() {
        let input = "-42";
        let result = Fraction::<i32>::try_from(input);
        assert!(result.is_ok());

        // Add more test cases with negative numbers if needed
    }

    #[test]
    fn try_from_negative_fraction() {
        let input = "-1/2";
        let result = Fraction::<i64>::try_from(input);
        assert!(result.is_ok());
        // Add more test cases with negative fractions if needed
    }

    #[test]
    fn try_from_invalid_input() {
        let input = "invalid"; // Not a valid integer or fraction
        let result = Fraction::<i16>::try_from(input);
        assert!(result.is_err());

        // Add more test cases with invalid input strings if needed
    }
}