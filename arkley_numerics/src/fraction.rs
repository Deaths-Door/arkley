use arkley_traits::{
    ArithmeticCore
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
#[derive(Debug,PartialEq)]
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
    /// Returns `Some` if the fraction is in the `Fraction::TopHeavy` variant, otherwise returns `None`.
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
    /// Returns `Some` if the fraction is in the `Fraction::TopHeavy` variant, otherwise returns `None`.
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
    /// If the current fraction is in the `Fraction::TopHeavy` variant, this method swaps the
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

impl<T> Fraction<T> where T : ArithmeticCore {
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