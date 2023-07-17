use crate::utils::{Zero};

/// Represents the GCD (Greatest Common Divisor) trait.
/// This trait provides a method to calculate the GCD between two values of the same type.
// by default uses Euclidean algorithm to calculate gcd (Greastes...) but for primitive numbers eg u8 .. i32...f64 Steins alogirthm is used
pub trait Gcd : Zero + std::ops::Rem<Output = Self> + Copy {
    /// Calculates the Greatest Common Divisor (GCD) between `self` and `other`.
    ///
    /// For primitive number types like `u8`, `i32`, `f64` etc, the Stein's algorithm is used
    /// to optimize the computation. For other types, the Euclidean algorithm is used by default if no custom implementation is given.
    /// TODO IMPLEMENT STEINS ALGORITHM 
    fn gcd(&self, other: &Self) -> Self {
        if other.is_zero() {
            *self 
        }
        else {
            (*other % *self).gcd(self)
        }
    }
}

macro_rules! steins_algorithm {
    ($T : ty) => {
        impl Gcd for $T {
            fn gcd(&self,other : &Self) -> Self {
                // Use Stein's algorithm
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 {
                    return (m | n).abs();
                }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // The algorithm needs positive numbers, but the minimum value
                // can't be represented as a positive one.
                // It's also a power of two, so the gcd can be
                // calculated by bitshifting in that case

                // Assuming two's complement, the number created by the shift
                // is positive for all numbers except gcd = abs(min value)
                // The call to .abs() causes a panic in debug mode
                if m == Self::MIN || n == Self::MIN {
                    return ((1 << shift) as Self).abs();
                }

                // guaranteed to be positive now, rest like unsigned algorithm
                m = m.abs();
                n = n.abs();

                // divide n and m by 2 until odd
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();

                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }
        }
    }
}


steins_algorithm!(i8);
steins_algorithm!(i16);
steins_algorithm!(i32);
steins_algorithm!(i64);

steins_algorithm!(u8);
steins_algorithm!(u16);
steins_algorithm!(u32);
steins_algorithm!(u64);

steins_algorithm!(f32);
steins_algorithm!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_i8() {
        let a: i8 = 15;
        let b: i8 = 25;
        let gcd_result = a.gcd(&b);
        assert_eq!(gcd_result, 5);
    }

    #[test]
    fn test_gcd_i16() {
        let a: i16 = 30;
        let b: i16 = 45;
        let gcd_result = a.gcd(&b);
        assert_eq!(gcd_result, 15);
    }

    #[test]
    fn test_gcd_i32() {
        let a: i32 = 80;
        let b: i32 = 120;
        let gcd_result = a.gcd(&b);
        assert_eq!(gcd_result, 40);
    }

    #[test]
    fn test_gcd_i64() {
        let a: i64 = 105;
        let b: i64 = 140;
        let gcd_result = a.gcd(&b);
        assert_eq!(gcd_result, 35);
    }
}