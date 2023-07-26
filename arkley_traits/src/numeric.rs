use crate::ArithmeticCore;

/// The `Numeric` trait is used to represent numeric numbers.
///
/// This trait is used to restrict types that behave like numeric numbers,
/// such as decimal standard form or coefficient in terms.
pub trait Numeric : ArithmeticCore {

}

macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {}
        )* 
    }
}

impl_numeric!(i8,i16,i32,i64);
impl_numeric!(f32,f64);