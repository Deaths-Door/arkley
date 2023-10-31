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

pub fn parse_equation<'a>(context : &'a Context<'a>) -> impl FnMut(&'a str) -> IResult<&'a str,Expression> {
    move |input| {
        let (input,lexpr) = parse_expression(context)(input)?;

        let (input,relational_operator) = delimited(
            multispace0,
            parse_relation_operator,
            multispace0
        )(input)?;

        let (input,rexpr) = parse_expression(context)(input)?;

        let equation = Equation::new(lexpr,relational_operator,rexpr);

        Ok((input,equation))
    }
}

fn parse_relation_operator(input: &str) -> IResult<&str,RelationalOperator> {
    alt((
        value(RelationalOperator::Equal,tag("=")),
        value(RelationalOperator::GreaterThan,tag(">")),
        value(RelationalOperator::LessThan,tag("<")),
    ))(input)
}

// TODO : Add try_from for expression once the lifetime garbage can be fixed

impl<'a> TryFrom<(&'a str,&'a Context<'a>)> for Equation {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from((input,context): (&'a str,&'a Context<'a>)) -> Result<Self, Self::Error> {
        all_consuming(parse_equation(context))(input).map(move |(_,eq)| eq)
    }
}