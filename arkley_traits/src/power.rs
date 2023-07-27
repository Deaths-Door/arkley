/// Trait for exponentiation.
pub trait Power<Rhs = Self>{
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