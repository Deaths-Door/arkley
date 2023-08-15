use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use arkley_traits::Power;

use crate::{StandardForm,ParsingNumberError};

/// Represents a numeric value that can be decimal (aka f64) or Fraction or Standardform number
///
/// `Note` : add fractions variant to is as well 
#[derive(Debug, PartialEq)]
pub enum Number {
    /// Represents a floating-point decimal number.
    Decimal(f64),
    /// Represents a number in the StandardForm notation.
    StandardForm(StandardForm),
}

impl PartialOrd<Number> for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number::Decimal(a), Number::Decimal(b)) => a.partial_cmp(b),
            (Number::StandardForm(a), Number::StandardForm(b)) => a.partial_cmp(b),
            (Number::StandardForm(a), Number::Decimal(b)) => a.partial_cmp(b),
            (Number::Decimal(a), Number::StandardForm(b)) => {
                let rhs : StandardForm = (*a).into();
                rhs.partial_cmp(b)
            },
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Decimal(float) => write!(f,"{}",float),
            Number::StandardForm(sf) => write!(f,"{}",sf),
        }
    }
}

impl TryFrom<&str> for Number {
    type Error = ParsingNumberError;
    fn try_from(value : &str) -> Result<Self, Self::Error> {
        let into_f64 = value.parse::<f64>();
        if let Ok(float) = into_f64 {
            return Ok(Number::Decimal(float));
        }

        let into_sf = StandardForm::try_from(value);

        if let Ok(standard_form) = into_sf{
            return Ok(Number::StandardForm(standard_form));
        }

        Err(ParsingNumberError(into_f64.unwrap_err(),into_sf.unwrap_err()))
    }
}

impl Power for Number {
    type Output = Number;

    fn to_the_power_of(self, other: Number) -> Self::Output {
        match (self, other) {
            (Number::Decimal(a), Number::Decimal(b)) => Number::Decimal(a.to_the_power_of(b)),
            _ => todo!("")
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self,other : Number) -> Self::Output {
        use crate::Number::Decimal;
        match (self,other) {
            (Decimal(f1),Decimal(f2)) => Decimal(f1 + f2),
            (Number::StandardForm(sf1),Number::StandardForm(sf2)) => Number::StandardForm(sf1 + sf2),
            (Number::StandardForm(sf1),Number::Decimal(f2)) => Number::StandardForm(sf1 + f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs : StandardForm = f1.into();
                Number::StandardForm(rhs + sf2)
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self,other : Number) -> Self::Output {
        match (self,other) {
            (Number::Decimal(f1),Number::Decimal(f2)) => Number::Decimal(f1 - f2),
            (Number::StandardForm(sf1),Number::StandardForm(sf2)) => Number::StandardForm(sf1 - sf2),
            (Number::StandardForm(sf1),Number::Decimal(f2)) => Number::StandardForm(sf1 - f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs : StandardForm = f1.into();
                Number::StandardForm(rhs - sf2)
            }
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self,other : Number) -> Self::Output {
        match (self,other) {
            (Number::Decimal(f1),Number::Decimal(f2)) => Number::Decimal(f1 * f2),
            (Number::StandardForm(sf1),Number::StandardForm(sf2)) => Number::StandardForm(sf1 * sf2),
            (Number::StandardForm(sf1),Number::Decimal(f2)) => Number::StandardForm(sf1 * f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs : StandardForm = f1.into();
                Number::StandardForm(rhs * sf2)
            }
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self,other : Number) -> Self::Output {
        use crate::Number::Decimal;
        match (self,other) {
            (Decimal(f1),Decimal(f2)) => Number::Decimal(f1 / f2),
            (Number::StandardForm(sf1),Number::StandardForm(sf2)) => Number::StandardForm(sf1 / sf2),
            (Number::StandardForm(sf1),Number::Decimal(f2)) => Number::StandardForm(sf1 / f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs : StandardForm = f1.into();
                Number::StandardForm(rhs / sf2)
            }
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Number) {
        let temp_self = std::mem::replace(self, Number::Decimal(0.0)); // Take ownership of self

        match (temp_self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *self = Number::Decimal(f1 + f2),
            (Number::StandardForm(sf1), Number::StandardForm(sf2)) => *self = Number::StandardForm(sf1 + sf2),
            (Number::StandardForm(sf1), Number::Decimal(f2)) => *self = Number::StandardForm(sf1 + f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs: StandardForm = (f1).into();
                *self = Number::StandardForm(rhs + sf2);
            }
        }
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, other: Number) {
        let temp_self = std::mem::replace(self, Number::Decimal(0.0)); // Take ownership of self

        match (temp_self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *self = Number::Decimal(f1 - f2),
            (Number::StandardForm(sf1), Number::StandardForm(sf2)) => *self = Number::StandardForm(sf1 - sf2),
            (Number::StandardForm(sf1), Number::Decimal(f2)) => *self = Number::StandardForm(sf1 - f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs: StandardForm = (f1).into();
                *self = Number::StandardForm(rhs - sf2);
            }
        }
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, other: Number) {
        let temp_self = std::mem::replace(self, Number::Decimal(0.0)); // Take ownership of self

        match (temp_self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *self = Number::Decimal(f1 * f2),
            (Number::StandardForm(sf1), Number::StandardForm(sf2)) => *self = Number::StandardForm(sf1 * sf2),
            (Number::StandardForm(sf1), Number::Decimal(f2)) => *self = Number::StandardForm(sf1 * f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs: StandardForm = (f1).into();
                *self = Number::StandardForm(rhs * sf2);
            }
        }
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, other: Number) {
        let temp_self = std::mem::replace(self, Number::Decimal(0.0)); // Take ownership of self
        match (temp_self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *self = Number::Decimal(f1 / f2),
            (Number::StandardForm(sf1), Number::StandardForm(sf2)) => *self = Number::StandardForm(sf1 / sf2),
            (Number::StandardForm(sf1), Number::Decimal(f2)) => *self = Number::StandardForm(sf1 / f2),
            (Number::Decimal(f1), Number::StandardForm(sf2)) => {
                let rhs: StandardForm = (f1).into();
                *self = Number::StandardForm(rhs / sf2);
            }
        }
    }
}

macro_rules! primitives {
    (eq => $($t : ty),*) => {
        $(
            impl PartialEq<$t> for Number {
                fn eq(&self, other: &$t) -> bool {
                    match self {
                        Number::Decimal(f) => f == &(*other as f64),
                        Number::StandardForm(sf) => sf == other,
                    }
                }
            }
        )*
    };
    (ord => $($t : ty),*) => {
        $(
            impl PartialOrd<$t> for Number {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    match self {
                        Number::Decimal(f) => f.partial_cmp(&(*other as f64)),
                        Number::StandardForm(sf) => sf.partial_cmp(other)
                    }
                }
            }
        )*
    };

    (add => $($t : ty),*) => {
        $(
            impl Add<$t> for Number {
                type Output = Self;
                fn add(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f + other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf + other),
                    }
                }
            }
            
            impl AddAssign<$t> for Number {
                fn add_assign(&mut self, other: $t) {
                    *self += Number::Decimal(other as f64)
                }
            }
        )*
    };

    (sub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Number {
                type Output = Self;
                fn sub(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f - other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf - other),
                    }
                }
            }
            
            impl SubAssign<$t> for Number {
                fn sub_assign(&mut self, other: $t) {
                    *self -= Number::Decimal(other as f64)
                }
            }
        )*
    };
    (mul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Number {
                type Output = Self;
                fn mul(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f * other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf * other),
                    }
                }
            }
            
            impl MulAssign<$t> for Number {
                fn mul_assign(&mut self, other: $t) {
                    *self *= Number::Decimal(other as f64)
                }
            }
        )*
    };
    (div => $($t : ty),*) => {
        $(
            impl Div<$t> for Number {
                type Output = Self;
                fn div(self, other: $t) -> Self {
                    match self {
                        Number::Decimal(f) => Number::Decimal(f / other as f64),
                        Number::StandardForm(sf) => Number::StandardForm(sf / other),
                    }
                }
            }
            
            impl DivAssign<$t> for Number {
                fn div_assign(&mut self, other: $t) {
                    *self /= Number::Decimal(other as f64)
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

primitives!(eq => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(ord => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(operations => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_addition() {
        let num1 = Number::Decimal(2.5);
        let num2 = Number::Decimal(3.5);
        let result = num1 + num2;
        assert_eq!(result, Number::Decimal(6.0));
    }

    // Test subtraction
    #[test]
    fn test_subtraction() {
        let num1 = Number::Decimal(5.5);
        let num2 = Number::Decimal(3.5);
        let result = num1 - num2;
        assert_eq!(result, Number::Decimal(2.0));
    }

    // Test multiplication
    #[test]
    fn test_multiplication() {
        let num1 = Number::Decimal(2.5);
        let num2 = Number::Decimal(3.0);
        let result = num1 * num2;
        assert_eq!(result, Number::Decimal(7.5));
    }

    // Test division
    #[test]
    fn test_division() {
        let num1 = Number::Decimal(10.0);
        let num2 = Number::Decimal(2.0);
        let result = num1 / num2;
        assert_eq!(result, Number::Decimal(5.0));
    }

    // Test addition assignment
    #[test]
    fn test_addition_assignment() {
        let mut num = Number::Decimal(3.0);
        let num2 = Number::Decimal(2.0);
        num += num2;
        assert_eq!(num, Number::Decimal(5.0));
    }

    // Test subtraction assignment
    #[test]
    fn test_subtraction_assignment() {
        let mut num = Number::Decimal(5.0);
        let num2 = Number::Decimal(3.0);
        num -= num2;
        assert_eq!(num, Number::Decimal(2.0));
    }

    // Test multiplication assignment
    #[test]
    fn test_multiplication_assignment() {
        let mut num = Number::Decimal(2.5);
        let num2 = Number::Decimal(3.0);
        num *= num2;
        assert_eq!(num, Number::Decimal(7.5));
    }

    // Test division assignment
    #[test]
    fn test_division_assignment() {
        let mut num = Number::Decimal(10.0);
        let num2 = Number::Decimal(2.0);
        num /= num2;
        assert_eq!(num, Number::Decimal(5.0));
    }

    #[test]
    fn test_display_decimal() {
        let number = Number::Decimal(3.14);
        assert_eq!(format!("{}", number), "3.14");
        assert_eq!(number.to_string(), "3.14");
    }

    #[test]
    fn test_try_from_valid_number() {
        // Test a valid number conversion
        let input = "3.14";
        let result = Number::try_from(input);
        assert!(result.is_ok());

        // Check if the correct variant and value are returned
        if let Ok(Number::Decimal(value)) = result {
            assert_eq!(value, 3.14);
        } else {
            assert!(false, "Expected Ok(Number::Decimal(_)), but got an error.");
        }
    }

    #[test]
    fn test_try_from_invalid_number() {
        // Test an invalid number conversion
        let input = "abc"; // This is not a valid floating-point number
        let result = Number::try_from(input);
        assert!(result.is_err());

        // Check if the correct error variant is returned
        if let Err(_) = result {
        } else {
            assert!(false, "Expected Err(ParseFloatError), but got a success.");
        }
    }

    #[test]
    fn test_try_from_empty_string() {
        // Test conversion from an empty string
        let input = "";
        let result = Number::try_from(input);
        assert!(result.is_err());

        // Check if the correct error variant is returned
        if let Err(_) = result {
        } else {
            assert!(false, "Expected Err(ParseFloatError), but got a success.");
        }
    }
}