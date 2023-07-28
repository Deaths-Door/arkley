use std::str::FromStr;

use std::num::ParseFloatError;

use nom::{
    combinator::map,
    number::complete::float,
    character::complete::multispace0,
    IResult,
    Parser
};


pub fn parse_f64(input : &str) -> IResult<(&str, f32),ParseFloatError> {
    map(float,FromStr::from_str)(input)
}