use std::ops::{Add,Sub,Mul,Div,Neg,Rem,AddAssign,SubAssign,MulAssign,DivAssign};

use std::cmp::{max,min};
use crate::{
    utils::{
        Numeric,
        Abs,
        Power,
        Log,
    },
};

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
#[derive(Debug,PartialEq)]
pub struct StandardForm  {
    mantissa : f64,
    exponent : u8
}


impl StandardForm {
    /// Creates a new instance of StandardForm with the given mantissa and exponent
    pub fn new(mantissa : f64,exponent : u8) -> Self {
        Self { mantissa , exponent}.adjust()
    }

    fn adjust(mut self) -> Self {
        if !(self.mantissa >= 1.0 && self.mantissa <= 10.0) || !(self.mantissa >= -10.0  && self.mantissa <= -1.0) {
            let abs = self.mantissa.absolute();
            let log = abs.log_with_base(10.0).unwrap_or(0.0).ceil();
            
            self.mantissa /= 10.0.to_the_power_of(log);
            self.exponent = log as u8;

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
    pub const fn exponent(&self) -> &u8 {
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
}

impl std::fmt::Display for StandardForm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.exponent > 4 {
            return write!(f,"{}",self.to_scientific_notation());
        };

        write!(f,"{}",(self.mantissa * 10.0).to_the_power_of(self.exponent as f64))
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


impl Sub for StandardForm {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let max_power = min(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) - other.mantissa * 10.0.to_the_power_of((other.exponent - max_power) as f64);

        StandardForm::new(num_sum, max_power)
    }
}

impl Mul for StandardForm {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        StandardForm::new(self.mantissa * other.mantissa,self.exponent + other.exponent)
    }
}

impl Div for StandardForm {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        StandardForm::new(self.mantissa / other.mantissa,self.exponent - other.exponent)
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

impl SubAssign for StandardForm   {
    fn sub_assign(&mut self, other: Self) {
        let max_power = min(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0.to_the_power_of((self.exponent - max_power) as f64) - other.mantissa * 10.0.to_the_power_of((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;
    }
}

impl MulAssign for StandardForm  {
    fn mul_assign(&mut self, other: Self) {
        self.mantissa *= other.mantissa;
        self.exponent += other.exponent
    }
}

impl DivAssign for StandardForm{
    fn div_assign(&mut self, other: Self) {
        self.mantissa /= other.mantissa;
        self.exponent -= other.exponent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let sf1 = StandardForm::new(2.0, 5);
        let sf2 = StandardForm::new(2.0, 10);
        let result = sf1 + sf2;
        assert_eq!(result, StandardForm::new(2.00002, 10));
    }

    #[test]
    fn test_sub() {
        let sf1 = StandardForm::new(2.0, 5);
        let sf2 = StandardForm::new(2.0, 10);
        let result = sf1 - sf2;
        assert_eq!(result, StandardForm::new(17.0, 2));
    }

    #[test]
    fn test_mul() {
        let sf1 = StandardForm::new(2.0, 5);
        let sf2 = StandardForm::new(2.0, 10);
        let result = sf1 * sf2;
        assert_eq!(result, StandardForm::new(150.0, 3));
    }

    #[test]
    fn test_div() {
        let sf1 = StandardForm::new(2.0, 5);
        let sf2 = StandardForm::new(2.0, 10);
        let result = sf1 / sf2;
        assert_eq!(result, StandardForm::new(50.0, 1));
    }
}