use std::collections::HashMap;

use num_notation::Num;

use crate::{IntegerQuadratic, manipulation::Find};

/// A utility struct for determining the nature of roots in a quadratic equation.
///
/// The `Roots` struct provides a mechanism for calculating the nature of roots
/// of a quadratic equation based on its discriminant value. It contains methods
/// for this specific purpose, simplifying the process and providing a clear interface.
///
/// It is created by [Quadratic::roots] or [Quadratic::nature_of_roots] method.
#[derive(Debug,Clone)]
pub struct Discriminant<T>(pub(super) T); 

impl<T> Find<T> for Discriminant<IntegerQuadratic<T>> where T : Num + Clone + From<u8> {
    // D = b^2 - 4ac
    fn find(self) -> T {
        (self.0.b.clone() * self.0.b) - (T::from(4u8) * self.0.a * self.0.c)
    }
}


#[cfg(feature="describe")]
use arkley_describe::{
    Describe,
    Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier}
};

#[cfg(feature="describe")]
impl<T> Describe for Discriminant<IntegerQuadratic<T>> where T : Num + Clone + From<u8> + std::fmt::Display {
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
        let haupteil = resources.lookup_single_language::<&str>(lang, "discriminant-integerquadratic.haupttiel", None).unwrap(); 

        let nebenteil =  {
            let mut args = HashMap::from([
                ("a",self.0.a.to_string().into()),
                ("b",self.0.b.to_string().into()),
                ("c",self.0.c.to_string().into()),                
            ]);

            let result = self.find();
            args.insert("result", result.to_string().into());

            resources.lookup_single_language(lang, "discriminant-integerquadratic.subsitiute",Some(&args)).unwrap()
        };
        
        vec![format!("{haupteil}\n{nebenteil}")].into()
    }
}
/*
/// Represents the nature of the roots of a quadratic equation.
///
/// The `NatureOfRoots` enum classifies the nature of the roots of a quadratic equation.
/// It provides three variants, indicating whether the equation has distinct real roots,
/// a single root, or no real roots (which implies two complex roots).
#[derive(Debug)]
pub enum NatureOfRoots { 
    /// Indicates that the quadratic equation has two distinct real roots.
    DistinctRealRoots,
    /// Indicates that the quadratic equation has a single real root.
    SingleRoot,
    /// Indicates that the quadratic equation has no real roots, implying two complex roots.
    NoRealRoots  // 2 complex roots
}

impl<T> Discriminant<T> where T : PartialEq + PartialOrd  + From<u8> {
    /// Determines the nature of the roots of a quadratic equation based on its discriminant value.
    ///
    /// The `determine_nature_of_roots` method calculates the discriminant of a quadratic equation
    /// and classifies its nature of roots. It returns an instance of the `NatureOfRoots` enum to
    /// indicate whether the equation has distinct real roots, a single real root, or no real roots
    /// (implying two complex roots).
    ///
    /// # Returns
    ///
    /// - `NatureOfRoots::DistinctRealRoots`: If the discriminant is greater than 1.
    /// - `NatureOfRoots::SingleRoot`: If the discriminant is equal to 1.
    /// - `NatureOfRoots::NoRealRoots`: If the discriminant is less than 1, implying two complex roots.
    pub fn determine_nature_of_roots(&self) -> NatureOfRoots {
        if self.0 > 1u8.into() {
            return NatureOfRoots::DistinctRealRoots;
        }

        if self.0 == 1u8.into() {
            return NatureOfRoots::SingleRoot;
        }

        return NatureOfRoots::NoRealRoots;
    }
}*/