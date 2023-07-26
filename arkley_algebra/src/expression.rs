use crate::Term;

/// An enum representing a mathematical expression.
///
/// The `Expression` enum allows building complex mathematical expressions
/// `Note` : TODO CHECK IF #[derive(PartialEq)] is enough
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expression {
    /// Represents a basic unit in a mathematical expression.
    Term(Term),

    /// Represents the addition of two terms.
    ///
    /// The `Plus` variant is a binary operator (+) that takes two `Term` values as its operands.
    Plus(Term, Term),

    /// Represents the subtraction of two terms.
    ///
    /// The `Minus` variant is a binary operator (-) that takes two `Term` values as its operands.
    Minus(Term, Term),

    /// Represents the multiplication of two terms.
    ///
    /// The `Mal` variant is a binary operator (*) that takes two `Term` values as its operands.
    Mal(Term, Term),

    /// Represents the division of two terms.
    ///
    /// The `Durch` variant is a binary operator (/) that takes two `Term` values as its operands.
    /// `Note`: TODO Use `Fraction<Term>` instead once the `ArithmeticCore` trait is implemented for `Term`.
    Durch(Term, Term),

    /// Represents a more complex expression that contains nested expressions that contain `()` 
    Nested(Box<Expression>),
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
    pub fn new_plus(left: Term, right: Term) -> Self {
        Expression::Plus(left, right)
    }

    /// Create a new `Expression` representing the subtraction of two expressions.
    ///
    /// The `new_minus` function constructs an `Expression` with the `Expression::Minus` variant,
    /// combining two expressions as operands in a subtraction operation (`-`).
    pub fn new_minus(left : Term , right : Term) -> Self {
        Expression::Minus(left, right)
    }

    /// Create a new `Expression` representing the multiplication of two expressions.
    ///
    /// The `new_mal` function constructs an `Expression` with the `Expression::Mal` variant,
    /// combining two expressions as operands in a multiplication operation (`*`).
    pub fn new_mal(left : Term , right : Term) -> Self {
        Expression::Mal(left, right)
    }

    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The `new_durch` function constructs an `Expression` with the `Expression::Durch` variant,
    /// combining two expressions as operands in a division operation (`/`).
    pub fn new_durch(left : Term , right : Term) -> Self {
        Expression::Durch(left, right)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Term(term) => write!(f, "{}", term),
            Expression::Plus(left, right) => write!(f, "{} + {}", left, right),
            Expression::Minus(left, right) => write!(f, "{} - {}", left, right),
            Expression::Mal(left, right) => write!(f, "{} * {}", left, right),
            Expression::Durch(left, right) => write!(f, "{} / {}", left, right),
            Expression::Nested(inner) => write!(f, "({})", inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arkley_numerics::Number;
    use crate::Variables;
    
    // Helper function to create a Term with a single variable.
    fn create_term_with_variable(coeff: f64, var: char, exp: f64) -> Term {
        let mut variables = Variables::new();
        variables.insert(var, Number::Decimal(exp));
        Term::new_with_variable(Number::Decimal(coeff), variables)
    }

    #[test]
    fn test_display_for_term() {
        let term = Term::new(Number::Decimal(3.14));
        let expression = Expression::Term(term);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Term implementation
        let expected = "3.14";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_for_plus_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(2.5, 'x', 2.0);
        let term2 = create_term_with_variable(3.5, 'x', 2.0);
        let expression = Expression::Plus(term1, term2);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Plus variant
        let expected = "2.5x^2 + 3.5x^2";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_for_minus_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(5.0, 'x', 3.0);
        let term2 = create_term_with_variable(2.5, 'x', 3.0);
        let expression = Expression::Minus(term1, term2);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Minus variant
        let expected = "5x^3 - 2.5x^3";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_for_mal_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(2.0, 'x', 1.0);
        let term2 = create_term_with_variable(3.0, 'x', 2.0);
        let expression = Expression::Mal(term1, term2);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Mal variant
        let expected = "2x * 3x^2";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_for_durch_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(6.0, 'x', 3.0);
        let term2 = create_term_with_variable(2.0, 'x', 1.0);
        let expression = Expression::Durch(term1, term2);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Durch variant
        let expected = "6x^3 / 2x";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_display_for_nested_expression() {
        // Create an inner expression...
        let inner_term1 = create_term_with_variable(2.0, 'x', 1.0);
        let inner_term2 = create_term_with_variable(3.0, 'x', 2.0);
        let inner_expression = Expression::Plus(inner_term1, inner_term2);

        // Create a nested expression...
        let expression = Expression::Nested(Box::new(inner_expression));

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Nested variant
        let expected = "(2x + 3x^2)";

        assert_eq!(formatted, expected);
    }

}