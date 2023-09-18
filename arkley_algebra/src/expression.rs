use std::ops::{Add,Sub,Mul,Div};

use crate::Term;

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
    /// `Note` : TODO Create function for it
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

impl Expression {
    fn search_matching_term(&self,target : &Term,on_found : impl Fn(&Term) -> Expression + Clone) -> Option<Expression> {
        match self {
            // stop searching just expression can't be operated on like (..) , (yes ik its not true and a limitation but thats a problem for later)
            Expression::Nested(_) => None,
            Expression::Term(term) => match term == target {
                true => Some(on_found(term)),
                false => Some(self.clone())
            },
            Expression::Plus(right, left)
            | Expression::Minus(right, left)
            | Expression::Mal(right, left)
            | Expression::Durch(right, left) => {
                // Try searching in the right subtree first
                if let Some(expression) = right.search_matching_term(target,on_found.clone()) {
                    // If found in the right subtree, add the current node to the path and return it
                    return Some(expression);
                }

                // If not found in the left subtree, try searching in the left subtree
                if let Some(expression) = left.search_matching_term(target,on_found) {
                    // If found in the left subtree, add the current node to the path and return it
                    return Some(expression);
                }

                // If not found in either subtree, return right operation left
                None
            }
        }
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
/*
impl Add for Term {
    type Output = Expression;

    fn add(self,other : Term) -> Self::Output {
        match (self, other) {
            (Expression::Term(t1), Expression::Term(t2)) => t1 + t2
            //(not_term, other @ Expression::Term(_)) => not_term.evaluate() + other,
            //(self_ @ Expression::Term(_), not_term) => self_ + not_term.evaluate(),
            //(not_term1, not_term2) => not_term1.evaluate() + not_term2.evaluate(),
        }
    }
}*/

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
    fn test_search_matching_term_with_term_a() {
        // Create a sample expression tree for testing
        let term_a = create_term_with_variable(1.0,'x',1.0); // Define your Term here

        // Build the expression tree
        let expression_tree = Expression::new_plus(
            Expression::new_term(term_a.clone()),
            Expression::new_minus(
                Expression::new_term(term_a.clone()), // Another Term for the right subtree
                Expression::new_nested(Expression::Term(Term::new(Number::Decimal(1.0)))), // Nested Term
            ),
        );

        // aka x + x - (1)
        // or in a more 'tree' form x plus x - (1) where brackets seperate terms
        assert_eq!(&expression_tree.to_string(),"x + x - (1)");

        // Define a closure to test the search_matching_term function
        let on_found = |term : &Term| {
            term.clone() + term_a.clone()
        };

        let result = expression_tree.search_matching_term(&term_a, on_found);

      //  println!("{}", result.clone().map_or_else(|| "NONE".to_string(), |r| r.to_string()));

        // Test case for searching for term_a
        assert_eq!(result.is_some(),true);

        // check if result correct
        let expected = Expression::new_minus(
            create_term_with_variable(2.0,'x',1.0).into(),
            Term::new(Number::Decimal(1.0)).into()
        );

        assert_eq!(result,Some(expected));
    }

    /*
    #[test]
    fn test_try_set_variable_value_variable_not_found() {
        // Create a simple expression: 2x^3 + 3y^2.
        let term1 = Term::new_with_variable(Number::Decimal(2.0), Variables::from([('x', Number::Decimal(3.0))]));
        let term2 = Term::new_with_variable(Number::Decimal(3.0), Variables::from([('y', Number::Decimal(2.0))]));
        let mut expression = Expression::new_plus(term1.clone().into(), term2.clone().into());
    
        // Try to set the value of variable 'z' (not present in the expression).
        let result = expression.try_set_variable_value(&'z', Number::Decimal(1.0));
    
        // Assert that the value of variable 'z' has not been updated in the expression.
        assert_eq!(result, None);
    
        // Check if the expression remains unchanged.
        assert_eq!(expression, Expression::new_plus(term1.into(), term2.into()));
    }

    #[test]
    fn test_try_set_variable_value_single_variable() {
        // Create a simple expression: 2x^3.
        let term1 = Term::new_with_variable(Number::Decimal(2.0), Variables::from([('x', Number::Decimal(3.0))]));
        let mut expression = Expression::Term(term1);

        // Try to set the value of variable 'x' to 5.
        let result = expression.try_set_variable_value(&'x', Number::Decimal(5.0));

        // Assert that the value of variable 'x' has been updated in the expression.
        assert_eq!(result, Some(()));

        // Check if the updated expression is as expected: 2x^3 (x = 5) which is 250.
        let et = Term::new(Number::Decimal(250.0));
        let expected_expression = Expression::new_term(et);
        assert_eq!(expression, expected_expression);
    }*/
}