
use num_notation::Number;

use crate::{Term,ArithmeticOperation,Variables};

/// An enum representing a mathematical expression.
///
/// The `Expression` enum allows building complex mathematical expressions
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Expression {
    /// Represents a basic unit in a mathematical expression.
    Term(Term),

    /// Represents a binary operation between two expressions.
    ///
    /// The `Binary` variant includes the type of mathematical operation and the left
    /// and right operands as boxed expressions.
    Binary {
        /// - `operation`: The type of mathematical operation being performed, such as
        ///    addition, subtraction, multiplication, or division. It is of type `ArithmeticOperation`.
        operation: ArithmeticOperation,

        /// - `left`: The left operand of the binary operation, represented as a boxed `Expression`.
        left: Box<Expression>,

        /// - `right`: The right operand of the binary operation, also represented as a boxed `Expression`.
        right: Box<Expression>,
    },

    /// Represents a more complex expression that contains nested expressions.
    ///
    /// The `Nested` variant allows expressions to be nested within parentheses.
    Nested(Box<Expression>),
}

// To create Self
impl Expression {
    /// Create a new `Expression` containing a single `Term`.
    ///
    /// The `new_term` function wraps the provided `Term` into an `Expression::Term` variant.
    pub const fn new_term(term: Term) -> Self {
        Expression::Term(term)
    }

    pub(crate) fn new_binary(operation: ArithmeticOperation,left: Expression,right: Expression) -> Self {
        Expression::Binary { operation , left : Box::new(left) , right : Box::new(right) }
    }

    /// Create a new `Expression` representing the addition of two expressions.
    ///
    /// The `new_plus` function constructs an `Expression` with the `Expression::Plus` variant,
    /// combining two expressions as operands in an addition operation (`+`).
    pub fn new_plus(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Plus,left,right)
    }

    /// Create a new `Expression` representing the subtraction of two expressions.
    ///
    /// The `new_minus` function constructs an `Expression` with the `Expression::Minus` variant,
    /// combining two expressions as operands in a subtraction operation (`-`).
    pub fn new_minus(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Minus,left,right)
    }

    /// Create a new `Expression` representing the multiplication of two expressions.
    ///
    /// The `new_mal` function constructs an `Expression` with the `Expression::Mal` variant,
    /// combining two expressions as operands in a multiplication operation (`*`).
    pub fn new_mal(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Mal,left,right)
    }

    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The `new_durch` function constructs an `Expression` with the `Expression::Durch` variant,
    /// combining two expressions as operands in a division operation (`/`).
    pub fn new_durch(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Durch,left,right)
    }

    /// Create a new `Expression` representing an expression enclosed in parentheses.
    ///
    /// The `new_nested` function constructs an `Expression` with the `Expression::Nested` variant,
    /// containing the provided expression as an expression enclosed in parentheses.
    pub fn new_nested(inner: Expression) -> Self {
        Expression::Nested(Box::new(inner))
    }
}

impl From<Term> for Expression {
    fn from(value : Term) -> Self {
        Expression::new_term(value)
    }
}

impl From<Variables> for Expression {
    fn from(value : Variables) -> Self {
        Term::new_with_variable(Number::Decimal(1.0),value).into()
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Term(term) => write!(f, "{}", term),
            Expression::Nested(inner) => write!(f, "({})", inner),
            Expression::Binary { operation , left , right } => match operation {
                ArithmeticOperation::Plus => {
                    let s = format!("{left} + {right}").replace("0 + ","").replace(" + 0", "");
                    write!(f,"{s}")
                },
                ArithmeticOperation::Minus => {
                    let s = format!("{left} - {right}").replace("0 - ","-").replace(" - 0", "");
                    write!(f,"{s}")
                },
                ArithmeticOperation::Mal => {
                    let s = format!("{left}");

                    match s.char_indices().find(|(_,c)| "+-*/".contains(*c)) {
                        None => write!(f,"{s}")?,
                        Some((pos,op)) => if op == '-' && s.chars().nth(pos + 1).map(|c| c.is_digit(10) || c.is_ascii_lowercase()).unwrap_or(false) {
                            write!(f,"{s}")?
                        }
                        else {
                            write!(f,"({s})")?
                        }
                    };

                    write!(f,"({right})")
                },
                ArithmeticOperation::Durch => {
                    match **left {
                        Expression::Term(_) => write!(f,"{left}"),
                        _ => write!(f,"({left})")
                    }?;

                    write!(f,"/")?;

                    match **right {
                        Expression::Term(_) => write!(f,"{right}"),
                        _ => write!(f,"({right})")
                    }
                }
            },
        }
    }
}

/// Debug does not remove any extra 0s in the expression tree , eg used to represent -5 as a expression tree would be 0 - 5 where the 0 is ignored by the expression tree
impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Term(term) => write!(f, "{}", term),
            Expression::Nested(inner) => write!(f, "({})", inner),
            Expression::Binary { operation , left , right } => match operation {
                ArithmeticOperation::Plus => write!(f,"{left} + {right}"),
                ArithmeticOperation::Minus => write!(f, "{left} - {right}"),
                ArithmeticOperation::Mal => {
                    let s = format!("{left}");

                    match s.char_indices().find(|(_,c)| "+-*/".contains(*c)) {
                        None => write!(f,"{s}")?,
                        Some((pos,op)) => if op == '-' && s.chars().nth(pos + 1).map(|c| c.is_digit(10) || c.is_ascii_lowercase()).unwrap_or(false) {
                            write!(f,"{s}")?
                        }
                        else {
                            write!(f,"({s})")?
                        }
                    
                    };

                    write!(f,"({right})")
                },
                ArithmeticOperation::Durch => {
                    match **left {
                        Expression::Term(_) => write!(f,"{left}"),
                        _ => write!(f,"({left})")
                    }?;

                    write!(f,"/")?;

                    match **right {
                        Expression::Term(_) => write!(f,"{right}"),
                        _ => write!(f,"({right})")
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Helper function to create a Term with a single variable.
    fn create_term_with_variable(coeff: f64, var: char, exp: f64) -> Term {
        let mut variables = Variables::new();
        variables.insert(var, Number::Decimal(exp));
        Term::new_with_variable(Number::Decimal(coeff), variables)
    }   

    #[test]
    fn display_for_term() {
        let term = Term::new(Number::Decimal(3.14));
        let expression = Expression::Term(term);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Term implementation
        let expected = "3.14";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_nested_expression() {
        let inner_term = Term::new(Number::Decimal(2.0));
        let expression = Expression::new_nested(inner_term.into());

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Nested variant
        let expected = "(2)";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_plus_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(2.5, 'x', 2.0);
        let term2 = create_term_with_variable(3.5, 'x', 2.0);
        let expression = Expression::new_plus(term1.into(),term2.into());

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Plus variant
        let expected = "2.5x^2 + 3.5x^2";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_minus_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(5.0, 'x', 3.0);
        let term2 = create_term_with_variable(2.5, 'x', 3.0);
        let expression = Expression::new_minus(term1.into(),term2.into());


        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Minus variant
        let expected = "5x^3 - 2.5x^3";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_mal_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(2.0, 'x', 1.0);
        let term2 = create_term_with_variable(3.0, 'x', 2.0);
        let expression = Expression::new_mal(term1.into(),term2.into());


        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Mal variant
        let expected = "2x(3x^2)";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_durch_expression() {
        // Create two terms...
        let term1 = create_term_with_variable(6.0, 'x', 3.0);
        let term2 = create_term_with_variable(2.0, 'x', 1.0);
        let expression = Expression::new_durch(term1.into(),term2.into());


        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Durch variant
        let expected = "6x^3/2x";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_durch_expression_with_bracke() {
        // Create two terms...
        let term1 = create_term_with_variable(6.0, 'x', 3.0);
        let term2 = create_term_with_variable(2.0, 'x', 1.0);
        let inner = Expression::new_plus(term1.clone().into(),term2.into());
        let expression = Expression::new_durch(term1.into(),inner);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Durch variant
        let expected = "6x^3/(6x^3 + 2x)";

        assert_eq!(formatted, expected);
    }
}