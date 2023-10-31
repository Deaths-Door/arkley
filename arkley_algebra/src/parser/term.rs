use nom::{
    IResult, 
    sequence::{preceded, delimited, pair}, 
    multi::many1, 
    combinator::{map, opt, all_consuming},
    character::complete::{char, satisfy}, branch::alt,
};

use num_notation::{Number, parse_number};

use super::parse_add_sub;

use crate::{Term, Variables, ArithmeticOperation};

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
    
    let term = Term::new_with_variable(coefficient, variables.unwrap_or_default());

    Ok((input,term))
}

fn parse_variables_with_opt_sign(input : &str) -> IResult<&str,Term> {
    let (input,(sign,variables)) = pair(
        opt(parse_add_sub),
        parse_variables,
    )(input)?;

    let term = match sign {
        Some(sign) if sign == ArithmeticOperation::Minus => Term::new_with_variable((-1f64).into(), variables),
        _ => variables.into(),
    };

    Ok((input,term))
}

// Used by super::function
pub(super) fn satisfies_variable_name(input : &str) -> IResult<&str,char> {
    satisfy(|c| c >= 'a' && c <= 'z' && (c != 'e' || c!= 'E') )(input)
}

fn parse_variables(input : &str) -> IResult<&str,Variables> {
    let (input,vec) =  many1(
        pair(
            satisfies_variable_name,
            opt(parse_exponent)
        )
    )(input)?;

    let variables = vec.into_iter()
        .map(|(c,num)| (c,num.unwrap_or(1.0.into())))
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
    use super::*;

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
    fn test_parse_term_with_only_variable() {
        let input = "x";
        let result = parse_term(input);
        let mut expected_variables = Variables::new();
        expected_variables.insert('x', Number::Decimal(1.0));
        assert_eq!(
            result,
            Ok(("", Term::new_with_variable(Number::Decimal(1.0), expected_variables)))
        );
    }

    #[test]
    fn test_parse_term_with_only_variable_neg() {
        let input = "-x";
        let result = parse_term(input);
        let mut expected_variables = Variables::new();
        expected_variables.insert('x', Number::Decimal(1.0));
        assert_eq!(
            result,
            Ok(("", Term::new_with_variable(Number::Decimal(-1.0), expected_variables)))
        );
    }

    #[test]
    fn test_parse_term_with_variable_and_exponent() {
        let input = "x^2";
        let result = parse_term(input);
        let mut expected_variables = Variables::new();
        expected_variables.insert('x', Number::Decimal(2.0));
        assert_eq!(
            result,
            Ok(("", Term::new_with_variable(Number::Decimal(1.0), expected_variables)))
        );
    }

    #[test]
    fn test_parse_term_with_variable_and_exponent_in_parentheses() {
        let input = "x^(3)";
        let result = parse_term(input);
        let mut expected_variables = Variables::new();
        expected_variables.insert('x', Number::Decimal(3.0));
        assert_eq!(
            result,
            Ok(("", Term::new_with_variable(Number::Decimal(1.0), expected_variables)))
        );
    }

    #[test]
    fn test_parse_term_empty_input() {
        let input = "";
        let result = parse_term(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_variables_single_variable() {
        let input = "x^2";
        let result = parse_variables(input);
        let mut expected_variables = Variables::new();
        expected_variables.insert('x', Number::Decimal(2.0));
        assert_eq!(result, Ok(("", expected_variables)));
    }

    #[test]
    fn test_parse_variables_multiple_variables() {
        let input = "x^2y^3z";
        let result = parse_variables(input);
        let mut expected_variables = Variables::new();
        expected_variables.insert('x', Number::Decimal(2.0));
        expected_variables.insert('y', Number::Decimal(3.0));
        expected_variables.insert('z', Number::Decimal(1.0));
        assert_eq!(result, Ok(("", expected_variables)));
    }

    #[test]
    fn test_parse_variables_invalid_input() {
        let input = "x^2y3z"; // Invalid input missing '^' between 'y' and '3'
        let result: Result<(&str, std::collections::BTreeMap<char, Number>), nom::Err<nom::error::Error<&str>>> = parse_variables(input);
        let expected_variables = Variables::from([('x',2.0.into()),('y',1.0.into())]);

        assert_eq!(result, Ok(("3z", expected_variables)));
    }
}