use std::fmt::Write;

use dyn_clone::clone_box;
use num_notation::{Number, One, Signed};
use crate::{ArithmeticOperation, CustomizableExpression, Term};

/// An enum representing a mathematical expression.
///
/// The `Expression` enum allows building complex mathematical expressions
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
        ///    addition, subtraction, multiplication, or division. It is of type [ArithmeticOperation::Pow].
        /// 
        /// - For **exponentiation** (e.g., `2 ^ 3`), set to `ArithmeticOperation::Pow`.
        ///   In this case, `left` is the base, and `right` is the exponent.
        ///
        /// - For **nth root** (e.g., `√(25)` for square root), set to [ArithmeticOperation::Root].
        ///   In this case, `left` is n, and `right` is the expression for which we want the nth root.
        operation: ArithmeticOperation,

        /// - `left`: The left operand of the binary operation, represented as a boxed `Expression`.
        left: Box<Expression>,

        /// - `right`: The right operand of the binary operation, also represented as a boxed `Expression`.
        right: Box<Expression>,
    },

    /// Represents a custom or extended expression type.
    Custom(Box<dyn CustomizableExpression>)
}

// To create Self
impl Expression {
    /// Create a new `Expression` containing a single `Term`.
    ///
    /// The `new_term` function wraps the provided `Term` into an `Expression::Term` variant.
    pub fn new_term<T>(term: T) -> Self where T : Into<Term> {
        Expression::Term(T::into(term))
    }

    #[inline]
    pub(crate) fn new_binary<L,R>(operation: ArithmeticOperation,left: L,right: R) -> Self where Self : From<L> + From<R> {
        Expression::Binary { operation , left : Box::new(left.into()) , right : Box::new(right.into()) }
    }

    /// Create a new `Expression` representing the addition of two expressions.
    ///
    /// The `new_plus` function constructs an `Expression` with the [ArithmeticOperation::Plus] variant,
    /// combining two expressions as operands in an addition operation (`+`).
    pub fn new_plus<L,R>(left: L, right: R) -> Self  where Self : From<L> + From<R> {
        Self::new_binary(ArithmeticOperation::Plus,left,right)
    }

    /// Create a new `Expression` representing the subtraction of two expressions.
    ///
    /// The `new_minus` function constructs an `Expression` with the [ArithmeticOperation::Minus] variant,
    /// combining two expressions as operands in a subtraction operation (`-`).
    pub fn new_minus<L,R>(left: L, right: R) -> Self  where Self : From<L> + From<R> {
        Self::new_binary(ArithmeticOperation::Minus,left,right)
    }

    /// Create a new `Expression` representing the multiplication of two expressions.
    ///
    /// The `new_mal` function constructs an `Expression` with the [ArithmeticOperation::Mal] variant,
    /// combining two expressions as operands in a multiplication operation (`*`).
    pub fn new_mal<L,R>(left: L, right: R) -> Self  where Self : From<L> + From<R> {
        Self::new_binary(ArithmeticOperation::Mal,left,right)
    }

    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The function constructs an `Expression` with the [ArithmeticOperation::Durch] variant,
    /// combining two expressions as operands in a division operation (`/`).
    pub fn new_durch<L,R>(left: L, right: R) -> Self where Self : From<L> + From<R> {
        Self::new_binary(ArithmeticOperation::Durch,left,right)
    }

    
    /// Create a new `Expression` representing the exponention of two expressions.
    ///
    /// The function constructs an `Expression` with the [ArithmeticOperation::Pow] variant,
    /// combining two expressions as operands in a power operation (`^`).
    pub fn new_pow<L,R>(base: L, exponent: R) -> Self where Self : From<L> + From<R> {
        Self::new_binary(ArithmeticOperation::Pow,base,exponent)
    }

    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The function constructs an `Expression` with the [ArithmeticOperation::Root] variant,
    /// combining two expressions as operands in a root operation
    pub fn new_root<L,R>(n: L, expression: R) -> Self where Self : From<L> + From<R> {
        Self::new_binary(ArithmeticOperation::Root,n,expression)
    }

    /// Constructs a new `Expression` variant of [Expression::Custom].
    pub fn new_custom<T>(value : T) -> Self where T : CustomizableExpression + 'static {
        Self::Custom(Box::new(value))
    }
}


impl<T> From<T> for Expression where Term : From<T> {
    fn from(value: T) -> Self {
        Self::new_term(value)
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            Self::Term(arg0) => Self::Term(arg0.clone()),
            Self::Binary { operation, left, right } => Self::Binary { operation: operation.clone(), left: left.clone(), right: right.clone() },
            Self::Custom(arg0) => Self::Custom(clone_box(&**arg0)),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ArithmeticOperation::*;
        match self {
            Self::Term(term) => write!(f, "{term}"),
            Self::Binary { operation , left , right } 
                if operation == &Plus => write!(f,"{left} + {right}"),
                Self::Binary { operation , left , right } 
                if operation == &Minus => write!(f,"{left} - {right}"),
            Self::Binary { operation , left , right } 
                if operation == &Mal => {
                    match &**left {
                        // Write Nothing
                        Self::Term(term) if term.is_numeric_one() => (),
                        Self::Term(term) if term.is_numeric(-1f64) => f.write_char('-')?,
                        Self::Term(term) => write!(f,"{left}")?,
                        _ =>  write!(f,"({left})")?
                    };

                    write!(f,"({})",right)
                },
            Self::Binary { operation , left , right } 
                if operation == &Durch => {
                    match **left {
                        Self::Term(_) => write!(f,"{left}"),
                        _ => write!(f,"({left})")
                    }?;

                    write!(f,"/")?;

                    match **right {
                        Self::Term(_) => write!(f,"{right}"),
                        _ => write!(f,"({right})")
                    }
                },
            Self::Binary { operation, left : base, right: exponent } 
                if operation == &Pow => {
                    match **base {
                        Self::Term(_) => write!(f,"{base}"),
                        _ => write!(f,"({base})")
                    }?;

                    write!(f,"^")?;

                    match **exponent {
                        Self::Term(ref t) if 
                            (t.variables.is_empty() && t.coefficient.is_positive()) || 
                            (t.coefficient.is_one() && t.variables.len() == 1)  => write!(f,"{exponent}"),
                        _ => write!(f,"({exponent})")
                    }
            },
            Self::Binary { operation, left : n, right: expression } 
                if operation == &Root => {
            
                if let Self::Term(term) = &**n {
                    if term.variables.is_empty() {
                        let sign = if term.coefficient == 2 { ArithmeticOperation::SQRT_SIGN }
                            else if term.coefficient == 3 { ArithmeticOperation::CBRT_SIGN }
                            else { return write!(f,"{expression}^(1/{n})") };
                            
                        return match &**expression {
                            Self::Term(t) if 
                                (t.variables.is_empty() && t.coefficient.is_positive()) || 
                                (t.coefficient.is_one() && t.variables.len() == 1) => write!(f,"{sign}{t}"),
                            _ => write!(f,"{sign}({expression})")
                        }
                    }
                } 
                

                write!(f,"{expression}^(1/{n})")
            }
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use num_notation::Number;

    use super::*;

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
    fn display_for_plus_expression() {
        // Create two terms...
        let term1 = Term::new_with_variable_to(2.5, 'x',2.0);
        let term2 = Term::new_with_variable_to(3.5, 'x', 2.0);
        let expression = Expression::new_plus(term1,term2);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Plus variant
        let expected = "2.5x^2 + 3.5x^2";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_minus_expression() {
        // Create two terms...
        let term1 = Term::new_with_variable_to(5.0, 'x', 3.0);
        let term2 = Term::new_with_variable_to(2.5, 'x', 3.0);
        let expression = Expression::new_minus(term1,term2);


        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Minus variant
        let expected = "5x^3 - 2.5x^3";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_mal_expression() {
        // Create two terms...
        let term1 = Term::new_with_variable_to(2.0, 'x', 1.0);
        let term2 = Term::new_with_variable_to(3.0, 'x', 2.0);
        let expression = Expression::new_mal(term1,term2);


        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Mal variant
        let expected = "2x(3x^2)";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_durch_expression() {
        // Create two terms...
        let term1 = Term::new_with_variable_to(6.0, 'x', 3.0);
        let term2 = Term::new_with_variable_to(2.0, 'x', 1.0);
        let expression = Expression::new_durch(term1,term2);


        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Durch variant
        let expected = "6x^3/2x";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn display_for_durch_expression_with_bracke() {
        // Create two terms...
        let term1 = Term::new_with_variable_to(6.0, 'x', 3.0);
        let term2 = Term::new_with_variable_to(2.0, 'x', 1.0);
        let inner = Expression::new_plus(term1.clone(),term2);
        let expression = Expression::new_durch(term1,inner);

        // Format the expression using the Display trait
        let formatted = format!("{}", expression);

        // Expected output based on the Durch variant
        let expected = "6x^3/(6x^3 + 2x)";

        assert_eq!(formatted, expected);
    }

    #[test]
    fn mul_add_t() {
        assert_eq!(
            Expression::new_plus('a',Expression::new_mal('y','z')).to_string().replace(" ",""),
            "a+y(z)"
        )
    }
}