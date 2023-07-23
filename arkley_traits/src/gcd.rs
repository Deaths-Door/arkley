use crate::Zero;
/// Represents the GCD (Greatest Common Divisor) trait.
/// This trait provides a method to calculate the GCD between two values of the same type.
/// by default uses Euclidean algorithm to calculate GCD (or HCF) but for primitive numbers eg u8 .. i32...f64 Steins alogirthm is used
/// `TODO` : maybe add Rhs and output to make it more like add etc methods
pub trait Gcd/*<Rhs = Self>*/ : Zero + std::ops::Rem<Output = Self> + Sized + Copy {
    /// Calculates the Greatest Common Divisor (GCD) between `self` and `other`.
    ///
    /// For primitive number types like `i32` etc, the Stein's algorithm is used
    /// to optimize the computation. For other types, the Euclidean algorithm is used by default if no custom implementation is given.
    /// 'Note' : Find a way to make default implementation copyless
    fn gcd(self, other: Self) -> Self {  
        if other.is_zero() { 
            self
        } else {
            (other % self).gcd(self)
        }
    }
}

macro_rules! impl_gcd {
    (signed; $($t:ty),*) => {
        $(
            impl Gcd for $t {
                fn gcd(self,other : Self) -> Self {
                    // Use Stein's algorithm
                    let mut m = self;
                    let mut n = other;
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
        )*
    };
    (unsigned; $($t:ty),*) => {
        $(
            impl Gcd for $t {
                fn gcd(self,other : Self) -> Self {
                    let mut m = self;
                    let mut n = other;

                    // find common factors of 2
                    let shift = (m | n).trailing_zeros();

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
        )*
    };
    (float; $($t:ty),*) => {
        $(
            impl Gcd for $t {}
        )* 
    }
}


impl_gcd!(signed; i8,i16,i32,i64);
impl_gcd!(unsigned; u8,u16,u32,u64);
impl_gcd!(float; f32,f64);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_i8() {
        let a: i8 = 15;
        let b: i8 = 25;
        let gcd_result = a.gcd(b);
        assert_eq!(gcd_result, 5);
    }

    #[test]
    fn test_gcd_i16() {
        let a: i16 = 30;
        let b: i16 = 45;
        let gcd_result = a.gcd(b);
        assert_eq!(gcd_result, 15);
    }

    #[test]
    fn test_gcd_i32() {
        let a: i32 = 80;
        let b: i32 = 120;
        let gcd_result = a.gcd(b);
        assert_eq!(gcd_result, 40);
    }

    #[test]
    fn test_gcd_i64() {
        let a: i64 = 105;
        let b: i64 = 140;
        let gcd_result = a.gcd(b);
        assert_eq!(gcd_result, 35);
    }

    #[test]
    fn one_two(){
        let a: i64 = 1;
        let b: i64 = 2;
        let gcd_result = a.gcd(b);
        assert_eq!(gcd_result, 1);
    }
}