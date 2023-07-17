/// Trait for types that represent a zero value.
pub trait Zero : PartialEq + Sized {
    /// The zero value for the implementing type.2
    const ZERO : Self;

    /// Checks if the value is zero.
    ///
    /// # Returns
    ///
    /// `true` if the value is zero, `false` otherwise.
    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

macro_rules! impl_zero {
    ($T : ty,$v : expr) => { 
        impl Zero for $T {
            const ZERO : Self = $v;
        }
    }
}

impl_zero!(i8,0);
impl_zero!(i16,0);
impl_zero!(i32,0);
impl_zero!(i64,0);

impl_zero!(u8,0);
impl_zero!(u16,0);
impl_zero!(u32,0);
impl_zero!(u64,0);

impl_zero!(f32,0);
impl_zero!(f64,0);


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