use std::ops::{Add,Sub,Mul,Div,Rem,Neg,AddAssign,SubAssign,MulAssign,DivAssign};

use arkley_traits::{Gcd,Zero,Lcm,Abs,Power};

/// Approximates the fractional representation of a floating-point number with a given tolerance.
///
/// `Note` : at time of writing this doesnt work at all
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
    if value.is_nan() { 
        return Fraction::NaN 
    }

    if value.is_infinite() { 
        return match value.is_sign_negative() {
            true => Fraction::PositiveInfinity,
            false => Fraction::NegativeInfinity
        };
    }

    let mut precision = 0;
    let mut new = value;

    let tenth : f64 = 10.0;

    while (new.floor() - new).abs() < tolerance {
        precision += 1;
        new = value * tenth.powi(precision);
        
        if value.is_infinite() { 
            return match value.is_sign_negative() {
                true => Fraction::PositiveInfinity,
                false => Fraction::NegativeInfinity
            };
        }
    }
    let n = new.ceil() as i64;
    let d = tenth.powi(precision).ceil() as i64;
    
    Fraction::new_unchecked_reduced(n,d)
}

/// The `Fraction` struct represents a fraction with a numerator and denominator.
///
/// `Note` : Avoid using f32 and f64 as N or D as it leads to errors with current implementation but works for intergers
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
    pub const fn denominator(&self) -> Option<&D>{
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

impl<N,D> Fraction<N,D> where N : Zero + Gcd + Div<N,Output = N> + Neg<Output = N> + PartialOrd,
D : Zero + Div<N,Output = D> + Neg<Output = D> + PartialOrd + Into<N> + Copy { 
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

        if numerator > N::zero() {
            Fraction::PositiveInfinity
        }
        else {
            Fraction::NegativeInfinity
        }
    }
}

impl<N,D> Fraction<N,D> where N : Zero + Gcd + Div<N,Output = N> + Neg<Output = N>,
                            D : Zero + Div<N,Output = D> + Neg<Output = D> + PartialOrd + Into<N> + Copy {
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
        let n = if denominator < D::zero() {
            -numerator / gcd
        } else {
            numerator / gcd
        };

        let d = if denominator < D::zero() {
            -denominator / gcd
        } else {
            denominator / gcd
        };
        Fraction::TopHeavy(n,d)
    }
}

impl<N,D> Abs for Fraction<N,D> where N : Abs {
    // Required method
    fn absolute(self) -> Self {
        match self {
            Fraction::TopHeavy(numerator,denominator) => Fraction::new_unchecked(numerator.absolute(),denominator),
            Fraction::NaN => Fraction::NaN,
            Fraction::PositiveInfinity | Fraction::NegativeInfinity  => Fraction::PositiveInfinity,
        }   
    }
}

impl<N,D> Zero for Fraction<N,D> where N : PartialEq + Sized + From<u8> , D : PartialEq + Sized + From<u8> {
    fn zero() -> Self {
        Fraction::new_unchecked(0_u8.into(),1_u8.into())
    }
}

impl<N, D> Rem for Fraction<N, D> where N : Rem<N,Output = N>  {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        match (self,other) {
            (Fraction::TopHeavy(n1,d1),Fraction::TopHeavy(n2,_)) => {
                Fraction::new_unchecked(n1 % n2,d1)
            },
            _ => Fraction::NaN
        }
    }
}

impl<N,D> Gcd for Fraction<N,D> where Self : Zero + Rem<Output = Self> + Sized + Copy {}
impl<N,D> Lcm for Fraction<N,D> where Self : Gcd + Div<Output = Self> + Mul<Output = Self> {}
/*
impl<'a, N, D> TryInto<f64> for &'a Fraction<N, D> where f64: From<N> {
    type Error = (<f64 as TryInto<<N as TryInto<f64>>::Error>>::Error);
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Fraction::TopHeavy(numerator,denominator) => {
                let n : f64 = numerator.try_into().map_err(|_| ())?;
                let d : f64 = denominator.try_into().map_err(|_| ())?;
                Ok(n / d)
            } ,
            Fraction::NaN => Ok(f64::NAN),
            Fraction::PositiveInfinity => Ok(f64::INFINITY),
            Fraction::NegativeInfinity => Ok(f64::NEG_INFINITY),
        }
    }
}
*/
/*
impl<N,D> Fraction<N,D> where N : TryInto<f64>, D : TryInto<f64> { // N : Div<D> + Into<f64> , f64: TryFrom<<N as Div<D>>::Output> + Into<f64> {
    /// Converts the fraction to its decimal representation (`f64`).
    ///
    /// # Returns
    ///
    /// - `Ok(value)`: If the conversion is successful, returns the decimal representation as `f64`.
    /// - `Err(err)`: If an error occurs during the conversion, returns the actual error.
    ///
    pub fn as_f64(self) -> Result<f64,()>/*Result<f64,<f64 as TryFrom<<N as Div<D>>::Output>>::Error>*/ {
        match self {
            Fraction::TopHeavy(numerator,denominator) => {
                let n : f64 = numerator.try_into().map_err(|_| ())?;
                let d : f64 = denominator.try_into().map_err(|_| ())?;
                Ok(n / d)
            } ,
            Fraction::NaN => Ok(f64::NAN),
            Fraction::PositiveInfinity => Ok(f64::INFINITY),
            Fraction::NegativeInfinity => Ok(f64::NEG_INFINITY),
        }
    }
}*/

impl<N,D> std::fmt::Display for Fraction<N,D> where N : std::fmt::Display , D : std::fmt::Display{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Fraction::TopHeavy(numerator,denominator) => {
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
            impl From<$t> for Fraction<$t,$t> {
                fn from(value: $t) -> Self {
                    Fraction::new_unchecked(value, 1)
                }
            }
        )*
    }
}

from_ints!(u8, i8, u16, i16, u32, i32, u64, i64);

impl From<f64> for Fraction<i64,i64> {
    fn from(value: f64) -> Self {
        from_f64(value,f64::EPSILON)
    }
}

macro_rules! operation_primitives {
    ($($t : ty),*) => {
        $(
            impl<N,D> Add<$t> for Fraction<N,D> where Self :  Add<Self,Output = Self> , $t : Into<Self> {
                type Output = Self;
                fn add(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self + rhs
                }
            }

            impl<N,D> Sub<$t> for Fraction<N,D> where  Self : From<$t> + Sub<Self,Output = Self>{
                type Output = Self;
                fn sub(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self - rhs
                }
            }

            
            impl<N,D> Mul<$t> for Fraction<N,D> where Self: From<$t> + Mul<Self,Output = Self>{
                type Output = Self;
                fn mul(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self * rhs
                }
            }

            impl<N,D> Div<$t> for Fraction<N,D> where Self : From<$t> + Div<Self,Output = Self> {
                type Output = Self;

                fn div(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self / rhs
                }
            }
        )*
    };
}

operation_primitives!(u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);

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

/*
impl<N,D> PartialEq<f64> for Fraction<N,D> where Self : Copy , D : TryInto<f64> , N : TryInto<f64> {
    fn eq(&self, other: &f64) -> bool {
        match self.as_f64() {
            Err(_) => false,
            Ok(value) => (value - *other).abs() < f64::EPSILON
        }
    }
}*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_unchecked(){
        let fraction1 = Fraction::new_unchecked(1, 2);
        let fraction2 = Fraction::new_unchecked(3, 4);
        let fraction3 = Fraction::new_unchecked(6,8);

        assert_eq!(fraction1,Fraction::TopHeavy(1,2));
        assert_eq!(fraction2,Fraction::TopHeavy(3,4));
        assert_ne!(fraction3,fraction1);
    }

    #[test]
    fn new_unchecked_reduced(){
        let fraction1 = Fraction::new_unchecked_reduced(4,8);
        let fraction2 = Fraction::new_unchecked_reduced(3, 4);
        let fraction3 = Fraction::new_unchecked_reduced(6,8);

        assert_eq!(fraction1,Fraction::TopHeavy(1,2));
        assert_eq!(fraction2,Fraction::TopHeavy(3,4));
        assert_ne!(fraction3,Fraction::NaN);
    }

    #[test]
fn test_new_fraction() {
    // Test case 1: Create a new fraction with a reduced numerator and denominator
    let fraction1 = Fraction::new(2, 4);
    assert_eq!(fraction1, Fraction::TopHeavy(1, 2));

    // Test case 2: Create a new fraction with an unreduced numerator and denominator
    let fraction2 = Fraction::new(5, 15);
    assert_eq!(fraction2, Fraction::TopHeavy(1, 3));

    // Test case 3: Create a new fraction with a negative numerator and denominator
    let fraction3 = Fraction::new(-10, -20);
    assert_eq!(fraction3, Fraction::TopHeavy(1, 2));

    // Test case 4: Create a new fraction with a zero numerator and non-zero denominator
    let fraction4 = Fraction::new(0, 5);
    assert_eq!(fraction4, Fraction::TopHeavy(0, 1));

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
    assert_eq!(fraction8,Fraction::TopHeavy(5, 2));
    let fraction9 = Fraction::new(3, 4);
    assert_eq!(fraction9,Fraction::TopHeavy(3, 4));
}

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

    #[test]
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
    }
    
    #[test]
    fn neg(){
        let fraction1 = Fraction::new(1,2);
        let result = -fraction1;
        assert_ne!(fraction1,result);
        assert_eq!(result,Fraction::new(-1,2));
    }

    #[test]
    fn neg_f64(){
        let fraction1 = Fraction::new(1.0,2.0);
        let result = -fraction1;
        assert_ne!(fraction1,result);
        assert_eq!(result,Fraction::new(-1.0,2.0));
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

    #[test]
    fn test_zero_trait() {
        let fraction1 = Fraction::new(0, 7);
        let fraction2 = Fraction::new(3, 1);
        let fraction3 = Fraction::new(4, 0);

        assert!(fraction1.is_zero());
        assert!(!fraction2.is_zero());
        assert!(!fraction3.is_zero()); // Fractions with a denominator of zero are not considered zero
    }
    #[test]
    fn test_rem() {
        let fraction1 = Fraction::TopHeavy(10, 4);
        let fraction2 = Fraction::TopHeavy(3, 4);
        let remainder = fraction1 % fraction2;
        assert_eq!(remainder, Fraction::new(1, 4));
    }

    /*#[test]
    fn test_gcd() {
        let fraction1 = Fraction::new(4, 6);
        let fraction2 = Fraction::new(3, 9);
        let gcd_result = fraction1.gcd(fraction2);
        assert_eq!(gcd_result, Fraction::new(1, 3));
    }

    #[test]
    fn test_lcm() {
        let fraction1 = Fraction::new(3, 5);
        let fraction2 = Fraction::new(2, 3);
        let lcm_result = fraction1.lcm(fraction2);
        assert_eq!(lcm_result, Fraction::new(2, 5));
    }*/
/*
    #[test]
    fn as_f64(){
        let fraction1 = Fraction::new(2,4);
        let fraction2 = Fraction::new(1,4);
        let result1 = fraction1.as_f64().unwrap();
        let result2 = fraction2.as_f64().unwrap();

        assert_eq!(result1,0.5);
        assert_eq!(fraction1,result1);
        assert_eq!(fraction2,0.25);
        assert_eq!(fraction1,result2);
    }*/
}