impl ArithmeticOperation {
    pub(crate) const fn negate_if_plus_or_minus(self) -> Self {
        use ArithmeticOperation::*;
        match self {
            Plus => Minus,
            Minus => Plus,
            _ => self
        }
    }
}