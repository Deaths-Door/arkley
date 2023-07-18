use std::ops::{Add,Sub,Mul,Div,Neg,Rem,AddAssign,SubAssign,MulAssign,DivAssign};

use std::cmp::max;

use crate::{
    utils::{
        Numeric
    },
};

use super::Fraction;
/*
/// Represents a number in standard form.
///
/// The `Standardform` struct holds a fraction representing the significand (mantissa) of the number
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
pub struct Standardform<N,D> where N : Numeric , D : Numeric  {
    mantissa : Fraction<N,D>,
    exponent : u8
}

impl<N, D> Standardform<N, D> where N: Numeric,D: Numeric {
    /// Returns a reference to the fraction representing the significand (mantissa) of the number.
    pub const fn mantissa(&self) -> &Fraction<N, D> {
        &self.mantissa
    }

    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    pub const fn exponent(&self) -> &u8 {
        &self.exponent
    }
}

impl<N, D> Standardform<N, D> where N: Numeric, D: Numeric , Fraction<N, D>: PartialOrd<i8> + MulAssign<i32> {
    /// Creates a new instance of StandardForm with the given mantissa and exponent
    pub fn new(mantissa : Fraction<N,D>,exponent : u8) -> Self {
        Self { mantissa , exponent}.adjust()
    }

    fn adjust(mut self) -> Self {
        if !(self.mantissa >= 1 as i8  && self.mantissa <= 10 as i8 ) || !(self.mantissa >= -10 as i8  && self.mantissa <= -1 as i8) {
            let abs = self.mantissa.absolute();
            let log = abs.log(10).ceil();
            
            self.mantissa /= 10 ^ log;
            self.exponent = log;

            if self.mantissa < 0 {
                self.mantissa = -self.mantissa;
            }
            else if self.mantissa > 0 as i8 && self.mantissa <= 1 as i8 {
                self.mantissa *= 10;
                self.exponent -= 1;
            }
            
        }

        self
    }
}

impl<N,D> Standardform<N,D> where N: Numeric + std::fmt::Display ,D: Numeric + std::fmt::Display  {
    /// Returns the string representation of the number in scientific notation.
    fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }
    
    /// Returns the string representation of the number in engineering notation.
    fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }
}

impl<N,D> std::fmt::Display for Standardform<N,D> where N : Numeric + std::fmt::Display , D : Numeric + std::fmt::Display , Fraction<N,D> : std::fmt::Display + Mul<u8,Output = Self> + Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.exponent > 4 {
            return write!(f,"{}",self.to_scientific_notation());
        };

        write!(f,"{}",self.mantissa * self.exponent)
    }
}

impl<N,D> std::ops::Add for Standardform<N, D> where N: Numeric, D: Numeric , Fraction<N, D>: PartialOrd<i8> + Mul<i32> {
    type Output = Standardform<N,D>;

    fn add(self, other: Self) -> Self {
        let max_power = max(self.exponent, other.exponent);
        let m = self.mantissa * 10_i32.pow((self.exponent - max_power).into());
        let n = other.mantissa * 10_i32.pow((other.exponent - max_power).into());
        Standardform::new(m + n, max_power)
    }
}*/