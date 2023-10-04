use nom::{IResult, sequence::{delimited, tuple, preceded},character::complete::{char, multispace0, one_of}, branch::alt, combinator::{value, map}, multi::fold_many0};

use crate::{Expression, ArithmeticOperation, parse_term};

/// Parses a mathematical expression from the input string.
///
/// This function takes an input string and parses it into a mathematical expression. It handles
/// expressions with various levels of complexity, including terms, binary operations, and nested
/// expressions.
///
/// # Arguments
///
/// * `input`: A string containing the mathematical expression to be parsed.
pub fn parse_expression(_input: &str) -> IResult<&str, Expression> {
    let _parser = tuple((
        parse_term,

    ));

    todo!()
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