use num_notation::Num;

use crate::manipulation::Find;
use super::IntegerQuadratic;

/// A utility struct for representing the sum of roots in a quadratic equation.
///
/// The `SumOfRoots` struct provides a mechanism for calculating the sum of roots
/// of a quadratic equation using Vieta's Formulas. It is created by the
/// [super::Quadratic::sum_of_roots] method.
#[derive(Debug, Clone)]
pub struct SumOfRoots<T>(pub(super) T);

impl<T,O> From<T> for SumOfRoots<IntegerQuadratic<O>> where T : Into<IntegerQuadratic<O>> , O : Num + Clone {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> Find for SumOfRoots<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + std::ops::Neg<Output = T> {
    type Output = T;
    // -b/a
    fn find(self) -> T {
        (-self.0.b) / self.0.a
    }
}

#[cfg(feature="describe")]
use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

#[cfg(feature="describe")]
impl<T> Describe for SumOfRoots<IntegerQuadratic<T>> where T : Num + Clone + std::fmt::Display {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let args = std::collections::HashMap::from([
            ("b",self.0.b.to_string().into()),
            ("a",self.0.a.to_string().into())
        ]);

        let s = resources.lookup_single_language(lang, "product-integerquadratic",Some(&args))?;

        vec![s].into()
    }
}