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
        all_consuming(parse_expression(context))(input)
            .map(|(_,expression)| expression)
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
    use nom_supreme::final_parser::final_parser;
    use test_case::test_case;

    #[test_case("3 + 4","3 + 4")]
    #[test_case("1 + (2 * 3)","1 + 2(3)")]
    #[test_case("1 + 2(4)","1 + 2(4)")]
    #[test_case("-5 + 2","-5 + 2")]
    #[test_case("2 + 3 * 4 - 5 / 1","2+3(4)-5/1")]
    #[test_case("(2 + 3)(4/3)","(2 + 3)(4/3)")]
    // We are comparing strings as the divide sign is interpreted as a fraction,
    // which is not wrong its just not what i was expecting
    //  #[test_case("(2 + 3)(4/4)","(2 + 3)(4/4)")]
    #[test_case("(2 + 3)(4/5)","(2 + 3)(4/5)")]
    #[test_case("(5-6)(2+3)","(5-6)(2+3)")]
    #[test_case("2x^2 + 4y/8u^2","2x^2 + 4y/8u^2")]
    #[test_case("3a - 2b^3","3a - 2b^3")]
    #[test_case("-(x + y)","-(x + y)")]
    #[test_case("5(2x - 3y) + z","5(2x - 3y) + z")]
    #[test_case("(a^2 + b)(c - d)","(a^2 + b)(c - d)")]
    #[test_case("x / (y + z)","x/(y + z)")]
    #[test_case("1-5/8","1-5/8")]
    #[test_case("a+y*z","a+y(z)")]
    fn parse_basic_and_complex_expressions(input : &str,expected : &str) {
        let context = Default::default();
        let parsed : Result<Expression,nom::error::Error<&str>> = final_parser(parse_expression(&context))(input);

        // And we are replacing whitespaces , so that the format of the input 
        // and my [std::fmt::Display] can be correctly compared
        assert_eq!(
            parsed.map(|v| v.to_string().replace(" ", "")),
            Ok(expected.to_string().replace(" ", ""))
        )
    }

    #[test]
    fn parse_invalid_expression() {
        let input_str = "5 + (2 * 3"; 
        let context = Default::default();   

        let parsed  = parse_expression(&context)(input_str);
       
        // one would thing it should be none but parser stops checking at 5 + so output is 5 , for full consumuing use try_from
        let unwrapped = parsed.unwrap().1;
        assert_eq!(&unwrapped.to_string(),"5")
    }
    #[test_case("a + lightspeed", "3 + 299792458", &[("lightspeed", "299792458"),("a", "3")])]
    #[test_case("b * conversion_rate", "543x(1.23)", &[("conversion_rate", "1.23"),("b", "543x")])]
    #[test_case("(gravity)", "9.81", &[("gravity", "9.81")])]
    #[test_case("area * price", "(x(y))(10)", &[("price", "10"),("area", "length * width"),("length","x"),("width","y")])]
    #[test_case("tax_rate * income", "0.25(1000 + 500)", &[("tax_rate", "0.25"),("income", "salary + bonus"),("salary","1000"),("bonus","500")])]
    // TODO: Why is this passing on some ocassions , and failing on others with stackoverflow
    #[test_case("discount(total)", "5y(x+y(z))", &[("discount", "discounted_price"),("total", "x + y * z"),("discounted_price","5y")])]
    #[test_case("(target - progress) / efficiency", "(100000 - 0.75) / 0.05 ", &[("efficiency", "rate"),("target", "goal"), ("progress", "0.75"),("rate","0.05"),("goal","100000")])]
    #[test_case("current_year + age", "2024 + y", &[("current_year", "2024"), ("age", "y")])]
    #[test_case("name_length(full_name)", "f(659711497118 + 8310497104)", &[("name_length", "get_length"),("full_name", "first_name + last_name"),("first_name","659711497118"),("last_name","8310497104"),("get_length","f")])]
    fn with_context(input : &str,output : &str,tags : &[(&str,&str)]) {
        let mut context = Context::default();
        context.extend_tags_str(tags.to_owned().into_iter());

        println!("{:?}",ExpressionToken::to_rpn(ExpressionToken::parse(&context)(input).unwrap().1));

        assert_eq!(
            Expression::try_from((input,&context)).map(|s| s.to_string().replace(" ","")),
            Ok(output.replace(" ",""))
        )
    }
}