use fluent_templates::{StaticLoader, LanguageIdentifier};
use crate::FilterLevel;

/// Represents a generic trait for describing addition operations.
///
/// The associated type `DescribeOutput` specifies the return type of the `describe_add` method.
pub trait DescribeAdd<DescribeOutput = String, Rhs = Self> : std::ops::Add<Rhs> + Sized {
    /// Describes the addition operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the addition.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the addition operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_add(self, other: Rhs, resources: &StaticLoader, filter_level: FilterLevel,lang: &LanguageIdentifier) -> Option<DescribeOutput> {
        match filter_level {
            FilterLevel::Beginner => self.describe_add_beginner(other, resources,lang),
            FilterLevel::Intermediate => self.describe_add_intermediate(other, resources,lang),
            FilterLevel::Advanced => self.describe_add_advanced(other, resources,lang),
        }
    }

    /// Describes the addition operation at the beginner filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the addition.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    ///
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the addition operation
    /// at the beginner filter level, or `None` if not described.
    fn describe_add_beginner(self, other: Rhs, resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the addition operation at the intermediate filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the addition.
    /// - `resources`: A `StaticLoader` used for localization.
    ///- `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the addition operation
    /// at the intermediate filter level, or `None` if not described.
    fn describe_add_intermediate(self, other: Rhs, resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the addition operation at the advanced filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the addition.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the addition operation
    /// at the advanced filter level, or `None` if not described.
    fn describe_add_advanced(self, other: Rhs, resources: &StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
}

/// Represents a generic trait for describing subtraction operations.
///
/// The associated type `DescribeOutput` specifies the return type of the `describe_sub` method.
pub trait DescribeSub<DescribeOutput = String,Rhs = Self>:std::ops::Sub<Rhs> + Sized {
    /// Describes the subtraction operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the subtraction operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_sub(self, other: Rhs, resources: StaticLoader, filter_level: FilterLevel,lang: &LanguageIdentifier) -> Option<DescribeOutput> {
        match filter_level {
            FilterLevel::Beginner => self.describe_sub_beginner(other, resources,lang),
            FilterLevel::Intermediate => self.describe_sub_intermediate(other, resources,lang),
            FilterLevel::Advanced => self.describe_sub_advanced(other, resources,lang),
        }
    }

    /// Describes the subtraction operation at the beginner filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the subtraction operation
    /// at the beginner filter level, or `None` if not described.
    fn describe_sub_beginner(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the subtraction operation at the intermediate filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the subtraction operation
    /// at the intermediate filter level, or `None` if not described.
    fn describe_sub_intermediate(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the subtraction operation at the advanced filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the subtraction.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the subtraction operation
    /// at the advanced filter level, or `None` if not described.
    fn describe_sub_advanced(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
}

/// Represents a generic trait for describing multiplication operations.
///
/// The associated type `DescribeOutput` specifies the return type of the `describe_mul` method.
pub trait DescribeMul<DescribeOutput = String,Rhs = Self>: std::ops::Mul<Rhs> + Sized {
    /// Describes the multiplication operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the multiplication.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the multiplication operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_mul(self, other: Rhs, resources: StaticLoader, filter_level: FilterLevel,lang: &LanguageIdentifier) -> Option<DescribeOutput> {
        match filter_level {
            FilterLevel::Beginner => self.describe_mul_beginner(other, resources,lang),
            FilterLevel::Intermediate => self.describe_mul_intermediate(other, resources,lang),
            FilterLevel::Advanced => self.describe_mul_advanced(other, resources,lang),
        }
    }

    /// Describes the multiplication operation at the beginner filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the multiplication.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the multiplication operation
    /// at the beginner filter level, or `None` if not described.
    fn describe_mul_beginner(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the multiplication operation at the intermediate filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the multiplication.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the multiplication operation
    /// at the intermediate filter level, or `None` if not described.
    fn describe_mul_intermediate(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the multiplication operation at the advanced filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the multiplication.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the multiplication operation
    /// at the advanced filter level, or `None` if not described.
    fn describe_mul_advanced(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
}

/// Represents a generic trait for describing division operations.
///
/// The associated type `DescribeOutput` specifies the return type of the `describe_div` method.
pub trait DescribeDiv<DescribeOutput = String,Rhs = Self>:  std::ops::Div<Rhs> + Sized {
    /// Describes the division operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the division.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the division operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_div(self, other: Rhs, resources: StaticLoader, filter_level: FilterLevel,lang: &LanguageIdentifier) -> Option<DescribeOutput> {
        match filter_level {
            FilterLevel::Beginner => self.describe_div_beginner(other, resources,lang),
            FilterLevel::Intermediate => self.describe_div_intermediate(other, resources,lang),
            FilterLevel::Advanced => self.describe_div_advanced(other, resources,lang),
        }
    }

    /// Describes the division operation at the beginner filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the division.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the division operation
    /// at the beginner filter level, or `None` if not described.
    fn describe_div_beginner(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the division operation at the intermediate filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the division.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the division operation
    /// at the intermediate filter level, or `None` if not described.
    fn describe_div_intermediate(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the division operation at the advanced filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the division.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the division operation
    /// at the advanced filter level, or `None` if not described.
    fn describe_div_advanced(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
}

/// Represents a generic trait for describing exponentiation operations.
///
/// The associated type `DescribeOutput` specifies the return type of the `describe_pow` method.
pub trait DescribePow<DescribeOutput = String,Rhs = Self>: num_traits::Pow<Rhs> +  Sized {
    /// Describes the exponentiation operation between the current instance and the right-hand side `Rhs`.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the exponentiation.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `filter_level`: The `FilterLevel` to control the level of details in the description.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the exponentiation operation,
    /// or `None` if the operation is not described due to the filtering level.
    fn describe_pow(self, other: Rhs, resources: StaticLoader, filter_level: FilterLevel,lang: &LanguageIdentifier) -> Option<DescribeOutput> {
        match filter_level {
            FilterLevel::Beginner => self.describe_pow_beginner(other, resources,lang),
            FilterLevel::Intermediate => self.describe_pow_intermediate(other, resources,lang),
            FilterLevel::Advanced => self.describe_pow_advanced(other, resources,lang),
        }
    }

    /// Describes the exponentiation operation at the beginner filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the exponentiation.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the exponentiation operation
    /// at the beginner filter level, or `None` if not described.
    fn describe_pow_beginner(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the exponentiation operation at the intermediate filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the exponentiation.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the exponentiation operation
    /// at the intermediate filter level, or `None` if not described.
    fn describe_pow_intermediate(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;

    /// Describes the exponentiation operation at the advanced filter level.
    ///
    /// # Parameters
    ///
    /// - `self`: The object on which the method is called.
    /// - `other`: The right-hand side argument of the exponentiation.
    /// - `resources`: A `StaticLoader` used for localization.
    /// - `lang` : Language to be localized into
    /// # Returns
    ///
    /// An `Option<DescribeOutput>` representing the description of the exponentiation operation
    /// at the advanced filter level, or `None` if not described.
    fn describe_pow_advanced(self, other: Rhs, resources: StaticLoader,lang: &LanguageIdentifier) -> Option<DescribeOutput>;
}
