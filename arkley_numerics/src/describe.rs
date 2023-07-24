use arkley_describe::{Describe,DescribeOperation,FilterLevel,Step};
use arkley_traits::ArithmeticCore;

use super::{Fraction,Decimal};

impl<T> Describe for Fraction<T> where T : ArithmeticCore {
    type Output = Step;

    fn describe(
        self,
        _other: Self,
        filter_level: Option<FilterLevel>,
        _operation: DescribeOperation,
    ) -> Option<Self::Output> {
        match filter_level.map(|level| level > FilterLevel::Advanced).unwrap_or(true) {
            false => None,
            true => todo!("not done yet")
        }
    }
}

impl Describe for Decimal {
    type Output = Step;

    fn describe(
        self,
        _other: Self,
        filter_level: Option<FilterLevel>,
        _operation: DescribeOperation,
    ) -> Option<Self::Output> {
        match filter_level.map(|level| level > FilterLevel::Intermediate).unwrap_or(true) {
            false => None,
            true => todo!("not done yet")
        }
    }
}