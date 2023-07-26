use crate::Term;

/// An enum representing a mathematical expression.
///
/// The `Expression` enum allows building complex mathematical expressions
pub enum Expression {
    /// Represents a basic unit in a mathematical expression.
    Term(Term),

    /// Represents the addition of two expressions.
    ///
    /// The `Plus` variant is a binary operator(+) that takes two `Expression` values as its operands.
    Plus(Box<Expression>,Box<Expression>),

    /// Represents the subtraction of two expressions.
    ///
    /// The `Minus` variant is a binary operator(-) that takes two `Expression` values as its operands.
    Minus(Box<Expression>,Box<Expression>),

    /// Represents the multiplication of two expressions.
    ///
    /// The `Mal` variant is a binary operator(*) that takes two `Expression` values as its operands.
    Mal(Box<Expression>,Box<Expression>),

    /// Represents the division of two expressions.
    ///
    /// The `Durch` variant is a binary operator(/) that takes two `Expression` values as its operands.
    /// `Note`: TODO Use `Fraction<Expression>` instead once `ArithmeticCore` trait is implemented for `Expression`.
    Durch(Box<Expression>,Box<Expression>),
}

impl Expression {
    /// Create a new `Expression` containing a single `Term`.
    ///
    /// The `new_term` function wraps the provided `Term` into an `Expression::Term` variant.
    pub fn new_term(term: Term) -> Self {
        Expression::Term(term)
    }

    /// Create a new `Expression` representing the addition of two expressions.
    ///
    /// The `new_plus` function constructs an `Expression` with the `Expression::Plus` variant,
    /// combining two expressions as operands in an addition operation (`+`).
    pub fn new_plus(left: Expression, right: Expression) -> Self {
        Expression::Plus(Box::new(left), Box::new(right))
    }

    /// Create a new `Expression` representing the subtraction of two expressions.
    ///
    /// The `new_minus` function constructs an `Expression` with the `Expression::Minus` variant,
    /// combining two expressions as operands in a subtraction operation (`-`).
    pub fn new_minus(left: Expression, right: Expression) -> Self {
        Expression::Minus(Box::new(left), Box::new(right))
    }

    /// Create a new `Expression` representing the multiplication of two expressions.
    ///
    /// The `new_mal` function constructs an `Expression` with the `Expression::Mal` variant,
    /// combining two expressions as operands in a multiplication operation (`*`).
    pub fn new_mal(left: Expression, right: Expression) -> Self {
        Expression::Mal(Box::new(left), Box::new(right))
    }

    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The `new_durch` function constructs an `Expression` with the `Expression::Durch` variant,
    /// combining two expressions as operands in a division operation (`/`).
    pub fn new_durch(left: Expression, right: Expression) -> Self {
        Expression::Durch(Box::new(left), Box::new(right))
    }
}

impl From<Term> for Expression {
    fn from(term : Term) -> Self {
        Expression::new_term(term)
    }
}