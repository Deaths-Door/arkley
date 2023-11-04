use num_notation::Num;

use crate::manipulation::Find;
use super::IntegerQuadratic;


/// A utility struct representing the x-coordinate of the axis of symmetry of a quadratic equation.
///
/// The `AxisOfSymmetry` struct provides a mechanism for storing the x-coordinate of the axis
/// of symmetry, which is a vertical line passing through the vertex of the parabola.
/// It is calculated using the formula: `x = -b / (2a)`, where `a` and `b` are the coefficients
/// of the quadratic equation `ax^2 + bx + c = 0`.
///
/// It is created by methods that calculate the axis of symmetry, such as [super::Quadratic::axis_of_symmetry].
#[derive(Debug, Clone)]
pub struct AxisOfSymmetry<T>(pub(super) T);

impl<T,O> From<T> for AxisOfSymmetry<IntegerQuadratic<O>> where T : Into<IntegerQuadratic<O>> , O : Num + Clone {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> Find<T> for AxisOfSymmetry<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + std::ops::Neg<Output = T> {
    // x = -b / 2a
    fn find(self) -> T {
        (-self.0.b) / (T::from(2u8) * self.0.a)
    }
}

#[cfg(feature="describe")]
use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

#[cfg(feature="describe")]
impl<T> Describe for AxisOfSymmetry<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + std::fmt::Display {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let args = std::collections::HashMap::from([
            ("b",self.0.b.to_string().into()),
            ("a",self.0.a.to_string().into())
        ]);

        let s = resources.lookup_single_language(lang, "axis-symmetry-integerquadratic",Some(&args))?;

        vec![s].into()
    }
}