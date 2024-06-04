use std::{collections::HashMap, ops::{Deref, DerefMut}};

use num_notation::Number;

use crate::Variable;


/// Represents a collection of variables, each associated with a numerical value.

// This struct is a new type introduced due to a change in the underlying data structure
// from BTreeMap to HashMap. This change affects behavior related to key ordering 
// (previously sorted, now based on hash function) and allows for adding new methods 
// specific to the functionality.
#[derive(Default , Debug , Eq, PartialEq ,Clone)]
pub struct Variables(VariablesHashMap);
type VariablesHashMap = HashMap<Variable,Number>;

impl Variables {
    /// Creates a new empty `Variables` instance.
    pub fn new() -> Self {
        Self(VariablesHashMap::new())
    }
}

impl Deref for Variables {
    type Target = VariablesHashMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Variables where VariablesHashMap : From<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl IntoIterator for Variables {
    type Item = <VariablesHashMap as IntoIterator>::Item;
    type IntoIter = <VariablesHashMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<<VariablesHashMap as IntoIterator>::Item> for Variables {
    fn from_iter<T: IntoIterator<Item = <VariablesHashMap as IntoIterator>::Item>>(iter: T) -> Self {
        VariablesHashMap::from_iter(iter).into()
    }
}