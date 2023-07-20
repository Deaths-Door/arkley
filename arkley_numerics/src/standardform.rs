use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};

use std::cmp::{max,min};
use std::num::ParseFloatError;

use arkley_traits::{Power,Abs,Log};

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
        Self { mantissa , exponent }.adjust()
    }

    fn adjust(mut self) -> Self {
        if !(self.mantissa >= 1.0 && self.mantissa <= 10.0) || !(self.mantissa >= -10.0  && self.mantissa <= -1.0) {
            let abs = self.mantissa.absolute();
            let log = abs.log_with_base(10.0).unwrap_or(0.0).ceil();
            
            self.mantissa /= 10.0.to_the_power_of(log);
            self.exponent = log as i8;

            if self.mantissa < 0.0 {
                self.mantissa = -self.mantissa;
            }
            else if self.mantissa > 0.0 && self.mantissa <= 1.0 {
                self.mantissa *= 10.0;
                self.exponent -= 1;
            }   
        };

        self
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
    pub const fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }
        
    /// Returns the string representation of the number in engineering notation.
    pub const fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }

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

macro_rules! operations {
    ($trait:ident |)
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
        self.exponent += other.exponent
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
        self.exponent /= other.exponent
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
                    *self = *self + rhs;
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
                    *self = *self - rhs;
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
                    *self = *self * rhs;
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
                    *self = *self / rhs;
                }
            }
        )*
    };
}

operation_primitives!(add => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
operation_primitives!(sub => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
operation_primitives!(mul => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
operation_primitives!(div => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);