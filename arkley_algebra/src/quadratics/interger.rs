use num_notation::{Num, Signed};

use super::{Discriminant, Quadratic};

/// Represents an integer quadratic equation of the form `a*x^2 + b*x + c`.
///
/// This struct allows you to work with quadratic equations where `a`, `b`, and `c` can be any numeric type,
/// such as integers (`i32`, `i64`, etc.) or floating-point numbers (`f32`, `f64`).
#[derive(Clone)]
pub struct IntegerQuadratic<T> where T: Num + Clone{
    pub(super) a: T,
    pub(super) b: T,
    pub(super) c: T,
}

impl<T> std::fmt::Debug for IntegerQuadratic<T> where T : std::fmt::Display + Signed + Clone{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self}")
    }
}

impl<T> std::fmt::Display for IntegerQuadratic<T> where T : std::fmt::Display + Signed + Clone{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index,(item,x)) in [(&self.a,"x^2"),(&self.b,"x"),(&self.c,"")].iter().enumerate() {
            if item.is_one() {
                f.write_str(x)?
            }

            if !item.is_zero() {
                match index == 0 {
                    true => write!(f,"{item}{x}"),
                    false => match item.is_positive() {
                        true => write!(f," + {item}{x}"),
                        false => write!(f," - {}{}",item.abs(),x)
                    }
                }?
            }
        }
        
        f.write_str(" = 0")
    }
}

impl<T> IntegerQuadratic<T> where T: Num + Clone {
    /// Creates a new `IntegerQuadratic` instance with the provided coefficients.
    ///
    /// # Arguments
    ///
    /// * `a` - The coefficient for the quadratic term (`a*x^2`).
    /// * `b` - The coefficient for the linear term (`b*x`).
    /// * `c` - The constant term (`c`).
    ///
    /// # Returns
    ///
    /// A new `IntegerQuadratic` instance with the given coefficients
    pub const fn new( a: T,b: T,c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T> Quadratic<T> for IntegerQuadratic<T> where T: Num + Clone + From<u8>  {
    fn discriminant(self) -> Discriminant<Self> {
        Discriminant(self)
    }
}