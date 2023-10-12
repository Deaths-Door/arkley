use num_notation::fraction::Num;

use crate::Quadratic;


#[derive(Clone)]
pub struct IntegerQuadratic<T> where T: Num {
    a: T,
    b: T,
    c: T,
}

impl Quadratic for IntegerQuadratic<T> where T: Num {
    
}

impl<T> IntegerQuadratic<T> where T: Num {
    pub const fn new( a: T,b: T,c: T) -> Self {
        Self { a, b, c }
    }
}
