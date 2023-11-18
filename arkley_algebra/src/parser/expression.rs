use nom::{IResult, combinator::all_consuming};

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
pub fn parse_expression<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Expression> + 'b {
    // TODO : This parses 2x = 5 into 5 as an equation and succeds for some reason
    move |input| {
        let (input,vec) = Token::parse(context)(input)?;
        let expression = Token::into_expression_tree(Token::to_rpn(vec));
    
        Ok((input,expression))
    }
}

impl<'a,'b> TryFrom<(&'a str,&'b Context<'b>)> for Expression {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from((input,context): (&'a str,&'b Context<'b>)) -> Result<Self, Self::Error> {
        let (_,vec) = all_consuming(Token::parse(context))(input)?;
        let expression = Token::into_expression_tree(Token::to_rpn(vec));
    
        Ok(expression)
    }
}

impl<'a> TryFrom<&'a str> for Expression {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let context = Context::default();

        let (_,vec) = all_consuming(Token::parse(&context))(input)?;
        let expression = Token::into_expression_tree(Token::to_rpn(vec));
    
        Ok(expression)
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
        let expected_expression = Expression::new_plus( 3.0,  4.0);

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
            1.0, 
            Expression::new_mal(2.0, 4.0)
        );
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().1,expected_expression);
    }

    #[test]
    fn parse_expression_with_unary_minus() {
        let input_str = "-5 + 2";
        let context = Default::default();   

        let parsed = parse_expression(&context)(input_str);
        let expected_expression =  Expression::new_plus(-5.0,  2.0);

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
            Expression::new_plus(2.0, Expression::new_mal(3.0, 4.0)),
            Expression::new_durch(5.0, 1.0)
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

    #[test]
    fn with_context() {
        let mut context = Context::default();
        context.tags_mut().insert("five", 5.into());
        context.tags_mut().insert("two", 2.into());
        context.tags_mut().insert("sieben", 7.into());

        let result = parse_expression(&context)("five * two + sieben");

        assert!(result.is_ok());

        assert_eq!(&result.unwrap().1.to_string(),"5 * 2 + 7")
    }
}