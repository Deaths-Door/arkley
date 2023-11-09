use std::ops::*;
use num_traits::Pow;
use fluent_templates::{StaticLoader, LanguageIdentifier};

/// As the author is lazy and may change the return type in the future
pub type Steps = Vec<String>;

macro_rules! create_describe {
    (op => $name : ident,$t : ident,$fn : ident) => {
        #[doc = concat!("Represents a generic trait for describing ", stringify!($t), " operations.
        The associated type `DescribeOutput` specifies the return type of the method.")]
        pub trait $name <Rhs = Self,DescribeOutput = Steps> : $t<Rhs> + Sized {
            /// Describes the operation between the current instance and the right-hand side `Rhs`.
            /// 
            /// # Parameters
            /// 
            /// - `self`: The object on which the method is called.
            /// - `other`: The right-hand side argument of the subtraction.
            /// - `resources`: A `StaticLoader` used for localization.
            /// - `lang` : Language to be localized into
            /// # Returns
            /// 
            /// An `Option<DescribeOutput>` representing the description of the operation"]
            /// None only when `resources.get(lang,..)` returns None
            fn $fn (self, other: Rhs, resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
        }
    };
}

#[doc = "Represents a generic trait for describing the operations. The associated type `DescribeOutput` specifies the return type of the method."]
pub trait Describe<DescribeOutput = Steps> : Sized {
    /// Describes the operation
    /// 
    /// # Parameters
    /// 
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    /// 
    /// An `Option<DescribeOutput>` representing the description
    /// None only when `resources.get(lang,..)` returns None
    fn describe(self,resources:&StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
}


create_describe!(op => DescribeAdd,Add,describe_add);
create_describe!(op => DescribeSub,Sub,describe_sub);
create_describe!(op => DescribeMul,Mul,describe_mul);
create_describe!(op => DescribeDiv,Div,describe_div);
create_describe!(op => DescribePow,Pow,describe_pow);