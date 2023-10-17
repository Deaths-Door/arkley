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
}

/*
/*
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    todo!()/*delimited(
        multispace0,
        alt((
            parse_nested_expression,
            parse_binary_expression,
            map(parse_term,|value| value.into()), 
        )),
        multispace0
    )(input)*/
}*/

/*
fn parse_expression_binary(input: &str) -> IResult<&str, Expression> {
    delimited(
        multispace0,
        alt((
            parse_nested_expression,
           // parse_binary_expression,
            map(parse_term,|value| value.into()), 
        )),
        multispace0
    )(input)
}

fn parse_nested_expression(input: &str) -> IResult<&str, Expression> {
    let parser = delimited(char('('), parse_expression, char(')'));
    map(parser,|expr| Expression::new_nested(expr))(input)
}

fn parse_arithmetic_operation(input: &str) -> IResult<&str, ArithmeticOperation> {
    alt((
        value(ArithmeticOperation::Plus, char('+')),
        value(ArithmeticOperation::Minus, char('-')),
        value(ArithmeticOperation::Mal, char('*')),
        value(ArithmeticOperation::Durch, char('/')),
    ))(input)
}

fn parse_binary_expression(input: &str) -> IResult<&str, Expression> {
    let parse_mul_expr = tuple((parse_expression_binary,preceded(char('('),parse_expression_binary)));
    let map_mul_expr = map(parse_mul_expr,|(lexpr,rexpr)| Expression::new_mal(lexpr, rexpr));

    let parse_other_expr = tuple((parse_expression_binary,parse_arithmetic_operation,parse_expression_binary));
    let map_other_expr = map(parse_other_expr,|(lexpr,op,rexpr)| Expression::new_binary(op, lexpr, rexpr));

    alt((
        map_other_expr,
        map_mul_expr
    ))(input)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! create_parsing_tests {
        (ok => $(( $from : expr,$expected : expr) ),*) => {
            $(
                assert_eq!(parse_expression($from).map(|(_, expr)| expr),Ok($expected));
            )*
        };
    }

    #[test]
    fn order_of_operations() {
        create_parsing_tests!(ok => 
            ("2 + 3 * 4 ",Expression::new_plus(2.0.into(),Expression::new_mal(3.0.into(), 4.0.into()))),
            ("(2 + 3) * 4",Expression::new_mal(
                    Expression::new_nested(
                        Expression::new_plus(2.0.into(), 3.0.into())
                    ),
                    4.0.into()
                )
            )
        );
    }
}*/