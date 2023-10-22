use nom::{IResult, combinator::map};

use crate::Expression;

use super::tokens::Token;

/// Parses a mathematical expression from the input string.
///
/// This function takes an input string and parses it into a mathematical expression. It handles
/// expressions with various levels of complexity, including terms, binary operations, and nested
/// expressions.
///
/// # Arguments
///
/// * `input`: A string containing the mathematical expression to be parsed.

// TODO : Give better reason then None for why its invalid
pub fn parse_expression(input: &str) -> IResult<&str, Option<Expression>> {
    map(Token::into_tokens,|vec : Vec<Token>| Token::into_expression_tree(Token::to_rpn(vec)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_addition() {
        let input_str = "3 + 4";
        let parsed = parse_expression(input_str);
        let expected_expression = Expression::new_plus( 3.0.into(),  4.0.into());

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,Some(expected_expression));
    }

    #[test]
    fn parse_complex_expression() {
        let input_str = "1 + (2 * 3)";
        let parsed = parse_expression(input_str);

        let expected_expression = Expression::new_plus(
            1.0.into(), 
            Expression::new_nested(
                Expression::new_mal(2.0.into(), 3.0.into())
            )
        );
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,Some(expected_expression));
    }

    #[test]
    fn parse_with_implicit_mul() {
        let input_str = "1 + 2(4)";
        let parsed = parse_expression(input_str);

        let expected_expression = Expression::new_plus(
            1.0.into(), 
            Expression::new_mal(2.0.into(), 4.0.into())
        );
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,Some(expected_expression));
    }

    #[test]
    fn parse_expression_with_unary_minus() {
        let input_str = "-5 + 2";
        let parsed = parse_expression(input_str);
        let expected_expression =  Expression::new_plus((-5.0).into(),  2.0.into());

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,Some(expected_expression));
    }

    #[test]
    fn parse_invalid_expression() {
        let input_str = "1 + (2 * 3";
        let parsed = parse_expression(input_str);
       
        let unwrapped = parsed.unwrap().1;
        println!("{}",unwrapped.clone().unwrap());
        assert!(unwrapped.is_none());
    }

    #[test]
    fn parse_expression_with_multiple_operators() {
        let input_str = "2 + 3 * 4 - 5 / 1";
        let parsed = parse_expression(input_str);
        let expected_expression = Expression::new_minus(
            Expression::new_plus(2.0.into(), Expression::new_mal(3.0.into(), 4.0.into())),
            Expression::new_durch(5.0.into(), 1.0.into())
        );

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,Some(expected_expression));
    }

    #[test]
    fn double_brackets() {
        let input_str = "(2 + 3)(4/4)";
        let parsed = parse_expression(input_str);

        assert!(parsed.is_ok());
        assert_eq!(&parsed.unwrap().1.unwrap().to_string(),"(2 + 3)(4/4)");
    }
}