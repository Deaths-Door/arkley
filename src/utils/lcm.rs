use crate::utils::Gcd;

/// Trait for calculating the Least Common Multiple (LCM).
pub trait Lcm: Gcd + std::ops::Div<Output = Self> + std::ops::Mul<Output = Self> {
    /// Calculates the Least Common Multiple (LCM) between `self` and `other`.
    ///
    /// # Parameters
    ///
    /// - `other`: The other value to calculate the LCM with.
    ///
    /// # Returns
    ///
    /// The calculated LCM of `self` and `other`.
    fn lcm(&self, other: &Self) -> Self {
        *self / self.gcd(other) * *other
    }
}

macro_rules! impl_lcm {
    ($T : ty) => { 
        impl Lcm for $T {}
    }
}

impl_lcm!(i8);
impl_lcm!(i16);
impl_lcm!(i32);
impl_lcm!(i64);

impl_lcm!(u8);
impl_lcm!(u16);
impl_lcm!(u32);
impl_lcm!(u64);

impl_lcm!(f32);
impl_lcm!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm_i8() {
        let a: i8 = 6;
        let b: i8 = 8;
        let lcm_result = a.lcm(&b);
        assert_eq!(lcm_result, 24);
    }

    #[test]
    fn test_lcm_i16() {
        let a: i16 = 12;
        let b: i16 = 18;
        let lcm_result = a.lcm(&b);
        assert_eq!(lcm_result, 36);
    }

    #[test]
    fn test_lcm_i32() {
        let a: i32 = 15;
        let b: i32 = 20;
        let lcm_result = a.lcm(&b);
        assert_eq!(lcm_result, 60);
    }

    #[test]
    fn test_lcm_i64() {
        let a: i64 = 24;
        let b: i64 = 36;
        let lcm_result = a.lcm(&b);
        assert_eq!(lcm_result, 72);
    }
}