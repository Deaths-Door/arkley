use nom::{
    IResult, 
    character::complete::{char, multispace0}, 
    sequence::tuple, 
    multi::many1_count, 
    combinator::map, 
    branch::alt
};


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
pub fn parse_operator(input : &str) -> IResult<&str,char> {
    let parse_plus = || many1_count(char('+'));
    let parse_minus = || many1_count(char('-'));
    
    let parse_plus_minus = tuple((parse_plus(), multispace0, parse_minus()));
    let parse_minus_plus = tuple((parse_minus(), multispace0, parse_plus()));

    
    let map_plus = map(parse_plus(),|_| '+' );
    let map_minus = map(parse_minus(),|count| calculate_final_sign(0, count));
    let map_plus_minus = map(parse_plus_minus,|(c1,_,c2)| {
        println!("Plus = {c1} , minus = {c2} ");
        calculate_final_sign(c2, c2)
    });
    let map_minus_plus = map(parse_minus_plus,|(c2,_,c1)| {
        println!("Plus = {c1} , minus = {c2} ");
        calculate_final_sign(c1, c2)
    });

    alt((
        map_plus_minus,
        map_minus_plus,
        map_plus,
        map_minus,
        char('*'),
        char('/')
    ))(input)
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
fn calculate_final_sign(_plus:usize,_minus:usize) -> char {
    if _plus == 0 && _minus == 0 {
        return '+';
    }

    let plus = (_plus as isize) - 1isize;
    let minus = _minus as isize;

    match plus < minus {
        true => if minus %2 == 0 { '+' } else { '-' }
        false => '+',
    }
}

#[cfg(test)]
mod calculate_final_sign_tests {
    use super::*;

    #[test]
    fn test_final_sign_plus_less_than_minus() {
        assert_eq!(calculate_final_sign(4, 5), '-');
        assert_eq!(calculate_final_sign(2, 7), '-');
    }

    #[test]
    fn test_final_sign_plus_greater_than_minus() {
        assert_eq!(calculate_final_sign(5, 4), '+');
        assert_eq!(calculate_final_sign(7, 2), '+');
    }

    #[test]
    fn test_final_sign_plus_equal_to_minus() {
        assert_eq!(calculate_final_sign(3, 3), '-');
        assert_eq!(calculate_final_sign(0, 0), '+');
    }
}


#[cfg(test)]
mod pare_tests {
    use super::*;

    #[test]
    fn test_regular_operator() {
        assert_eq!(parse_operator("*"), Ok(("", '*')));
        assert_eq!(parse_operator("/"), Ok(("", '/')));
        assert_eq!(parse_operator("+"), Ok(("", '+')));
        assert_eq!(parse_operator("-"), Ok(("", '-')));
    }

    #[test]
    fn test_special_sequence() {
        assert_eq!(parse_operator("++"), Ok(("", '+')));
        assert_eq!(parse_operator("--"), Ok(("", '+')));
        assert_eq!(parse_operator("+ -"), Ok(("", '-')));
        assert_eq!(parse_operator("- + "), Ok((" ", '-')));
        assert_eq!(parse_operator("+-"), Ok(("", '-')));
        assert_eq!(parse_operator("-+"), Ok(("", '-')));
    }

    #[test]
    fn test_invalid_operator() {
        // Add more test cases for invalid input
        assert!(parse_operator("abc").is_err());
        assert!(parse_operator("").is_err());
    }
}
