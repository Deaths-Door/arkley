use std::{cmp::Ordering, collections::HashMap, fmt::Write, ops::Deref};

use num_notation::Number;

use crate::Label;

/// Represents a variable in an algebraic term.
#[derive(Hash,Ord,Eq,Debug,Clone)]
pub struct Variable {
    /// The character representing the variable letter (e.g., 'x', 'y', 'z').
    pub(crate) letter : char,
    /// An optional label associated with the variable. This can be used to provide
    /// additional context or identification for the variable, especially when dealing
    /// with complex terms.
    pub(crate) label : Option<String>
}

impl Variable {
    /// Constructs a new `Variable` with the given letter and optional label.
    ///
    /// # Arguments
    ///
    /// * `letter` - The character representing the variable (e.g., 'x', 'y', 'z').
    /// * `label` - An optional label associated with the variable. This can be used to provide
    ///             additional context or identification for the variable.
    ///
    pub const fn new(letter: char, label: Option<String>) -> Self {
        Self { letter, label }
    }
}

impl From<char> for Variable {
    fn from(value: char) -> Self {
        Variable { letter: value, label: None }
    }
}

impl From<(char,Option<String>)> for Variable {
    fn from(value: (char,Option<String>)) -> Self {
        Variable::new(value.0,value.1)
    }
}

impl From<(char,String)> for Variable {
    fn from(value: (char,String)) -> Self {
        Variable::new(value.0,value.1.into())
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.letter)?;

        if let Some(ref label) = self.label {
            write!(f,"{{{label}}}")?;
        }

        Ok(())
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match (&self.label, &other.label) {
            (None,None) => self.letter == other.letter,
            (Some(_),None)| (None,Some(_)) => false,
            (Some(sl),Some(ol)) => sl == ol && self.letter == other.letter
        }
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.label, &other.label) {
            // If only one variable has a label (None, Some(_)) or (Some(_), None),
            // consider the variable with a label to be "greater".
            // This prioritizes labeled variables for ordering purposes.
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),


            // If both variables lack labels (None), compare their letters for ordering.
            (None, None) => self.letter.partial_cmp(&other.letter),
            
            // If both variables have labels (Some(String), Some(String)),
            // first check if the labels are equal. If so, fall back to comparing letters.
            (Some(l1), Some(l2)) if l1 == l2 => self.letter.partial_cmp(&other.letter),

            // If labels are different, prioritize label comparison.
            (Some(l1), Some(l2)) => l1.partial_cmp(l2)
        }
    }
}

impl<'a> Label<'a> for Variable {
    fn label(&'a self) -> Option<&'a str> {
        self.label.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case('x', None, 'x', None)] // Same letters, no labels
    #[test_case('x', Some("label_x"), 'x', Some("label_x"))] // Same letter, same label

    fn check_equality_for_variables<'a>(
        letter1 : char,label1 : impl Into<Option<&'a str>>,
        letter2 : char,label2 : impl Into<Option<&'a str>>
    ) {
        assert_eq!(Variable::new(letter1,label1.into().map(str::to_string)),Variable::new(letter2,label2.into().map(str::to_string)))
    }

    #[test_case('x', None, 'y', None)] // Different letters, no labels 
    #[test_case('x', Some("label_x"), 'y', None)] // Different letters, one with label (should be unequal)
    #[test_case('x', None, 'x', Some("label_x"))] // Same letter, one with label (should be unequal)
    #[test_case('x', Some("label_x"), 'y', Some("label_y"))] // Different letters, different labels (should be unequal)
    #[test_case('x', Some("label_x"), 'x', Some("label_y"))] // Same letter, different labels (should be unequal)
    #[test_case('y', None, 'x', None)] // Different letters, same empty label
    #[test_case('a', None, 'A', None)] // Different case, same empty label (should be unequal)
    #[test_case('a', Some("Apple"), 'a', Some("apple"))] // Same letter, case-sensitive labels (should be unequal)
    #[test_case('x', Some("VariableX"), 'y', Some("variableX"))] // Same letter, similar labels with different case (should be unequal)
    #[test_case('x', None, 'y', Some(""))] // Same letter, null label vs. empty string (should be unequal)
    #[test_case('x', None, 'x', Some(""))] // Same letter, empty label vs. label with empty string (should be unequal)
    fn check_ne_equality_for_variables<'a>(
        letter1 : char,label1 : impl Into<Option<&'a str>>,
        letter2 : char,label2 : impl Into<Option<&'a str>>
    ) {
        assert_ne!(Variable::new(letter1,label1.into().map(str::to_string)),Variable::new(letter2,label2.into().map(str::to_string)))
    }
}