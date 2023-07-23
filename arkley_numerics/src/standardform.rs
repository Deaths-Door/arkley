use crate::Decimal;

use std::num::ParseFloatError;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number (using a underlying fraction for zero precision loss)
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
/// 'Note' : TODO use Decimal<i8,i8> when and if possible to reduce memory usage
/// `Note` : TODO restrict decimal to be positive always
pub struct StandardForm  {
    mantissa : Decimal,
    exponent : i8
}

impl StandardForm {
    /// Creates a new instance of StandardForm with the given mantissa and exponent
    pub fn new(mantissa : Decimal,exponent : i8) -> Self {
        let mut instance = Self { mantissa , exponent };
        instance.adjust();
        instance
    }
    
    fn adjust(&mut self) {
        todo!("IMPLEMENT THIS RIGHT NOWs")
        //if self.mantissa >= 1 && self.mantissa <= 10 {
            //..
        //}
    }

    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    pub const fn mantissa(&self) -> &Decimal {
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

impl std::fmt::Display for StandardForm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.exponent > 4 {
            return write!(f,"{}",self.to_scientific_notation());
        };

        write!(f,"{}",(self.mantissa * 10.0).to_the_power_of(self.exponent as f64))
    }
}


/*use std::ops::{Add,Sub,Mul,Div,Rem,AddAssign,SubAssign,MulAssign,DivAssign};

use std::cmp::{max,min};
use std::num::ParseFloatError;

use arkley_traits::{Power,Gcd,Lcm,Zero};
*/
/*
//use crate::Numeric;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number (using a underlying fraction for zero precision loss)
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
/// 'Note' : TODO implement Numeric for it and see if fraction or decimal is needed for it to retain precision
#[derive(Debug,PartialEq,Clone,Copy)]
pub struct StandardForm  {
    mantissa : Fraction<i8,i8>,
    exponent : i8
}

impl StandardForm {*/
 /*   /// Creates a new instance of StandardForm with the given mantissa and exponent
    pub fn new(mantissa : Fraction<i8,i8>,exponent : i8) -> Self {
        let mut instance = Self { mantissa , exponent };
        instance.adjust();
        instance
    }

    fn in_range(&self) -> bool {
        (self.mantissa >= 1.0 && self.mantissa <= 10.0) || (self.mantissa >= -10.0  && self.mantissa <= -1.0)
    }

    fn adjust(&mut self) {
        if !self.in_range() {
            let abs = self.mantissa.abs();
            let log = abs.log10().ceil();
            
            self.mantissa /= 10.0f64.powf(log);
            self.exponent = log as i8;

            if self.mantissa < 0.0 {
                self.mantissa = -self.mantissa;
            }
            else if self.mantissa > 0.0 && self.mantissa < 1.0 {
                self.mantissa *= 10.0;
                self.exponent -= 1;
            }   
        };
    }
*/
/*
macro_rules! from_primitives {
    ($($t : ty),*) =>{
        $(
            impl From<$t> for StandardForm {
                fn from(value: $t) -> StandardForm {
                    StandardForm::new(value as f64,1)
                }
            }
        )*
    }
}

from_primitives!(u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);

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

impl Rem<StandardForm> for StandardForm {
    type Output = StandardForm;

    fn rem(self, other: StandardForm) -> StandardForm {
        let division_result = self / other;
        self - division_result * other
    }
}

impl Add for StandardForm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_power = max(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) + other.mantissa * 10.0.to_the_power_of((other.exponent - max_power) as f64);
        StandardForm::new(num_sum, max_power)
    }
}

impl AddAssign for StandardForm {
    fn add_assign(&mut self, other: Self) {
        let max_power = max(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) + other.mantissa * 10.0.to_the_power_of((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();

    }
}

impl Sub for StandardForm {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let max_power = min(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) - other.mantissa * 10.0.to_the_power_of((other.exponent - max_power) as f64);

        StandardForm::new(num_sum, max_power)
    }
}

impl SubAssign for StandardForm {
    fn sub_assign(&mut self, other: Self) {
        let max_power = min(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) - other.mantissa * 10.0.to_the_power_of((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();
    }
}

impl Mul for StandardForm {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        StandardForm::new(self.mantissa * other.mantissa,self.exponent + other.exponent)
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

macro_rules! operation_primitives {
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
                    let max_power = max(self.exponent, rhs.exponent);
                    let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) + rhs.mantissa * 10.0.to_the_power_of((rhs.exponent - max_power) as f64);
            
                    self.mantissa = num_sum;
                    self.exponent = max_power;
                    self.adjust();
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

                    let max_power = min(self.exponent, rhs.exponent);
                    let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) - rhs.mantissa * 10.0.to_the_power_of((rhs.exponent - max_power) as f64);

                    self.mantissa = num_sum;
                    self.exponent = max_power;
                    self.adjust();
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
                    self - rhs
                }
            }
            
            impl MulAssign<$t> for StandardForm {
                fn mul_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    self.mantissa *= rhs.mantissa;
                    self.exponent += rhs.exponent;
                    self.adjust();
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
                    self - rhs
                }
            }
            
            impl DivAssign<$t> for StandardForm {
                fn div_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    self.mantissa /= rhs.mantissa;
                    self.exponent /= rhs.exponent;
                    self.adjust();
                }
            }
        )*
    };
}

operation_primitives!(add => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
operation_primitives!(sub => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
operation_primitives!(mul => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
operation_primitives!(div => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
*/
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignment_issue() {
        let sf1 = StandardForm::new(1.0,5);
        assert_eq!(*sf1.mantissa(),1.0);
        assert_eq!(*sf1.exponent(),5);
    }
     // Helper function to create a StandardForm instance from mantissa and exponent
     fn standard_form(mantissa: f64, exponent: i8) -> StandardForm {
        StandardForm { mantissa, exponent }
    }

    #[test]
    fn addition() {
        // Test addition between StandardForm instances
        let a = standard_form(1.2, 3);
        let b = standard_form(3.4, 2);
        let result = a + b;
        assert_eq!(result, 1540.into());
    }

    #[test]
    fn addition_u8() {
        // Test addition with u8
        let a = standard_form(1.0, 1);
        let b = 2u8;
        let result = a + b;
        assert_eq!(result, 12.into());
    }

    #[test]
    fn test_subtraction() {
        // Test subtraction between StandardForm instances
        let a = standard_form(4.6, 2);
        let b = standard_form(3.4, 2);
        let result = a - b;
        assert_eq!(result, 120.into());
    }

    #[test]
    fn subtraction_u8() {
        // Test subtraction with u8
        let a = standard_form(21.0, 1);
        let b = 2u8;
        let result = a - b;
        assert_eq!(result.mantissa, 1.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn multiplication() {
        // Test multiplication between StandardForm instances
        let a = standard_form(1.2, 3);
        let b = standard_form(3.0, 2);
        let result = a * b;
        assert_eq!(result.mantissa, 3.6);
        assert_eq!(result.exponent, 5);

        // Test multiplication with u8
        let a = standard_form(1.0, 1);
        let b = 2u8;
        let result = a * b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn multiplication_u8() {
        // Test multiplication with u8
        let a = standard_form(1.0, 1);
        let b = 2u8;
        let result = a * b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn division() {
        // Test division between StandardForm instances
        let a = standard_form(4.0, 2);
        let b = standard_form(2.0, 1);
        let result = a / b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);

        // Test division with u8
        let a = standard_form(2.0, 1);
        let b = 2u8;
        let result = a / b;
        assert_eq!(result.mantissa, 1.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn division_u8() {
        // Test division with u8
        let a = standard_form(2.0, 1);
        let b = 2u8;
        let result = a / b;
        assert_eq!(result.mantissa, 1.0);
        assert_eq!(result.exponent, 1);
    }


    #[test]
    fn add_assign() {
        let mut a = standard_form(1.0, 1);
        let b = standard_form(2.0, 1);
        a += b;
        assert_eq!(a.mantissa, 3.0);
        assert_eq!(a.exponent, 1);

        // Test AddAssign with u8
        let mut a = standard_form(1.0, 1);
        let b = 2u8;
        a += b;
        assert_eq!(a.mantissa, 21.0);
        assert_eq!(a.exponent, 1);
    }

    #[test]
    fn add_assign_u8() {
        // Test AddAssign with u8
        let mut a = standard_form(1.0, 1);
        let b = 2u8;
        a += b;
        assert_eq!(a.mantissa, 21.0);
        assert_eq!(a.exponent, 1);
    }
}*/