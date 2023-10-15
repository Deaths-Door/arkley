use nom::{
    sequence::{tuple, delimited}, 
    character::complete::multispace0, 
    bytes::complete::tag, branch::alt, combinator::{value, map}, IResult};

use crate::{Equation, RelationalOperator, parse_expression};


/// Parse an equation from the input string.
///
/// This function attempts to parse an equation from the input string, where an equation
/// consists of two expressions separated by a relation operator. If both expressions
/// are successfully parsed, it constructs an `Equation` object and returns it wrapped
/// in `Some`. If either of the expressions fails to parse, it returns `None`.
///
/// # Arguments
///
/// * `input`: A string containing the equation to be parsed.
///
/// # Returns
///
/// Returns an `Option<Equation>`, where `Some` contains a valid `Equation` object if
/// both expressions are successfully parsed, and `None` if parsing fails.

// TODO : Give better reason then None for why its invalid
pub fn parse_equation(input: &str) -> IResult<&str,Option<Equation>> {
    let parser = tuple((
        parse_expression,
        delimited(multispace0,parse_relation_operator,multispace0),
        parse_expression
    ));

    map(parser,|(eq1,relation,eq2)| match eq1.is_none() || eq2.is_none() {
        true => None,
        false => Some(Equation::new(eq1.unwrap(),relation,eq2.unwrap()))
    })(input)
}

fn parse_relation_operator(input: &str) -> IResult<&str,RelationalOperator> {
    alt((
        value(RelationalOperator::Equal,tag("=")),
        value(RelationalOperator::GreaterThan,tag(">")),
        value(RelationalOperator::LessThan,tag("<")),
    ))(input)
}