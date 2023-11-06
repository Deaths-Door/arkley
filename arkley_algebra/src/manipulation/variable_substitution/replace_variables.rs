use std::collections::HashMap;

/// A utility struct for representing variable replacements in a target expression.
///
/// The `VariableReplacements` struct allows you to manage and apply variable replacements within
/// a target mathematical expression. It stores a collection of variable-value pairs for substitution. 
/// It is created by [super::VariableSubstitution::replace_single_variable]
#[derive(Debug, Clone)]
pub struct SingleVariableReplacements<T, V> {
    pub(super) source: T,
    pub(super) variable : char,
    pub(super) value : V,
}

impl<T, V> SingleVariableReplacements<T, V> {
    pub(super) fn new(source: T, variable: char, value: V) -> Self { Self { source, variable, value } }
}

/// A utility struct for representing variable replacements in a target expression.
///
/// The `VariableReplacements` struct allows you to manage and apply variable replacements within
/// a target mathematical expression. It stores a collection of variable-value pairs for substitution. 
/// It is created by [super::VariableSubstitution::replace_variables]
#[derive(Debug, Clone)]
pub struct MultipleVariableReplacements<'a,T, V> {
    pub(super) source: T,
    pub(super) values : &'a HashMap<char,V>,
}

impl<'a, T, V> MultipleVariableReplacements<'a, T, V> {
    pub(super) fn new(source: T, values: &'a HashMap<char,V>) -> Self { Self { source, values } }
}