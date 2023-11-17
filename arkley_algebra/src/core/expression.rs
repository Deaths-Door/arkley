use num_notation::{Number, Signed, One};

use crate::{
    Term, ArithmeticOperation, Variables, Function
};

/// An enum representing a mathematical expression.
///
/// The `Expression` enum allows building complex mathematical expressions
#[derive(Clone,Hash)]
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

    /// Represents a mathematical expression that corresponds to a function.
    ///
    /// The `Function` variant represents a mathematical expression that is a function.
    /// It includes the name of the function as a string.
    ///
    /// - `name`: The name of the function, represented as a string. This can be used to
    ///   identify the specific mathematical function being applied.
    #[cfg(feature="function")]
    Function(Function)
}

// To create Self
impl Expression {
    /// Create a new `Expression` containing a single `Term`.
    ///
    /// The `new_term` function wraps the provided `Term` into an `Expression::Term` variant.
    pub const fn new_term(term: Term) -> Self {
        Expression::Term(term)
    }

    #[inline]
    pub(crate) fn new_binary(operation: ArithmeticOperation,left: Expression,right: Expression) -> Self {
        Expression::Binary { operation , left : Box::new(left) , right : Box::new(right) }
    }

    /// Create a new `Expression` representing the addition of two expressions.
    ///
    /// The `new_plus` function constructs an `Expression` with the [ArithmeticOperation::Plus] variant,
    /// combining two expressions as operands in an addition operation (`+`).
    pub fn new_plus(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Plus,left,right)
    }

    /// Create a new `Expression` representing the subtraction of two expressions.
    ///
    /// The `new_minus` function constructs an `Expression` with the [ArithmeticOperation::Minus] variant,
    /// combining two expressions as operands in a subtraction operation (`-`).
    pub fn new_minus(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Minus,left,right)
    }

    /// Create a new `Expression` representing the multiplication of two expressions.
    ///
    /// The `new_mal` function constructs an `Expression` with the [ArithmeticOperation::Mal] variant,
    /// combining two expressions as operands in a multiplication operation (`*`).
    pub fn new_mal(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Mal,left,right)
    }

    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The function constructs an `Expression` with the [ArithmeticOperation::Durch] variant,
    /// combining two expressions as operands in a division operation (`/`).
    pub fn new_durch(left: Expression, right: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Durch,left,right)
    }

    
    /// Create a new `Expression` representing the exponention of two expressions.
    ///
    /// The function constructs an `Expression` with the [ArithmeticOperation::Pow] variant,
    /// combining two expressions as operands in a power operation (`^`).
    pub fn new_pow(base: Expression, exponent: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Pow,base,exponent)
    }

    
    /// Create a new `Expression` representing the division of two expressions.
    ///
    /// The function constructs an `Expression` with the [ArithmeticOperation::Root] variant,
    /// combining two expressions as operands in a root operation
    pub fn new_root(n: Expression, expression: Expression) -> Self {
        Self::new_binary(ArithmeticOperation::Root,n,expression)
    }

    /// Creates a new `Expression` representing a mathematical function.
    ///
    /// This function creates a new `Expression` of the `Function` variant with the provided function name
    ///
    #[cfg(feature="function")]
    pub fn new_function(func : Function) -> Self {
        Self::Function(func)
    }
}

impl From<Term> for Expression {
    fn from(value : Term) -> Self {
        Expression::new_term(value)
    }
}

impl From<Number> for Expression {
    fn from(value : Number) -> Self {
        Expression::new_term(value.into())
    }
}

impl From<Variables> for Expression {
    fn from(value : Variables) -> Self {
        Term::new_with_variable(Number::Decimal(1.0),value).into()
    }
}

impl From<char> for Expression {
    fn from(value :char) -> Self {
        Term::from(value).into()
    }
}

impl From<Function> for Expression {
    fn from(value: Function) -> Self {
        Self::new_function(value)
    }
}

macro_rules! from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Expression {
                fn from(value : $t) -> Self {
                    Expression::new_term(num_notation::Number::Decimal(value as f64).into())
                }
            } 
        )*
    };
}

from!(u8,u16,u32,u64,i8,i16,i32,i64,f32,f64,usize);

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
            #[cfg(feature="function")]
            Self::Function(func) => write!(f,"{func}"),
            Self::Binary { operation , left , right } 
                if operation == &Plus => write!(f,"{left} + {right}"),
                Self::Binary { operation , left , right } 
                if operation == &Minus => write!(f,"{left} - {right}"),
            Self::Binary { operation , left , right } 
                if operation == &Mal => {
                    // Note : NO FUCKING CLUE WHY IT WORKS EXCEPT I WROTE IT AND NOW HAVE NO CLUE
                    let s = format!("{left}");

                    match s.char_indices().find(|(_,c)| "+*-/".contains(*c)) {
                        None => write!(f,"{s}")?,
                        Some((pos,op)) => match op == '-' && s.chars().nth(pos + 1).map(|c| c.is_digit(10) || c.is_ascii_lowercase()).unwrap_or(false) {
                            true => write!(f,"{s}")?,
                            false => write!(f,"({s})")?,
                        } 
                    };

                    match **right {
                        Expression::Function(_) => write!(f,"{right}"),
                        _ => write!(f,"({right})")
                    }
                },
            Self::Binary { operation , left , right } 
                if operation == &Durch => {
                    match **left {
                        Self::Term(_) | Self::Function(_) => write!(f,"{left}"),
                        _ => write!(f,"({left})")
                    }?;

                    write!(f,"/")?;

                    match **right {
                        Self::Function(_) | Self::Term(_) => write!(f,"{right}"),
                        _ => write!(f,"({right})")
                    }
                },
            Self::Binary { operation, left : base, right: exponent } 
                if operation == &Pow => {
                    match **base {
                        Self::Term(_) | Self::Function(_) => write!(f,"{base}"),
                        _ => write!(f,"({base})")
                    }?;

                    write!(f,"^")?;

                    match **exponent {
                        Self::Function(_) => write!(f,"{exponent}"),
                        // if coeff is pos and vars empty 
                        // if coeff not 1 and vars.length.isone
                        // else brackets
                        Self::Term(ref term) if term.is_numeric_one() || (!term.coefficient.is_one() && term.variables.len() == 1)  => write!(f,"{right}"),
                        _ => write!(f,"({exponent})")
                    }
            },
            Self::Binary { operation, left : n, right: expression } 
                if operation == &Root => {
                
            }
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