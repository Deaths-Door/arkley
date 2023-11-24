use std::{ops::*, fs::File, io::Write, path::Path};
use num_traits::Pow;
use fluent_templates::{StaticLoader, LanguageIdentifier, Loader};

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

/// A description 'write' the describption to a file 
pub fn write_description_to_file<T,S>(
    resources : &StaticLoader,
    locale : &LanguageIdentifier,
    item : T,
    path : S,
    on_not_described : impl FnOnce() -> (),
) -> std::io::Result<()>
    where S : AsRef<Path>,
    T : Describe,
{
    const LATEX_BEGIN : &[u8] = r"\documentclass{article}\begin{document}".as_bytes();
    const LATEX_END : &[u8]= r"\end{{document}}".as_bytes();

    // Same as File::create_new(path)
    let mut file = File::options().read(true).write(true).create_new(true).open(path.as_ref())?;

    let steps =  item.describe(resources, &locale);
    match steps {
        None =>{
            on_not_described();
            Ok(())
        },
        Some(steps) => {
            file.write_all(LATEX_BEGIN)?;
            file.write_all(steps.join("\n").as_bytes())?;
            file.write_all(LATEX_END)
        }
    }
}