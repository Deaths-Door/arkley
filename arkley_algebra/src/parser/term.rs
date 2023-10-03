use nom::{
    IResult, 
    sequence::{tuple, preceded, delimited}, 
    multi::{many1, many0}, 
    combinator::{map, opt},
    branch::alt, 
    character::complete::{alpha1,char, satisfy}
};
use num_notation::{Number, parse_number, Num};

use crate::{Term, VariableOperations, Variables};



/// Parse a mathematical term from a given input string.
///
/// A mathematical term can represent a part of a mathematical expression and consists of
/// a coefficient and optional variables with exponents.
/// 
/// **Note**: If the input string is empty, this function returns `Ok(("",1.0.into()))`.
pub fn parse_term(input : &str) -> IResult<&str,Term> {
    //num + opt var
    // +/- var , to many hadnle cases like -x where num is not there

    // Parse a term with a number and optional variables
    let parse_num_and_opt_vars = tuple((parse_number, opt(parse_variables)));
    let map_num_and_opt_vars = map(parse_num_and_opt_vars, |(coefficient, variables)| Term::new_with_variable(coefficient, variables.unwrap_or_default()) );
    
    // Parse a term with an optional sign (+/-) and variables
    let parse_opt_sign_and_vars = tuple((opt(alt((char('+'), char('-')))), parse_variables));
    let map_opt_sign_and_vars = map(parse_opt_sign_and_vars, |(sign, variable)| {
         let coefficient = match sign {
             Some('+') | None   => Number::Decimal(1.0), // Default to 1.0 for '+' sign
             Some('-') => Number::Decimal(-1.0), // Default to -1.0 for '-' sign
            _ => unreachable!()
         };
         Term::new_with_variable(coefficient, variable)
     });
    
    alt((map_num_and_opt_vars,   map_opt_sign_and_vars))(input)
}

fn parse_variables(input : &str) -> IResult<&str,Variables> {
    let parse_var_name = satisfy(|c| c >= 'a' && c <= 'z' && (c != 'e' || c!= 'E') );
    let parser =  many0(tuple((parse_var_name,opt(parse_variable_exponent))));
    map(parser,|vec| 
        vec.into_iter()
            .map(|(c,num)| (c,num.unwrap_or(1.0.into())))
            .collect()
    )(input)
}

fn parse_variable_exponent(input : &str) -> IResult<&str,Number> {
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
        assert_eq!(result,Ok(("",1.0.into())));
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
    fn test_parse_variables_no_input() {
        let input = "";
        let result = parse_variables(input);
        let expected_variables = Variables::new();
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