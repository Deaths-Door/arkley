use nom::{character::complete::{alphanumeric0, char, satisfy}, combinator::{all_consuming, opt}, sequence::{delimited, pair}, IResult};

use crate::Variable;

 
impl<'a> TryFrom<&'a str> for Variable {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        all_consuming(parse_variable)(input).map(|(_,value)| value)
    }
}

/// Parses a variable name from a string slice.
/// This function attempts to parse a variable name from the provided string slice (`input`). 
/// The expected format for a variable name is a single letter followed by an optional label 
/// enclosed in curly braces (`{}`).
pub fn parse_variable(input : &str) -> IResult<&str,Variable> {
    // https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md
    let (input,(letter,label)) = pair(
        satisfies_variable_name,
        opt(
            delimited(char('{'), alphanumeric0, char('}'))
        )
    )(input)?;

    let variable = Variable::new(letter, label.map(str::to_string));

    Ok((input,variable))
}

fn satisfies_variable_name(input : &str) -> IResult<&str,char> {
    satisfy(|c| c >= 'a' && c <= 'z' && (c != 'e' || c!= 'E') )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("x" ; "single_letter")]
    #[test_case("y{label}" ; "with_label")]
    #[test_case("abc" ; "multiple_letters")]
    #[test_case("z{}" ; "empty_label")]
    fn test_parse_variable_valid(input: &str) {
        let result = parse_variable(input);
        assert!(result.is_ok());

        let expected_variable = Variable::new(input.chars().next().unwrap(), match input.contains("{") {
            true => Some(input[input.find("{").unwrap() + 1..input.find("}").unwrap()].to_string()),
            false => None,
        });
        assert_eq!(result.unwrap().1,expected_variable);
    }

    #[test_case("1x" ; "invalid_start_number")]
    #[test_case("_x" ; "invalid_start_underscore")]
    #[test_case("z{label" ; "missing_closing_brace")]
    #[test_case("x{label}suffix" ; "extra_characters")]
    #[test_case("w{{label}}" ; "multiple_opening_braces")]
    #[test_case("v{label}}" ; "multiple_closing_braces")]
    fn test_parse_variable_invalid(input: &str) {
        let result = parse_variable(input);
        assert!(result.is_err() || !result.unwrap().0.is_empty());
    }
}