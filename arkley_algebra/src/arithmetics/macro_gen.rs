use std::ops::{Add,Sub,Mul,Div};
use num_notation::{Number,Pow};

use crate::{Term,Expression,Variables};

#[cfg(feature="function")]
use crate::Function;

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
                    self + Term::from(other)

                }
            }

            impl Add<$t> for Expression {
                type Output = Expression;
                fn add(self, other: $t) -> Expression {
                    self + Term::from(other)
                }
            }

            #[cfg(feature="describe")]
            impl DescribeAdd<$t> for Term {
                fn describe_add(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_add(Term::from(other),resources,lang)
                }
            }

            #[cfg(feature="function")]
            impl Add<$t> for Function  {
                type Output = Expression; 
                fn add(self, rhs: $t) -> Self::Output {
                    self + Term::from(rhs)
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

            #[cfg(feature="describe")]
            impl DescribeSub<$t> for Term {
                fn describe_sub(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_sub(Term::from(other),resources,lang)
                }
            }

            #[cfg(feature="function")]
            impl Sub<$t> for Function  {
                type Output = Expression; 
                fn sub(self, rhs: $t) -> Self::Output {
                    self - Term::from(rhs)
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

            #[cfg(feature="describe")]
            impl DescribeMul<$t> for Term {
                fn describe_mul(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_mul(Term::from(other),resources,lang)
                }
            }

            #[cfg(feature="function")]
            impl Mul<$t> for Function  {
                type Output = Expression; 
                fn mul(self, rhs: $t) -> Self::Output {
                    self * Term::from(rhs)
                }
            }
        )*
    };

    (div => $($t : ty),*) => {
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

            #[cfg(feature="describe")]
            impl DescribeDiv<$t> for Term {
                fn describe_div(self,other:$t,resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<Steps> {
                    self.describe_div(Term::from(other),resources,lang)
                }
            }

            #[cfg(feature="function")]
            impl Div<$t> for Function  {
                type Output = Expression; 
                fn div(self, rhs: $t) -> Self::Output {
                    self / Term::from(rhs)
                }
            }
        )*
    };
    (pow => $v:ty => $($t:ty),*) => {
        impl Pow<$v> for Term {
            type Output = Expression;
            fn pow(self, other: $v) -> Expression {
                Expression::new_pow(self,other)
            }
        }

        impl Pow<$v> for Expression {
            type Output = Expression;
            fn pow(self, other: $v) -> Expression {
                Expression::new_pow(self,other)
            }
        }

        #[cfg(feature="function")]
        impl Pow<$v> for Function  {
            type Output = Expression; 
            fn pow(self, other: $v) -> Self::Output {
                Expression::new_pow(self,other)
            }
        }

        $(
            impl Pow<$t> for Term {
                type Output = Expression;
                fn pow(self, other: $t) -> Expression {
                    __pow(self,other as f64)
                }
            }

            impl Pow<$t> for Expression {
                type Output = Expression;
                fn pow(self, other: $t) -> Expression {
                    // TODO : Check if any more simpilications can be done idk eg like 1/5 ^ 5 becomes 1 
                    __pow(self,other as f64)
                }
            }

            #[cfg(feature="function")]
            impl Pow<$t> for Function  {
                type Output = Expression; 
                fn pow(self, other: $t) -> Self::Output {
                    __pow(self,other as f64)

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

fn __pow<T,V>(value : T,other : V) -> Expression where V : From<u8> + PartialEq ,Expression : From<T> + From<V> {
    if other == V::from(1u8) {
        return value.into();
    }

    if other == V::from(0u8) {
        return 1u8.into();
    }

    Expression::new_pow(value,other)
}

primitives_operations!(ops => i8, i16, i32, i64, u8, u16, u32, u64,f32, f64 , Number , Variables);
primitives_operations!(pow => Variables => i8, i16, i32, i64, u8, u16, u32, u64,f32, f64);