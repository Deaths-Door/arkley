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
    fn lcm(self, other: Self) -> Self {
        self / self.gcd(other) * other
    }
}

macro_rules! impl_lcm {
    ($($t:ty),*) => { 
        $(
            impl Lcm for $t {}

        )*
    }
}

impl_lcm!(u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm_i8() {
        let a: i8 = 6;
        let b: i8 = 8;
        let lcm_result = a.lcm(b);
        assert_eq!(lcm_result, 24);
    }

    #[test]
    fn test_lcm_i16() {
        let a: i16 = 12;
        let b: i16 = 18;
        let lcm_result = a.lcm(b);
        assert_eq!(lcm_result, 36);
    }

    #[test]
    fn test_lcm_i32() {
        let a: i32 = 15;
        let b: i32 = 20;
        let lcm_result = a.lcm(b);
        assert_eq!(lcm_result, 60);
    }

    #[test]
    fn test_lcm_i64() {
        let a: i64 = 24;
        let b: i64 = 36;
        let lcm_result = a.lcm(b);
        assert_eq!(lcm_result, 72);
    }
}