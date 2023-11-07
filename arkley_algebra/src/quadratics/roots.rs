use num_notation::Num;

use crate::manipulation::Find;
use super::*;
/// A utility struct for determining the nature of roots in a quadratic equation.
///
/// The `Roots` struct provides a mechanism for calculating the nature of roots
/// of a quadratic equation based on its discriminant value. It contains methods
/// for this specific purpose, simplifying the process and providing a clear interface.
///
/// It is created by [super::Quadratic::roots] method.
#[derive(Debug,Clone)]
pub struct Roots<T>(pub(super) Discriminant<T>);

impl<T,O> From<T> for Roots<O> where T : Into<Discriminant<O>> , O : Num + Clone {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Represents the nature of the roots of a quadratic equation.
///
/// The `NatureOfRoots` enum classifies the nature of the roots of a quadratic equation.
/// It provides three variants, indicating whether the equation has distinct real roots,
/// a single root, or no real roots (which implies two complex roots)
#[derive(Debug)]
pub enum Nature<T> {
    /// Indicates that the quadratic equation has two distinct real roots.
    DistinctRealRoots(T,T),
    /// Indicates that the quadratic equation has a single real root.
    SingleRoot(T),
    /// Indicates that the quadratic equation has no real roots, implying two complex roots.
    NoRealRoots  
}

impl<T> Find for Roots<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + PartialOrd + std::ops::Neg<Output = T> + num_notation::Pow<T,Output = T> {
    type Output = Nature<T>;
    // Use the discriminant to determine the nature of the roots:
    fn find(self) -> Nature<T> {
        let d = self.0.clone().find();

        let zero = T::from(0u8);

        if d < zero {
            return Nature::NoRealRoots;
        }

        let two: T = T::from(2);

        if d == zero {
            let root = -self.0.0.b / (two * self.0.0.a);
            return Nature::SingleRoot(root)
        }

        let neg_b = -self.0.0.b ;

        let sqrt = d.pow(T::from(1) / two.clone());  // as 25^0.5 = sqrt(25)

        let two_a = two.clone() * self.0.0.a;

        let r1 = (neg_b.clone() + sqrt.clone()) / two_a.clone();
        let r2 = (neg_b - sqrt) / two_a;

        return Nature::DistinctRealRoots(r1,r2)
    }
}

#[cfg(feature="describe")]
use arkley_describe::{
    Describe, Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

#[cfg(feature="describe")]
impl<T> Describe for Roots<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + std::fmt::Display + PartialOrd + std::ops::Neg<Output = T> + num_notation::Pow<T,Output = T>  {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        use std::collections::HashMap;
        
        let discriminant_description = self.0.clone().describe(resources,lang)?;

        let nature = self.find();

        let description = match nature {
            Nature::DistinctRealRoots(r1, r2) => {
                let args = HashMap::from([
                    ("r1",r1.to_string().into()),
                    ("r2",r2.to_string().into())

                ]);
                resources.lookup_single_language::<&str>(lang, "roots-integerquadratic.two",Some(&args))
            },
            Nature::SingleRoot(root) => {
                let args = HashMap::from([("root",root.to_string().into())]);
                resources.lookup_single_language::<&str>(lang, "roots-integerquadratic.one",Some(&args))
            },
            Nature::NoRealRoots => resources.lookup_single_language::<&str>(lang, "roots-integerquadratic.zero",None),
        }?;

        let mut vec = Vec::new();

        vec.extend(discriminant_description.into_iter());
        vec.push(description);

        vec.into()
    }
}