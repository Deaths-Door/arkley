use std::ops::{Add,Sub,Mul,Div};
use num_notation::Number;

use crate::{Term,Expression,Variables};

#[cfg(feature="describe")]
use arkley_describe::{
    DescribeAdd, DescribeSub , DescribeMul , DescribeDiv ,Steps,
    fluent_templates::{StaticLoader, LanguageIdentifier},
};

macro_rules! primitives_operations {
    (add => $($t : ty),*) => {
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

            #[cfg(feature="describe")]
            impl DescribeAdd<$t> for Term {
                fn describe_add(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_add(Term::from(other),resources,lang)
                }
            }
        )*
    };

    (sub => $($t : ty),*) => {
        $(
            impl Sub<$t> for Term {
                type Output = Expression;
                fn sub(self, other: $t) -> Expression {
                    let term = Term::from(other);
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

            #[cfg(feature="describe")]
            impl DescribeSub<$t> for Term {
                fn describe_sub(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_sub(Term::from(other),resources,lang)
                }
            }
        )*
    };

    (mul => $($t : ty),*) => {
        $(
            impl Mul<$t> for Term {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    let term = Term::from(other);
                    self * term
                }
            }

            impl Mul<$t> for Expression {
                type Output = Expression;
                fn mul(self, other: $t) -> Expression {
                    let term = Term::from(other);
                    self - term
                }
            }

            #[cfg(feature="describe")]
            impl DescribeMul<$t> for Term {
                fn describe_mul(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_mul(Term::from(other),resources,lang)
                }
            }
        )*
    };

    (div => $($t : ty),*) => {
        $(
            impl Div<$t> for Term {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    let term = Term::from(other);
                    self / term
                }
            }

            impl Div<$t> for Expression {
                type Output = Expression;
                fn div(self, other: $t) -> Expression {
                    let term = Term::from(other);
                    self - term
                }
            }

            #[cfg(feature="describe")]
            impl DescribeDiv<$t> for Term {
                fn describe_div(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_div(Term::from(other),resources,lang)
                }
            }
        )*
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
