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