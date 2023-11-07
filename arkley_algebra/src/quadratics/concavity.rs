use std::{cmp::Ordering, collections::HashMap};

use num_notation::Num;
use crate::manipulation::Find;
use super::{IntegerQuadratic, Nature};

/// A utility struct for determining the concavity of a parabola.
///
/// The `Concavity` struct provides a mechanism for determining the direction in which
/// a parabola opens (whether it's concave upward or downward) based on the coefficient `a`
/// in the quadratic equation.
///
/// It is created by methods that calculate the concavity, such as [super::Quadratic::is_concave_upward].
#[derive(Debug, Clone)]
pub struct Concavity<T>(pub(super) T);

impl<T,O> From<T> for Concavity<IntegerQuadratic<O>> where T : Into<IntegerQuadratic<O>> , O : Num + Clone {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// An enum representing the concavity of a parabola.
#[derive(Debug, Clone)]
pub enum ConcavityType {
    /// The parabola opens upward 🙂 (a > 0).
    Upward,
    
    /// The parabola opens downward 🙁 (a < 0).
    Downward,
    
    /// Concavity is undefined 😡 (a = 0).
    Undefined,
}

impl<T> Concavity<IntegerQuadratic<T>> where T : Num + Clone + Ord  {
    /// Determines the concavity of the parabola based on the coefficient `a`.
    ///
    /// If `a` is greater than 0, the parabola opens upward (concave upward).
    /// If `a` is less than 0, the parabola opens downward (concave downward).
    /// If `a` is equal to 0, concavity is undefined (or you can return `None` to indicate this).
    ///
    /// # Returns
    ///
    /// - `Some(true)` if the parabola is concave upward.
    /// - `Some(false)` if the parabola is concave downward.
    /// - `None` if concavity is undefined.
    pub fn is_upward(&self) -> Option<bool> {
        match self.is_undefined() {
            true => None,
            false => Some(self.0.a > T::zero()),
        }
    }

    /// Determines the concavity of the parabola based on the coefficient `a`.
    ///
    /// If `a` is greater than 0, the parabola opens upward (concave upward).
    /// If `a` is less than 0, the parabola opens downward (concave downward).
    /// If `a` is equal to 0, concavity is undefined (or you can return `None` to indicate this).
    ///
    /// # Returns
    ///
    /// - `Some(false)` if the parabola is concave upward.
    /// - `Some(true)` if the parabola is concave downward.
    /// - `None` if concavity is undefined.
    pub fn is_downward(&self) -> Option<bool> {
        match self.is_undefined() {
            true => None,
            false => Some(self.0.a < T::zero()),
        }
    }

    /// Checks if the concavity of the parabola is undefined (a = 0).
    ///
    /// # Returns
    ///
    /// - `true` if the concavity is undefined (a = 0).
    /// - `false` if the concavity is defined (a is not equal to 0).
    pub fn is_undefined(&self) -> bool {
        self.0.a.is_zero()
    }

    /// Determines the concavity of the parabola based on the coefficient `a`.
    ///
    /// If `a` is greater than 0, the parabola opens upward (concave upward).
    /// If `a` is less than 0, the parabola opens downward (concave downward).
    /// If `a` is equal to 0, concavity is undefined.
    ///
    /// # Returns
    ///
    /// - `ConcavityType::Upward` if the parabola is concave upward.
    /// - `ConcavityType::Downward` if the parabola is concave downward.
    /// - `ConcavityType::Undefined` if concavity is undefined.
    pub fn concavity_type(&self) -> ConcavityType {
        match self.0.a.cmp(&T::zero()) {
            Ordering::Less => ConcavityType::Downward,
            Ordering::Equal =>  ConcavityType::Undefined,
            Ordering::Greater => ConcavityType::Upward,
        }
    }
}

impl<T> Find for Concavity<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + std::ops::Neg<Output = T> + Ord {
    type Output = ConcavityType;
    // -b/a
    fn find(self) -> ConcavityType {
        self.concavity_type()
    }
}

#[cfg(feature="describe")]
use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

#[cfg(feature="describe")]
impl<T> Describe for Concavity<IntegerQuadratic<T>> where T : Num + Clone + Ord + std::fmt::Display {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let text_id = match self.concavity_type() {
            ConcavityType::Upward => "concavity-integerquadratic-upwards",
            ConcavityType::Downward => "concavity-integerquadratic-downwards",
            ConcavityType::Undefined => "concavity-integerquadratic-undefined",
        };

        let args = HashMap::from([
            ("a",self.0.a.to_string().into())
        ]);

        let string = resources.lookup_single_language(lang,text_id, Some(&args))?;

        vec![string].into()
    }
}