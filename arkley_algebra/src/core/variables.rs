use std::{cmp::Ordering, collections::BTreeMap, ops::{Deref, DerefMut}};

use itertools::Itertools;
use num_notation::Number;

use crate::Variable;


/// Represents a collection of variables, each associated with a numerical value.

// This struct is a new type introduced due to a change in the underlying data structure
// from BTreeMap to HashMap. This change affects behavior related to key ordering 
// (previously sorted, now based on hash function) and allows for adding new methods 
// specific to the functionality.
#[derive(Default , Debug , Eq, PartialEq, Ord ,Clone)]
pub struct Variables(VariableBtreeMap);
type VariableBtreeMap = BTreeMap<Variable,Number>;

impl Variables {
    /// Creates a new empty `Variables` instance.
    pub fn new() -> Self {
        Self(VariableBtreeMap::new())
    }
}

impl Deref for Variables {
    type Target = VariableBtreeMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<[(Variable,Number); N]> for Variables {
    fn from(value: [(Variable,Number); N]) -> Self {
        Self(VariableBtreeMap::from(value))
    }
}

impl IntoIterator for Variables {
    type Item = <VariableBtreeMap as IntoIterator>::Item;
    type IntoIter = <VariableBtreeMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<<VariableBtreeMap as IntoIterator>::Item> for Variables {
    fn from_iter<T: IntoIterator<Item = <VariableBtreeMap as IntoIterator>::Item>>(iter: T) -> Self {
        Self(VariableBtreeMap::from_iter(iter))
    }
}

impl PartialOrd for Variables {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut map = self.0.iter()
            .filter_map(|(sk,sv)|{
                match other.0.get(sk) {
                    None => Some(Ordering::Greater),
                    Some(ov) => sv.partial_cmp(ov)
                }
            })
            // TODO; remove need of count_by and hashmap using simple int variables
            .counts_by(|v| v);

        let equal_count = map.remove(&Ordering::Equal).unwrap_or(0);
        let greater_count = map.remove(&Ordering::Greater).unwrap_or(0);
        let less_count = map.remove(&Ordering::Less).unwrap_or(0);

        if cfg!(test) {
            println!("{less_count};{equal_count};{greater_count}");
        }

        match greater_count {
            0 => match equal_count {
                0 => match less_count {
                    0 => None,
                    _ => Some(Ordering::Less)
                },
                _ => Some(Ordering::Equal)
            },
            _ => Some(Ordering::Greater)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use test_case::test_case;
    use crate::Variables;

    #[test_case("x","x^2",Ordering::Less)]
    #[test_case("x^2","x",Ordering::Greater)]
    #[test_case("x^2","x^2",Ordering::Equal)]
    #[test_case("x^3","x^2",Ordering::Greater)]
    #[test_case("ax^2","x^2",Ordering::Greater)]
    #[test_case("xy", "yx", Ordering::Equal)] 
    #[test_case("x^2y", "xy^2", Ordering::Greater)]
    #[test_case("x^2y", "x^3", Ordering::Less)]
    #[test_case("axy", "a^2x", Ordering::Greater)] 
    #[test_case("axy", "ayz", Ordering::Less)]   
    #[test_case("axyz", "axy", Ordering::Greater)]
    fn ord(v1 : &'static str,v2 : &'static str,expect : impl Into<Option<Ordering>>) -> Result<(),<Variables as TryFrom<&'static str>>::Error> { 
        let v1 =  Variables::try_from(v1)?;
        let v2 = Variables::try_from(v2)?;
        
        assert_eq!(
            v1.partial_cmp(&v2),
            expect.into()
        );

        Ok(())
    }
}