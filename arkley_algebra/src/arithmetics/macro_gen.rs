use std::ops::{Add,Sub,Mul,Div};
use num_notation::Number;

use crate::{Term,Expression,Variables};

macro_rules! primitives_operations {
    (add => $($t : ty),*) => {
        $(
            impl Add<$t> for Term {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    self + Term::from(other)

                }
            }

            impl Add<$t> for Expression {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    self + Term::from(other)
                }
            }
        )*
    };

    (sub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Term {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    self - Term::from(other)
                }
            }

            impl Sub<$t> for Expression {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    self - Term::from(other)
                }
            }
        )*
    };

    (mul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Term {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    self * Term::from(other)

                }
            }

            impl Mul<$t> for Expression {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    self * Term::from(other)
                }
            }
        )*
    };

    (div => $($t : ty),*) => {
        /* TODO: enable again
        $(
            impl Div<$t> for Term {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    self / Term::from(other)
                }
            }

            impl Div<$t> for Expression {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    self / Term::from(other)
                }
            }
        )**/
    };

    (ops => $($t:ty),*) => {
        $(
            primitives_operations!(add => $t);
            primitives_operations!(sub => $t);
            primitives_operations!(mul => $t);
            primitives_operations!(div => $t);
        )*
    };
}

primitives_operations!(ops => i8, i16, i32, i64, u8, u16, u32, u64,f32, f64 , Number , Variables);