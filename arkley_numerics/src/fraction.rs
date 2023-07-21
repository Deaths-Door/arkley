
use crate::Numeric;

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
    Fraction::new_unchecked(h1,k1)
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

impl<N,D> TryFrom<&str> for Fraction<N,D> where N : Numeric , D : Numeric {
    type Error = ();

    fn from(s: &str) -> Self {
        match s {
            "NaN" => Ok(Fraction::NaN),
            "∞" | "+∞" | "inf" | "infinity" | "+inf" => Ok(Fraction::PositiveInfinity),
            "-∞" | "-inf" | "-infinity" => Ok(Fraction::NegativeInfinity),
            _ => match s.find('/') {
                None => Err(()),
                Some(index) => {
                    let numerator = N::try_from(&s[..index]).map_err(|_| ())?;
                    let denominator = D::try_from(&s[index + 1..]).map_err(|_| ())?;
                    Ok(Fraction::Fraction(numerator, denominator))
                }
            },
        }
    }
}

from_ints!(u8, i8, u16, i16, u32, i32, u64, i64);