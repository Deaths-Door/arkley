/*use std::collections::BTreeMap;

use arkley_numerics::Fraction;

pub(crate) struct Variables(BTreeMap<char,Fraction<i64,i64>>);

impl Variables {
    fn have_same_chars(&self,other : &Variables) -> bool {
        let variable_chars: Vec<_> = self.0.keys().collect();
        let other_chars: Vec<_> = other.0.keys().collect();
        variable_chars == other_chars    
    }
}*/