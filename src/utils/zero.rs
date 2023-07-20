/// Trait for types that represent a zero value.
pub trait Zero : PartialEq + Sized {
    /// The zero value for the implementing type.
    #[deprecated(note="use zero() instead")]
    const ZERO : Self;

    /// Checks if the value is zero.
    ///
    /// # Returns
    ///
    /// `true` if the value is zero, `false` otherwise.
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    /// The zero value for the implementing type.
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($($t:ty => $v : expr)*) => {
        $(
            impl Zero for $t {
                const ZERO : Self = $v;

                fn zero() -> Self {
                    $v
                }
            }
        )*
    };
}

impl_zero!(
    i8 => 0
    i16 => 0
    i32 => 0
    i64 => 0

    u8 => 0
    u16 => 0
    u32 => 0
    u64 => 0

    f32 => 0.0
    f64 => 0.0
);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_zero_i8() {
        let value: i8 = 0;
        assert!(value.is_zero());
    
        let value: i8 = 42;
        assert!(!value.is_zero());
    }
    
    #[test]
    fn test_is_zero_i16() {
        let value: i16 = 0;
        assert!(value.is_zero());
    
        let value: i16 = -10;
        assert!(!value.is_zero());
    }
    
    #[test]
    fn test_is_zero_i32() {
        let value: i32 = 0;
        assert!(value.is_zero());
    
        let value: i32 = 12345;
        assert!(!value.is_zero());
    }
    
    #[test]
    fn test_is_zero_i64() {
        let value: i64 = 0;
        assert!(value.is_zero());
    
        let value: i64 = -9876543210;
        assert!(!value.is_zero());
    }
}