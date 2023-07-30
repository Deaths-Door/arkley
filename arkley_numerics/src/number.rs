use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use arkley_traits::Power;

use crate::StandardForm;

/// Represents a numeric value that can be decimal (aka f64) or Fraction or Standardform number
///
/// `Note` : add fractions variant to is as well 
/// `Note` : Remove Clone as its only used for op assign for f64 btw standardform
#[derive(Debug,PartialEq)]
pub enum Number {
    /// Represents a floating-point decimal number.
    Decimal(f64),
    /// Represents a number in the StandardForm notation.
    StandardForm(StandardForm)
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

macro_rules! partial_number {
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
}

partial_number!(eq => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
partial_number!(ord => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Decimal(float) => write!(f,"{}",float),
            Number::StandardForm(sf) => write!(f,"{}",sf),
        }
    }
}

impl TryFrom<&str> for Number {
    type Error = ();//ParsingNumberError;
    fn try_from(value : &str) -> Result<Self, Self::Error> {
        todo!()
      //  value.parse::<f64>().and_then(|float| Ok(Number::Decimal(float)) ).map_err(|_| ())
    }
}

impl Power for Number {
    type Output = Number;

    fn to_the_power_of(self, other: Number) -> Self::Output {
        match (self, other) {
            (Number::Decimal(a), Number::Decimal(b)) => Number::Decimal(a.to_the_power_of(b)),
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