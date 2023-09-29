use std::ops::{Add,Sub,Mul,Div};
use num_notation::Number;

use crate::{Term,Expression,Variables};

macro_rules! primitives_operations {
    (padd => $($t : ty),*) => {
        $(
            impl Add<$t> for Term {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self + term
                }
            }

            impl Add<$t> for Expression {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self + term
                }
            }
        )*
    };

    (psub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Term {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self - term
                }
            }

            impl Sub<$t> for Expression {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self - term
                }
            }
        )*
    };

    (pmul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Term {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self * term
                }
            }
        )*
    };

    (pdiv => $($t : ty),*) => {
        $(
            impl Div<$t> for Term {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    let n = Number::Decimal(other as f64);
                    let term = Term::from(n);

                    self / term
                }
            }
        )*
    };

    (vadd => $($t : ty),*) => {
        $(
            impl Add<$t> for Term {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self + term
                }
            }

            impl Add<$t> for Expression {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self + term
                }
            }
        )*
    };

    (vsub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Term {
                type Output = Expression;
                fn sub(self, n: $t) -> Expression {
                    let term = Term::from(n);

                    self - term
                }
            }

            impl Sub<$t> for Expression {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self - term
                }
            }
        )*
    };

    (vmul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Term {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self * term
                }
            }
        )*
    };

    (vdiv => $($t : ty),*) => {
        $(
            impl Div<$t> for Term {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    let term = Term::from(other);

                    self / term
                }
            }
        )*
    };

    (pops => $($t:ty),*) => {
        $(
            primitives_operations!(padd => $t);
            primitives_operations!(psub => $t);
            primitives_operations!(pmul => $t);
            primitives_operations!(pdiv => $t);
        )*
    };

    (nvops => $($t : ty),*) => {
        $(
            primitives_operations!(vadd => $t);
            primitives_operations!(vsub => $t);
            primitives_operations!(vmul => $t);
            primitives_operations!(vdiv => $t);
        )*
    }
}

primitives_operations!(pops => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);
primitives_operations!(nvops => Number,Variables);