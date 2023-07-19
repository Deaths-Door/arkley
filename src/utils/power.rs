/// Trait for exponentiation.
pub trait Power<Rhs>{
    /// The output type of the exponentiation operation.
    type Output;

    /// Raises `self` to the power of `exp`.
    ///
    /// # Arguments
    ///
    /// * `exp` - The exponent.
    ///
    /// # Returns
    ///
    /// The result of `self` raised to the power of `exp`.
    fn to_the_power_of(self, other: Rhs) -> Self::Output;
}

macro_rules! impl_power {
    (signed; $($t : ty),*) => {
        $(
            impl Power<$t> for $t {
                type Output = $t;

                    fn to_the_power_of(self, other: $t) -> Self::Output {
                        self.pow(other as u32)
                    }
            }
        )*
    };
    (float; $($t : ty),*) => {
        $(
            impl Power<$t> for $t {
                type Output = $t;

                    fn to_the_power_of(self, other: $t) -> Self::Output {
                        self.powf(other)
                    }
            }
        )*
    };
}

impl_power!(signed; i8,i16,i32,i64);
impl_power!(float; f32,f64);

//,f32,f64
    /*i8 => (i8 => i8, i16 => i16, i32 => i32),
    i16 => (i8 => i8, i16 => i16, i32 => i32),
    i32 => (i8 => i8, i16 => i16, i32 => i32),
    i64 => (i8 => i64,i16 => i64,i32 => i64,i64 => i64)*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_i8() {
        assert_eq!(2_i8.to_the_power_of(3_i8), 8_i8);
        //assert_eq!(3_i8.to_the_power_of(2_i16), 9_i8);
        //assert_eq!(4_i8.to_the_power_of(2_i32), 16_i8);
      //  assert_eq!(2_i8.to_the_power_of(3_i64), 8_i8);
    }

    #[test]
    fn test_power_i16() {
        //assert_eq!(2_i16.to_the_power_of(3_i8), 8_i16);
        assert_eq!(3_i16.to_the_power_of(2_i16), 9_i16);
        //assert_eq!(4_i16.to_the_power_of(2_i32), 16_i16);
     //   assert_eq!(2_i16.to_the_power_of(3_i64), 8_i16);
    }

    #[test]
    fn test_power_i32() {
       // assert_eq!(2_i32.to_the_power_of(3_i8), 8_i32);
        //assert_eq!(3_i32.to_the_power_of(2_i16), 9_i32);
        assert_eq!(4_i32.to_the_power_of(2_i32), 16_i32);
       // assert_eq!(2_i32.to_the_power_of(3_i64), 8_i32);
    }

    #[test]
    fn test_power_i64() {
        assert_eq!(2_i64.to_the_power_of(3_i8), 8_i64);
        assert_eq!(3_i64.to_the_power_of(2_i16), 9_i64);
        assert_eq!(4_i64.to_the_power_of(2_i32), 16_i64);
        assert_eq!(2_i64.to_the_power_of(3_i64), 8_i64);
    }
}
