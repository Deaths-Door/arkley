use nom::{IResult, combinator::{map, all_consuming}, Parser, InputLength, error::{ParseError, ErrorKind}};

use crate::{Expression, Context};

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
pub fn parse_expression<'a>(context : &'a Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,Expression> {
    move |input| {
        let (input,vec) = Token::into_tokens(input, context)?;
        let expression = Token::into_expression_tree(Token::to_rpn(vec));
    
        Ok((input,expression))
    }
}

// TODO : Add try_from for expression once the lifetime garbage can be fixed

impl<'a> TryFrom<(&'a str,&'a Context<'a>)> for Expression {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from((input,context): (&'a str,&'a Context<'a>)) -> Result<Self, Self::Error> {
        all_consuming(parse_expression(context))(input).map(move |(_,expr)| expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_addition() {
        let input_str = "3 + 4";
        let context = Default::default();   
        let parsed = parse_expression(&context)(input_str);
        let expected_expression = Expression::new_plus( 3.0.into(),  4.0.into());

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,expected_expression);
    }

    #[test]
    fn parse_complex_expression() {
        let input_str = "1 + (2 * 3)";
        let context = Default::default();   
        let parsed = parse_expression(&context)(input_str);

        assert!(parsed.is_ok());
        assert_eq!(&parsed.unwrap().1.to_string(),"1 + 2(3)"); // unnesscary brackets removed
    }

    #[test]
    fn parse_with_implicit_mul() {
        let input_str = "1 + 2(4)";
        let context = Default::default();   
        let parsed = parse_expression(&context)(input_str);

        let expected_expression = Expression::new_plus(
            1.0.into(), 
            Expression::new_mal(2.0.into(), 4.0.into())
        );
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,expected_expression);
    }

    #[test]
    fn parse_expression_with_unary_minus() {
        let input_str = "-5 + 2";
        let context = Default::default();   

        let parsed = parse_expression(&context)(input_str);
        let expected_expression =  Expression::new_plus((-5.0).into(),  2.0.into());

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,expected_expression);
    }

    #[test]
    fn parse_invalid_expression() {
        let input_str = "5 + (2 * 3"; 
        let context = Default::default();   

        let parsed = parse_expression(&context)(input_str);
       
        // one would thing it should be none but parser stops checking at 5 + so output is 5 , for full consumuing use try_from
        let unwrapped = parsed.unwrap().1;
        assert_eq!(&unwrapped.to_string(),"5")
    }

    #[test]
    fn parse_expression_with_multiple_operators() {
        let input_str = "2 + 3 * 4 - 5 / 1";
        let context = Default::default();   

        let parsed = parse_expression(&context)(input_str);
        let expected_expression = Expression::new_minus(
            Expression::new_plus(2.0.into(), Expression::new_mal(3.0.into(), 4.0.into())),
            Expression::new_durch(5.0.into(), 1.0.into())
        );

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,expected_expression);
    }

    #[test]
    fn double_brackets() {
        let input_str = "(2 + 3)(4/4)";
        let context = Default::default();   

        let parsed = parse_expression(&context)(input_str);

        assert!(parsed.is_ok());
        assert_eq!(&parsed.unwrap().1.to_string(),"(2 + 3)(4/4)");
    }
}