use std::ops::{Add,Sub,Mul,Div,Neg};

use num_notation::Number;

use crate::{Term,Variables};

/// An enum representing a mathematical expression.
///
/// The `Expression` enum allows building complex mathematical expressions
#[derive(Clone)]
#[cfg_attr(test,derive(PartialEq))]
pub enum Expression {
    /// Represents a basic unit in a mathematical expression.
    Term(Term),

    /// Represents the addition of two terms.
    ///
    /// The `Plus` variant is a binary operator (+) that takes two `Term` values as its operands.
    Plus(Box<Expression>,Box<Expression>),

    /// Represents the subtraction of two terms.
    ///
    /// The `Minus` variant is a binary operator (-) that takes two `Term` values as its operands.
    Minus(Box<Expression>,Box<Expression>),

    /// Represents the multiplication of two terms.
    ///
    /// The `Mal` variant is a binary operator (*) that takes two `Term` values as its operands.
    Mal(Box<Expression>,Box<Expression>),

    /// Represents the division of two terms.
    ///
    /// The `Durch` variant is a binary operator (/) that takes two `Term` values as its operands.
    Durch(Box<Expression>,Box<Expression>),

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

    /// Create a new `Expression` representing an expression enclosed in parentheses.
    ///
    /// The `new_nested` function constructs an `Expression` with the `Expression::Nested` variant,
    /// containing the provided expression as an expression enclosed in parentheses.
    pub fn new_nested(inner: Expression) -> Self {
        Expression::Nested(Box::new(inner))
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

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{self}")
    }
}

impl From<Term> for Expression {
    fn from(value : Term) -> Self {
        Expression::new_term(value)
    }
}

impl From<Number> for Expression {
    fn from(value : Number) -> Self {
        Term::new(value).into()
    }
}

impl From<Variables> for Expression {
    fn from(value : Variables) -> Self {
        Term::new_with_variable(Number::Decimal(1.0),value).into()
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {     
        match self {
            Expression::Term(term) => Expression::new_term(-term),
            Expression::Plus(right,left) => Expression::new_minus(-*right,-*left),
            Expression::Minus(right,left) => Expression::new_plus(-*right,-*left),
            Expression::Nested(inner) => Expression::new_nested(-*inner),
            _ => self
        }
    }
}

/*
impl Mul<Term> for Expression {
    type Output = Self;

    fn mul(self,other : Term) -> Expression {
        match self {
            Expression::Term(term) => (other * term).into(),
            Expression::Nested(inner) => (*inner * other).into(),
            //right * other *left + other  into expression .combine_all_terms.... is answer
            Expression::Plus(right, left) | Expression::Minus(right, left) => 
            _ => todo!()
            //  Expression::Mal(right, left)| Expression::Durch(right, left) => {
        }

        // (2x)(2x +  ..1 + y) for + and - yes
        // (2x)()
    }
}*/
/*
macro_rules! primitives! {
    
};*/

#[cfg(test)]
mod tests {
    use super::*;
    use num_notation::Number;
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
        let expression = Expression::new_term(term);

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
        let expression = Expression::new_plus(term1.into(), term2.into());

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
        let expression = Expression::new_minus(term1.into(), term2.into());

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
        let expression = Expression::new_mal(term1.into(), term2.into());

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
        let expression = Expression::new_durch(term1.into(), term2.into());

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
        let inner_expression = Expression::new_plus(inner_term1.into(), inner_term2.into());

        // Create a nested expression...
        let expression = Expression::Nested(Box::new(inner_expression));

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Nested variant
        let expected = "(2x + 3x^2)";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_neg() {
        // x + x - (x)
        let expression = Expression::new_plus(
            create_term_with_variable(1.0,'x',1.0).into(),
            Expression::new_minus(
                create_term_with_variable(1.0,'x',1.0).into(),
                Expression::new_nested(create_term_with_variable(1.0,'x',1.0).into())
            )
        );

        let negated_expression = -expression;

        let expected_expression = Expression::new_minus(
            create_term_with_variable(-1.0,'x',1.0).into(),
            Expression::new_plus(
                create_term_with_variable(-1.0,'x',1.0).into(),
                Expression::new_nested(create_term_with_variable(-1.0,'x',1.0).into())
            )
        );

        assert_eq!(negated_expression, expected_expression);
    }
}