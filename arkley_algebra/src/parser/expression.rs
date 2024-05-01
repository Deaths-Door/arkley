use nom::{IResult, combinator::all_consuming};

use crate::{Expression, Context};

use super::ExpressionToken;

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
    move |input| {
        let (input,vec) = ExpressionToken::parse(context)(input)?;
        let expression = ExpressionToken::into_expression_tree(ExpressionToken::to_rpn(vec));
    
        Ok((input,expression))
    }
}

impl<'a,'b> TryFrom<(&'a str,&'b Context<'b>)> for Expression {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from((input,context): (&'a str,&'b Context<'b>)) -> Result<Self, Self::Error> {
        let (_,tokens) = all_consuming(ExpressionToken::parse(context))(input)?;
        let expression = ExpressionToken::into_expression_tree(ExpressionToken::to_rpn(tokens));
    
        Ok(expression)
    }
}

impl<'a> TryFrom<&'a str> for Expression {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let context = Context::default();
        Self::try_from((input,&context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Term;
    use test_case::test_case;
    use num_notation::Number;

    #[test_case("3 + 4",Expression::new_plus(3,4))]
    #[test_case("1 + (2 * 3)",Expression::new_plus(1,Expression::new_mal(2,3)))]
    #[test_case("1 + 2(4)",Expression::new_plus(1,Expression::new_mal(2,4)))]
    #[test_case("-5 + 2",Expression::new_plus(-5,2))]
    #[test_case("2 + 3 * 4 - 5 / 1",Expression::new_minus(
        Expression::new_plus(2, Expression::new_mal(3, 4)),
        Expression::new_durch(5, 1)
    ))]
    #[test_case("(2 + 3)(4/4)",Expression::new_mal(
        Expression::new_plus(2,3),
        Expression::new_durch(4,4)
    ))]
    #[test_case("(5-6)(2+3)",Expression::new_mal(
        Expression::new_minus(5,-6),
        Expression::new_plus(2,3),
    ))]
    #[test_case("2x^2 + 4y/8u^2", Expression::new_plus(
        Expression::new_term(Term::new_with_variable_to(2f64,'x',2f64)),
        Expression::new_durch(
            Expression::new_term(Term::new_with_variable(4f64,'y')),
            Expression::new_term(Term::new_with_variable_to(8f64,'u',2f64))
        )
    ))]
    #[test_case("3a - 2b^3", Expression::new_minus(
        Expression::new_term((Number::from(3f64),'a')),
        Expression::new_term(Term::new_with_variable_to(2f64,'b',3f64))
    ))]
    #[test_case("-(x + y)", Expression::new_minus(
        0,
        Expression::new_plus(Expression::new_term('x'), Expression::new_term('y'))
    ))]
    #[test_case("5(2x - 3y) + z", Expression::new_plus(
        Expression::new_mal(
            5, Expression::new_minus(
                Expression::new_term((Number::from(2f64),'x')),
                Expression::new_term((Number::from(3f64),'y'))
            )
        ),
        Expression::new_term('z')
    ))]
    #[test_case("(a^2 + b)(c - d)", Expression::new_mal(
        Expression::new_plus(
            Expression::new_pow(Expression::new_term('a'), 2),
            Expression::new_term('b')
        ),
        Expression::new_minus(Expression::new_term('c'), Expression::new_term('d'))
    ))]
    #[test_case("x / (y + z)", Expression::new_durch(
        Expression::new_term('x'),
        Expression::new_plus(Expression::new_term('y'), Expression::new_term('z'))
    ))]
    #[test_case("1-5/8",Expression::new_minus(1,Expression::new_durch(5,8)))]
    fn parse_basic_and_complex_expressions(input : &str,expected : Expression) {
        assert_eq!(Expression::try_from(input).map(|v| v.to_string()),Ok(expected.to_string()))
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

    // TODO : ADD MORE WITH CONTEXTS 
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