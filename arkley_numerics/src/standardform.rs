use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};

use std::cmp::{max,min};
use std::num::ParseFloatError;

use arkley_traits::Power;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number 
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
#[derive(Debug,PartialEq)]
pub struct StandardForm  {
    mantissa : f64,
    exponent : i8
}

impl StandardForm {
    /// Creates a new instance of StandardForm with the given mantissa and exponent
    pub fn new(mantissa : f64,exponent : i8) -> Self {
        let mut instance = Self { mantissa , exponent};
        instance.adjust();
        instance
    }


    fn in_range(&self) -> bool {
        (self.mantissa >= 1.0 && self.mantissa <= 10.0) || (self.mantissa >= -10.0 && self.mantissa <= -1.0)
    }

    fn adjust(&mut self) {
        if self.in_range() {
            return;
        }

        match self.mantissa/*.abs()*/ > 0.0 {
            true => while !self.in_range() {
                self.mantissa /= 10.0;
                self.exponent += 1; 
            },
            false => while !self.in_range() {
                self.mantissa *= 10.0;
                self.exponent -= 1; 
            }
        }
    }

    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    pub const fn mantissa(&self) -> &f64 {
        &self.mantissa
    }

    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    pub const fn exponent(&self) -> &i8 {
        &self.exponent
    }

    /// Returns the string representation of the number in scientific notation.
    pub fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }
        
    /// Returns the string representation of the number in engineering notation.
    pub fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }

    /// Converts the `StandardForm` into a decimal floating-point number in base 10.
    /// If successful, it returns the decimal value as an `f64`.
    /// If parsing fails, it returns a `ParseFloatError`.
    pub fn as_decimal(&self) -> Result<f64, ParseFloatError>{
        self.to_engineering_notation().parse()
    }

}

impl Default for StandardForm {
    fn default() -> Self {
        Self { mantissa : 1.0, exponent : 0 }
    }
}

impl std::fmt::Display for StandardForm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.exponent > 4 {
            return write!(f,"{}",self.to_scientific_notation());
        };

        write!(f,"{}",self.mantissa * 10_i32.pow(self.exponent as u32) as f64)
    }
}

impl PartialOrd for StandardForm {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.exponent == other.exponent {
            true => self.mantissa.partial_cmp(&other.mantissa),
            false => self.exponent.partial_cmp(&other.exponent)
        }
    }
}

impl TryFrom<&str> for StandardForm {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(number) => Ok(number.into()),
            Err(_) => if let Some(index) = value.find('e') {
                let m_str : f64 = value[0..index].parse().map_err(|_| ())?;
                let e_str : i8 = value[index + 1..].parse().map_err(|_| ())?;
                Ok(StandardForm::new(m_str,e_str))
            }
            else if let Some(index) = value.find('^') {
                let m_str : f64 = value[0..index - 2].parse().map_err(|_| ())?;
                let e_str : i8 = value[index + 1..].parse().map_err(|_| ())?;
                Ok(StandardForm::new(m_str,e_str))
            }
            else {
                Err(())
            }
        }
    }
}

impl Add for StandardForm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_power = max(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0_f64.to_the_power_of((self.exponent - max_power) as f64) + other.mantissa * 10.0_f64.to_the_power_of((other.exponent - max_power) as f64);
        StandardForm::new(num_sum, max_power)
    }
}

impl AddAssign for StandardForm {
    fn add_assign(&mut self, other: Self) {
        let max_power = max(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0_f64.to_the_power_of((self.exponent - max_power) as f64) + other.mantissa * 10.0_f64.to_the_power_of((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();
    }
}

impl Sub for StandardForm {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_i32.pow((self.exponent - min) as u32) as f64;
        let y = other.mantissa * 10_i32.pow((other.exponent - min) as u32) as f64;

        let result = x - y;
        let rounded = (result * 1.0e6).round() / 1.0e6;

        StandardForm::new(rounded,min)
    }
}

impl SubAssign for StandardForm {
    fn sub_assign(&mut self, other: Self) {
        let max_power = min(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0_f64.to_the_power_of((self.exponent - max_power) as f64) - other.mantissa * 10.0_f64.to_the_power_of((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();
    }
}

impl Mul for StandardForm {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;
        let rounded = (mantissa * 1.0e6).round() / 1.0e6;
        StandardForm::new(rounded,exponent)
    }
}

impl MulAssign for StandardForm {
    fn mul_assign(&mut self, other: Self) {
        self.mantissa *= other.mantissa;
        self.exponent += other.exponent;
        self.adjust();
    }
}

impl Div for StandardForm {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        StandardForm::new(self.mantissa / other.mantissa,self.exponent - other.exponent)
    }
}

impl DivAssign for StandardForm {
    fn div_assign(&mut self, other: Self) {
        self.mantissa /= other.mantissa;
        self.exponent /= other.exponent;
        self.adjust();
    }
}

macro_rules! primitives {
    (form => $($t:ty),*) => {
        $(
            impl From<$t> for StandardForm {
                fn from(value: $t) -> Self {
                    StandardForm::new(value as f64,0)
                }
            }
        )*
    };

    (eq => $($t:ty),*) => {
        $(
            impl PartialEq<$t> for StandardForm {
                fn eq(&self,other: &$t) -> bool {
                    let rhs : Self = (*other).into();
                    *self == rhs
                }
            }
        )*
    };

    (add => $($t : ty),*) => {
        $(
            impl Add<$t> for StandardForm {
                type Output = Self;
                fn add(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self + rhs
                }
            }
            
            impl AddAssign<$t> for StandardForm {
                fn add_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self += rhs;
                }
            }
        )*
    };

    (sub => $($t : ty),*) => {
        $(
            impl Sub<$t> for StandardForm {
                type Output = Self;
                fn sub(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self - rhs
                }
            }
            
            impl SubAssign<$t> for StandardForm {
                fn sub_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self -= rhs;
                }
            }
        )*
    };
    (mul => $($t : ty),*) => {
        $(
            impl Mul<$t> for StandardForm {
                type Output = Self;
                fn mul(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self * rhs
                }
            }
            
            impl MulAssign<$t> for StandardForm {
                fn mul_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self *= rhs;
                }
            }
        )*
    };
    (div => $($t : ty),*) => {
        $(
            impl Div<$t> for StandardForm {
                type Output = Self;
                fn div(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self / rhs
                }
            }
            
            impl DivAssign<$t> for StandardForm {
                fn div_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self /= rhs;
                }
            }
        )*
    };
    (operations => $($t:ty),*) => {
        $(
            primitives!(add => $t);
            primitives!(sub => $t);
            primitives!(mul => $t);
            primitives!(div => $t);
        )*
    }
}

primitives!(operations => i8, i16, i32, i64, u8, u16, u32, u64);
primitives!(form => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(eq => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn assignment_issue() {
        let sf1 = StandardForm::new(1.0,5);
        assert_eq!(*sf1.mantissa(),1.0);
        assert_eq!(*sf1.exponent(),5);
    }

    #[test]
    fn from_u8_standardform(){
        let n = 2u8;
        let r : StandardForm = n.into();

        assert_eq!(r,StandardForm { mantissa : 2.0,exponent : 0 });
    }

    #[test]
    fn test_normalize_with_valid_range() {
        let mut sf = StandardForm::new(2.5, 3);
        sf.adjust();
        assert_eq!(sf.mantissa, 2.5);
        assert_eq!(sf.exponent, 3);
    }

    #[test]
    fn test_normalize_with_invalid_range() {
        let mut sf = StandardForm::new(20.0, 3);
        sf.adjust();
        assert_eq!(sf.mantissa, 2.0);
        assert_eq!(sf.exponent, 4);
    }

    #[test]
    fn test_normalize_with_small_mantissa() {
        let mut sf = StandardForm::new(-0.25, 2);
        sf.adjust();
        assert_eq!(sf.mantissa, -2.5);
        assert_eq!(sf.exponent, 1);
    }

    #[test]
    fn test_normalize_with_large_negative_mantissa() {
        let mut sf = StandardForm::new(-750.0, 4);
        sf.adjust();
        assert_eq!(sf.mantissa, -7.5);
        assert_eq!(sf.exponent, 6);
    }

    #[test]
    fn addition() {
        // Test addition between StandardForm instances
        let a = StandardForm::new(1.2, 3);
        let b = StandardForm::new(3.4, 2);
        let result = a + b;
        assert_eq!(result, StandardForm::new(1.54,3) );
    }

    #[test]
    fn addition_u8() {
        // Test addition with u8
        let a = StandardForm::new(1.0, 1);
        let b = 2u8;
        let result = a + b;
        assert_eq!(result, StandardForm::new(1.2,1));
    }

    #[test]
    fn test_subtraction() {
        // Test subtraction between StandardForm instances
        let a = StandardForm::new(4.6, 2);
        let b = StandardForm::new(3.4, 2);
        let result = a - b;
        assert_eq!(result, StandardForm::new(1.2,2));
    }

    #[test]
    fn subtraction_u8() {
        //210
        let a = StandardForm::new(21.0, 1);
        println!("{a}");
        //2
        let b = 2u8;
        let result = a - b;
        assert_eq!(result.mantissa, 2.18);
        assert_eq!(result.exponent, 2);
    }

    #[test]
    fn multiplication() {
        // Test multiplication between StandardForm instances
        let a = StandardForm::new(1.2, 3);
        let b = StandardForm::new(3.0, 2);
        let result = a * b;
        assert_eq!(result.mantissa, 3.6);
        assert_eq!(result.exponent, 5);
    }

    #[test]
    fn multiplication_u8() {
        // Test multiplication with u8
        let a = StandardForm::new(1.0, 1);        
        let b = 2u8;
        let result = a * b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn division() {
        // Test division between StandardForm instances
        let a = StandardForm::new(4.0, 2);
        let b = StandardForm::new(2.0, 1);
        let result = a / b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn division_u8() {
        // Test division with u8
        let a = StandardForm::new(2.0, 1);
        let b = 2u8;
        let result = a / b;
        assert_eq!(result.mantissa, 1.0);
        assert_eq!(result.exponent, 1);
    }


    #[test]
    fn add_assign() {
        let mut a = StandardForm::new(1.0, 1);
        let b = StandardForm::new(2.0, 1);
        a += b;
        assert_eq!(a.mantissa, 3.0);
        assert_eq!(a.exponent, 1);
    }

    #[test]
    fn add_assign_u8() {
        // Test AddAssign with u8
        let mut a = StandardForm::new(1.0, 1);

        let b = 2u8;

        a += b;
        assert_eq!(a.mantissa, 1.2);
        assert_eq!(a.exponent, 1);
    }

    #[test]
    fn test_partial_cmp_equal() {
        let sf1 = StandardForm::new(1.23, 3);
        let sf2 = StandardForm::new(1.23, 3);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Equal));
    }

    #[test]
    fn test_partial_cmp_greater() {

        //300
        let sf1 = StandardForm::new(3.0, 2);
        // 250
        let sf2 = StandardForm::new(2.5, 2);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Greater));
    }

    #[test]
    fn test_partial_cmp_less() {
        let sf1 = StandardForm::new(2.5, 2);
        let sf2 = StandardForm::new(3.0, 2);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_different_exponents() {
        let sf1 = StandardForm::new(1.5, 2);
        let sf2 = StandardForm::new(1.5, 3);

        // When exponents are different, the comparison is based on the magnitude
        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_zero() {
        let sf1 = StandardForm::new(0.0, 0);
        let sf2 = StandardForm::new(0.0, 0);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Equal));
    }

    #[test]
    fn test_partial_cmp_mixed_sign() {
        let sf1 = StandardForm::new(-1.0, 2);
        let sf2 = StandardForm::new(1.0, 2);

        // Negative numbers are considered less than positive numbers with the same magnitude
        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Less));
    }
}