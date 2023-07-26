use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

/// Represents a numeric value that can be either a floating-point decimal (`f64`)
/// or a fraction (`Fraction`) with customizable precision.
///
/// # Generic Parameters
///
/// `T`: A type that implements the `ArithmeticCore` trait, which defines basic
/// arithmetic operations like addition, subtraction, multiplication, and division.
///
/// `Note` : TODO add standardform to it maybe and add fractions variant to is as well
/// # Variants
///
/// - `Decimal(f64)`: Represents a floating-point decimal number.
/// - `Fraction(Fraction<T>)`: Represents a fraction with the given precision type `T`.
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Number {
    /// Represents a floating-point decimal number.
    Decimal(f64),
}

impl Add for Number {
    type Output = Number;

    fn add(self,other : Number) -> Self::Output {
        use crate::Number::Decimal;
        match (self,other) {
            (Decimal(f1),Decimal(f2)) => Decimal(f1 + f2),
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self,other : Number) -> Self::Output {
        use crate::Number::Decimal;
        match (self,other) {
            (Decimal(f1),Decimal(f2)) => Number::Decimal(f1 - f2),

        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self,other : Number) -> Self::Output {
        use crate::Number::Decimal;
        match (self,other) {
            (Decimal(f1),Decimal(f2)) => Number::Decimal(f1 * f2),

        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self,other : Number) -> Self::Output {
        use crate::Number::Decimal;
        match (self,other) {
            (Decimal(f1),Decimal(f2)) => Number::Decimal(f1 / f2),

        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Number) {
        match (self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *f1 += f2,
        }
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, other: Number) {
        match (self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *f1 -= f2,
        }
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, other: Number) {
        match (self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *f1 *= f2,
        }
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, other: Number) {
        match (self, other) {
            (Number::Decimal(f1), Number::Decimal(f2)) => *f1 /= f2,
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

    // Run all the tests
    fn main() {
        test_addition();
        test_subtraction();
        test_multiplication();
        test_division();
        test_addition_assignment();
        test_subtraction_assignment();
        test_multiplication_assignment();
        test_division_assignment();
    }
}