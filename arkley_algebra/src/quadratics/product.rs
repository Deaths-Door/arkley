use num_notation::Num;

use crate::manipulation::Find;
use super::IntegerQuadratic;

/// A utility struct for representing the product of roots in a quadratic equation.
///
/// The `ProductOfRoots` struct provides a mechanism for calculating the product of roots
/// of a quadratic equation using Vieta's Formulas. It is created by the
/// [super::Quadratic::product_of_roots] method.
#[derive(Debug, Clone)]
pub struct ProductOfRoots<T>(pub(super) T);

impl<T,O> From<T> for ProductOfRoots<IntegerQuadratic<O>> where T : Into<IntegerQuadratic<O>> , O : Num + Clone {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> Find for ProductOfRoots<IntegerQuadratic<T>> where T : Num + Clone {
    type Output = T;

    // c/a
    fn find(self) -> T {
        self.0.c / self.0.a
    }
}

#[cfg(feature="describe")]
use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

#[cfg(feature="describe")]
impl<T> Describe for ProductOfRoots<IntegerQuadratic<T>> where T : Num + Clone + std::fmt::Display {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let args = std::collections::HashMap::from([
            ("c",self.0.c.to_string().into()),
            ("a",self.0.a.to_string().into())
        ]);

        let s = resources.lookup_single_language(lang, "product-integerquadratic",Some(&args))?;

        vec![s].into()
    }
}