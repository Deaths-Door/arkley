use crate::Decimal;

use std::num::ParseFloatError;


/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number (using a underlying fraction for zero precision loss)
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
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