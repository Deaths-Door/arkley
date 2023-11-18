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
    (pow_primitives => $($t:ty),*) => {
        $(
            impl Pow<$t> for Term {
                type Output = Expression;
                fn pow(self, other: $t) -> Expression {
                    if other == 1u8 as $t {
                        return self.into();
                    }

                    if other == 0u8 as $t {
                        return 1u8.into();
                    }

                    Expression::new_pow(self,other)
                }
            }

            impl Pow<$t> for Expression {
                type Output = Expression;
                fn pow(self, other: $t) -> Expression {
                    
                }
            }

            #[cfg(feature="function")]
            impl Pow<$t> for Function  {
                type Output = Expression; 
                fn pow(self, other: $t) -> Self::Output {
                    if other == 1u8 as $t {
                        return self.into();
                    }

                    if other == 0u8 as $t {
                        return 1u8.into();
                    }

                    Expression::new_pow(self,other)
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
primitives_operations!(pow_primitives => i8, i16, i32, i64, u8, u16, u32, u64,f32, f64);