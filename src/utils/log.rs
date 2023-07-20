/// The `Log` trait represents the ability to calculate logarithms with a specified base.
///
/// Implementing this trait allows types to provide custom implementations for calculating
/// logarithms with a specific base. The result of the logarithm is returned as an `Option<Self>`,
/// where `Self` is the type implementing the `Log` trait.
pub trait Log where Self : Sized {
    /// Calculate the logarithm of the value with the specified base.
    ///
    /// # Parameters
    ///
    /// - `base`: The base of the logarithm.
    ///
    /// # Returns
    ///
    /// An optional result representing the logarithm of the value with the specified base. If
    /// the logarithm is defined and computable, it returns `Some(result)`, where `result` is of
    /// type `Self`. If the logarithm is undefined or cannot be computed, it returns `None`.
    fn log_with_base(&self, base: f64) -> Option<Self>;
}

// Implement Log trait for u8 to u64
macro_rules! impl_log_for_unsigned_integer {
    ($($t:ty),*) => {
        $(
            impl Log for $t {
                fn log_with_base(&self, base: f64) -> Option<Self> {
                    if *self == 0 {
                        None
                    } else {
                        (base > 0.0 && base != 1.0).then(|| (self.saturating_sub(1) as f64 / base.log(base) + 1.0) as $t)
                    }
                }
            }
        )*
    };
}

// Implement Log trait for i8 to i64
macro_rules! impl_log_for_signed_integer {
    ($($t:ty),*) => {
        $(
            impl Log for $t {
                fn log_with_base(&self, base: f64) -> Option<Self> {
                    if *self <= 0 {
                        None
                    } else {
                        (base > 0.0 && base != 1.0).then(|| (self.saturating_sub(1) as f64 / base.log(base) + 1.0) as $t)
                    }
                }
            }
        )*
    };
}

// Implement Log trait for f32 and f64
macro_rules! impl_log_for_float {
    ($($t:ty),*) => {
        $(
            impl Log for $t {
                fn log_with_base(&self, base: f64) -> Option<Self> {
                    if *self <= 0.0 || base <= 0.0 || base == 1.0 {
                        None
                    } else {
                        Some((self.ln() as f64 / base.ln()) as $t)
                    }
                }
            }
        )*
    };
}

// Invoke the macros to generate implementations
impl_log_for_unsigned_integer!(u8, u16, u32, u64);
impl_log_for_signed_integer!(i8, i16, i32, i64);
impl_log_for_float!(f32, f64);