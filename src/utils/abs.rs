/// Trait for absolute value computation.
pub trait Abs {
    /// Computes the absolute value of `self`.
    ///
    /// # Returns
    ///
    /// The absolute value of `self`.
    fn absolute(&self) -> Self;
}

macro_rules! impl_abs {
    (unsigned; $($t:ty),*) => {
        $(
            impl Abs for $t {
                fn absolute(&self) -> Self {
                    *self
                }
            }
        )*
    };
    ($($t:ty),*) => {
        $(
            impl Abs for $t {
                fn absolute(&self) -> Self {
                    self.abs()
                }
            }
        )*
    }
}

impl_abs!(unsigned; u8,u16,u32,u64);
impl_abs!(i8,i16,i32,i64,f32,f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i8_absolute_value() {
        let value: i8 = -5;
        let absolute_value = value.absolute();
        assert_eq!(absolute_value, 5);
    }

    #[test]
    fn test_i16_absolute_value() {
        let value: i16 = -10;
        let absolute_value = value.absolute();
        assert_eq!(absolute_value, 10);
    }

    #[test]
    fn test_i32_absolute_value() {
        let value: i32 = -7;
        let absolute_value = value.absolute();
        assert_eq!(absolute_value, 7);
    }

    #[test]
    fn test_i64_absolute_value() {
        let value: i64 = -15;
        let absolute_value = value.absolute();
        assert_eq!(absolute_value, 15);
    }

    #[test]
    fn test_f32_absolute_value() {
        let value: f32 = -3.14;
        let absolute_value = value.absolute();
        assert_eq!(absolute_value, 3.14);
    }

    #[test]
    fn test_f64_absolute_value() {
        let value: f64 = -2.5;
        let absolute_value = value.absolute();
        assert_eq!(absolute_value, 2.5);
    }
}