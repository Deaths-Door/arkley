use arkley_describe::Describe;

impl Describe for Fraction<T> where T : ArithmeticCore {
    type Output = Step;

    fn describe(
        self,
        other: $t,
        filter_level: Option<FilterLevel>,
        operation: DescribeOperation,
    ) -> Option<Self::Output> {
        match filter_level.map(|level| level > FilterLevel::Advanced).unwrap_or(true) {
            false => None,
            true => todo!("...")
        }
    }
}