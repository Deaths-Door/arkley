use nom::{
    IResult, 
    character::complete::{char, multispace0}, 
    sequence::{tuple, preceded}, 
    multi::many1_count, 
    combinator::map, 
    branch::alt
};

use crate::ArithmeticOperation;


/// Parses an operator from the input string.
///
/// This function is used to parse operators from an input string. It can handle various
/// operators such as '*', '/', as well as special cases where there are sequences of
/// '+' and '-' characters with optional whitespace between them.
///
/// # Arguments
///
/// * `input`: A reference to the input string to be parsed.
///
/// # Returns
///
/// - If a regular operator ('*' or '/' or '+' or '-' ) is found, it returns the corresponding character.
/// - If a special sequence '+-' or '-+' is found (with or without whitespace), it returns '-'.
/// - If a special sequence '--' is found (with or without whitespace), it returns '+'.
pub fn parse_operator(input : &str) -> IResult<&str,ArithmeticOperation> {
    alt((
        parse_final_add_sub,
        map(char('*'),|_| ArithmeticOperation::Mal),
        map(char('/'),|_| ArithmeticOperation::Durch),
    ))(input)
}

// TODO : Maybe can reduce the number of functions and then just use calculate_final_sign
// For now using parse_final_add_sub for future replacement
pub(super) fn parse_final_add_sub(input : &str) -> IResult<&str,ArithmeticOperation> {
    alt((
        parse_add_sub,
        parse_sub_add,
        parse_add,
        parse_sub,
    ))(input)
}


fn many_add(input : &str) -> IResult<&str,usize> {
    many1_count(char('+'))(input)
}

fn many_sub(input : &str) -> IResult<&str,usize> {
    many1_count(char('-'))(input)
}

fn parse_add(input : &str) -> IResult<&str,ArithmeticOperation> {
    map(preceded(multispace0,many_add),|_| ArithmeticOperation::Plus)(input)
}

fn parse_sub(input : &str) -> IResult<&str,ArithmeticOperation> {
    map(preceded(multispace0,many_sub),|count| calculate_final_sign(0, count))(input)
}

fn parse_add_sub(input : &str) -> IResult<&str,ArithmeticOperation> {
    let parse = tuple((many_add, multispace0, many_sub));
    map(parse,|(c1,_,c2)| calculate_final_sign(c1, c2) )(input)
}

fn parse_sub_add(input : &str) -> IResult<&str,ArithmeticOperation> {
    let parse = tuple((many_sub, multispace0, many_add));
    map(parse,|(c2,_,c1)| calculate_final_sign(c1, c2) )(input)
}

/// Calculates the final sign character based on the counts of plus and minus signs.
///
/// This function takes the counts of plus and minus signs and returns a character that
/// represents the final sign based on the comparison of these counts.
///
/// - If the count of plus signs is less than the count of minus signs, it returns '-'. 
///   Additionally, if the count of minus signs is even, it returns '+' instead of '-'.
/// - If the count of plus signs is greater than or equal to the count of minus signs, it returns '+'.
///
/// # Arguments
///
/// * `plus`: The count of plus signs.
/// * `minus`: The count of minus signs.
///
/// # Returns
///
/// A character representing the final sign ('+' or '-') based on the counts of plus and minus signs.
///
/// # Example
///
/// ```
/// let plus_count = 4;
/// let minus_count = 5;
/// let result = calculate_final_sign(plus_count, minus_count);
/// assert_eq!(result, '-');
/// ```
fn calculate_final_sign(_plus:usize,_minus:usize) -> ArithmeticOperation {
    if _plus == 0 && _minus == 0 {
        return ArithmeticOperation::Plus;
    }

    let plus = (_plus as isize) - 1isize;
    let minus = _minus as isize;

    match plus < minus {
        true => if minus %2 == 0 { ArithmeticOperation::Plus } else { ArithmeticOperation::Minus }
        false => ArithmeticOperation::Plus,
    }
}

#[cfg(test)]
mod calculate_final_sign_tests {
    use super::*;

    #[test]
    fn test_final_sign_plus_less_than_minus() {
       
        assert_eq!(calculate_final_sign(4, 5),  ArithmeticOperation::try_from('-').unwrap());
        assert_eq!(calculate_final_sign(2, 7),  ArithmeticOperation::try_from('-').unwrap());
    }

    #[test]
    fn test_final_sign_plus_greater_than_minus() {
        assert_eq!(calculate_final_sign(5, 4),  ArithmeticOperation::try_from('+').unwrap());
        assert_eq!(calculate_final_sign(7, 2), ArithmeticOperation::try_from('+').unwrap());
    }

    #[test]
    fn test_final_sign_plus_equal_to_minus() {
        assert_eq!(calculate_final_sign(3, 3),  ArithmeticOperation::try_from('-').unwrap());
        assert_eq!(calculate_final_sign(0, 0), ArithmeticOperation::try_from('+').unwrap());
    }
}


#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_regular_operator() {
        assert_eq!(parse_operator("*"), Ok(("", ArithmeticOperation::try_from('*').unwrap())));
        assert_eq!(parse_operator("/"), Ok(("", ArithmeticOperation::try_from('/').unwrap())));
        assert_eq!(parse_operator("+"), Ok(("", ArithmeticOperation::try_from('+').unwrap())));
        assert_eq!(parse_operator("-"), Ok(("",  ArithmeticOperation::try_from('-').unwrap())));
    }

    #[test]
    fn test_special_sequence() {
        assert_eq!(parse_operator("++"), Ok(("", ArithmeticOperation::try_from('+').unwrap())));
        assert_eq!(parse_operator("--"), Ok(("", ArithmeticOperation::try_from('+').unwrap())));
        assert_eq!(parse_operator("+ -"), Ok(("",  ArithmeticOperation::try_from('-').unwrap())));
        assert_eq!(parse_operator("- + "), Ok((" ", ArithmeticOperation::try_from('-').unwrap())));
        assert_eq!(parse_operator("+-"), Ok(("",  ArithmeticOperation::try_from('-').unwrap())));
        assert_eq!(parse_operator("-+"), Ok(("",  ArithmeticOperation::try_from('-').unwrap())));
    }

    #[test]
    fn test_invalid_operator() {
        // Add more test cases for invalid input
        assert!(parse_operator("abc").is_err());
        assert!(parse_operator("").is_err());
    }
}
