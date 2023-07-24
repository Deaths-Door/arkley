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


impl<N,D> Zero for Fraction<N,D> where N : PartialEq + Sized + From<u8> , D : PartialEq + Sized + From<u8> {
    fn zero() -> Self {
        Fraction::new_unchecked(0_u8.into(),1_u8.into())
    }
}

/*
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
impl<N,D> Lcm for Fraction<N,D> where Self : Gcd + Div<Output = Self> + Mul<Output = Self> {}*/
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