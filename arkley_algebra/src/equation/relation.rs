
/// An enumeration representing different relational operators.
#[derive(PartialEq, Clone, strum::Display)]
pub enum RelationalOperator {
    /// The equal-to operator: `=`
    #[strum(serialize = "=")]
    Equal,
    /// The greater-than operator: `>`
    #[strum(serialize = ">")]
    GreaterThan,
    /// The less-than operator: `<`
    #[strum(serialize = "<")]
    LessThan,
}

impl std::fmt::Debug for RelationalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
