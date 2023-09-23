use std::ops::{Add,Sub,Mul,Div,Neg};
use std::collections::{BTreeMap,BTreeSet};
use std::cmp::Ordering;

use num_notation::Number;

use crate::Expression;

impl Neg for Term {
    type Output = Self;

    fn neg(self) -> Self {
        Term::new_with_variable(-self.coefficient,self.variables.clone())
    }
}