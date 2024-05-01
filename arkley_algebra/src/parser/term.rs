use nom::{
    IResult, 
    sequence::{preceded, delimited, pair}, 
    multi::many1, 
    combinator::{opt, all_consuming},
    character::complete::char, branch::alt,
};

use num_notation::{Number, parse_number};

use super::parse_add_sub;

use crate::{parse_variable, ArithmeticOperation, Term, Variables};

impl<'a> TryFrom<&'a str> for Term {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        all_consuming(parse_term)(input).map(|(_,value)| value)
    }
}

/// Parse a mathematical term from a given input string.
///
/// A mathematical term can represent a part of a mathematical expression and consists of
/// a coefficient and optional variables with exponents.
pub fn parse_term(input : &str) -> IResult<&str,Term> {
    alt((
        parse_coefficient_with_opt_variables,
        parse_variables_with_opt_sign
    ))(input)
}

fn parse_coefficient_with_opt_variables(input : &str) -> IResult<&str,Term> {
    let (input,(coefficient,variables)) = pair(
        parse_number,
        opt(parse_variables)
    )(input)?;
    
    let term = Term::new_with_variables(coefficient, variables.unwrap_or_default());

    Ok((input,term))
}

fn parse_variables_with_opt_sign(input : &str) -> IResult<&str,Term> {
    let (input,(sign,variables)) = pair(
        opt(parse_add_sub),
        parse_variables,
    )(input)?;

    let term = match sign {
        Some(sign) if sign == ArithmeticOperation::Minus => Term::new_with_variables(-1f64, variables),
        _ => variables.into(),
    };

    Ok((input,term))
}

fn parse_variables(input : &str) -> IResult<&str,Variables> {
    let (input,vec) =  many1(
        pair(
            parse_variable,
            opt(parse_exponent)
        )
    )(input)?;

    let variables = vec.into_iter()
        .map(|(c,num)| (c,num.unwrap_or_else(|| 1.0.into())))
        .collect();

    Ok((input,variables))
}

fn parse_exponent(input : &str) -> IResult<&str,Number> {
    preceded(char('^'),
        delimited(
            opt(char('(')), 
            parse_number, 
            opt(char(')'))
        )
    )(input)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::Variable;

    use super::*;

    #[test_case("x",1,&[('x',1)])]
    #[test_case("-x^(-5)",-1,&[('x',-5)])]
    #[test_case("x^2",1,&[('x',2)])]
    #[test_case("x^(3)",1,&[('x',3)])]
    #[test_case("x^2y^3z",1,&[('x',2),('y',3),('z',1)])]
    #[test_case("-2x", -2, &[('x', 1)])]
    #[test_case("-3x^3y", -3, &[('x', 3), ('y', 1)])]
    fn check_parsing_with(input : &str,coeffiecent : i32,variables : &[(char,i32)]) {
        let expected_variables = Variables::from_iter(
            variables.into_iter().map(|(l,e)| (Variable::from(*l),Number::from((*e) as f64)))
        );

        assert_eq!(
            parse_term(input),
            Ok(("", Term::new_with_variables(coeffiecent as f64, expected_variables)))
        );
    }

    #[test]
    fn test_parse_term_with_only_number() {
        let input = "3.14";
        let result = parse_term(input);
        assert_eq!(
            result,
            Ok((
                "",
                Term::new(Number::Decimal(3.14))
            ))
        );
    }

    #[test]
    fn test_parse_term_empty_input() {
        let input = "";
        let result = parse_term(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variables_invalid_input() {
        let input = "x^2y3z"; // Invalid input missing '^' between 'y' and '3'
        let expected_variables = Variables::from([('x'.into(),2.0.into()),('y'.into(),1.0.into())]);
        assert_eq!(parse_variables(input), Ok(("3z", expected_variables)));
    }
}