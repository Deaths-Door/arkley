use nom::{branch::alt, character::complete::{char, multispace0}, combinator::{map, opt}, multi::fold_many0, sequence::{delimited, pair, separated_pair, tuple}, IResult, Parser};

use crate::{parse_operator, parse_term, parser::parse_add_sub, ArithmeticOperation, Context, CustomizableExpression, Expression, Term};


/// This `enum` is used for internal parsing of mathematical expressions represented as strings. 
/// It serves as a temporary representation during the tokenization and parsing process 
/// before converting the expression into the final `Expression` enum that the library can use.

/// Each variant of the `ExpressionToken` enum represents a specific element encountered while 
/// parsing the expression string:
#[derive(Debug)]
#[cfg_attr(test,derive(PartialEq))]
pub(super) enum ExpressionToken {
    /// Used for context parsing like "five_x_plus_y * x" => "(5x + y) * x" => "5x^2" and Terms 
    Expression(Expression),
    Operator(ArithmeticOperation),
    Custom(Box<dyn CustomizableExpression>),
    OpenParenthesis,
    CloseParenthesis,
}

impl From<ArithmeticOperation> for ExpressionToken {
    fn from(value: ArithmeticOperation) -> Self {
        Self::Operator(value)
    }
}

impl ArithmeticOperation {
    const fn precedence(&self) -> u8 {
        match self {
            ArithmeticOperation::Plus | ArithmeticOperation::Minus => 1,
            ArithmeticOperation::Mal | ArithmeticOperation::Durch => 2,
            ArithmeticOperation::Pow |  ArithmeticOperation::Root => 3,
        }
    }
}

impl ExpressionToken {
    // TODO : Add custom parser argument into this
    pub(super) fn parse<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,mut tokens) = Self::parse_with_optional_implicit_mul(context)(input)?;

            let inner_parser = tuple((
                multispace0,
                parse_operator,
                multispace0,
                Self::parse_with_optional_implicit_mul(context)
            ));

            let (input,follinow_tokens) = fold_many0(inner_parser, Vec::new,|mut tokens , (_,operation,_,inner_tokens)|{
                tokens.push(operation.into());
                tokens.extend(inner_tokens.into_iter());
                tokens
            })(input)?;

            tokens.extend(follinow_tokens.into_iter());

            Ok((input,tokens))
        }
    }

    fn create_tokens_for_optional_implicit_mul(mut lexpr_tokens : Vec<ExpressionToken>,rexpr_tokens : Vec<ExpressionToken>) -> Vec<ExpressionToken> {            
        // Do not put brackets around a single token
        if lexpr_tokens.len() != 1 {
            lexpr_tokens.insert(0, ExpressionToken::OpenParenthesis);
            lexpr_tokens.push(ExpressionToken::CloseParenthesis);
        }

        if !rexpr_tokens.is_empty() {
            lexpr_tokens.push(ArithmeticOperation::Mal.into());

            // No requirement to add bracktets as [Self::parse_nested_expression] already adds it
            lexpr_tokens.extend(rexpr_tokens.into_iter());    
        }

        lexpr_tokens
    }

    fn parse_with_optional_implicit_mul<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let option1 = map(
                separated_pair(
                    alt((
                        context.parse_tags(),
                        Term::map_into_tokens(),
                        Self::parse_nested_expression(context),
                    )),
                    multispace0,
                    opt(Self::parse_nested_expression(context))
                ),
                |(lexpr_tokens,rexpr_tokens)| Self::create_tokens_for_optional_implicit_mul(lexpr_tokens,rexpr_tokens.unwrap_or_default())
            );

            let option2 = map(
                separated_pair(
                    ArithmeticOperation::map_add_and_subtract_into_tokens(),
                    multispace0,
                    Self::parse_nested_expression(context)
                ),
                |(lexpr_tokens,rexpr_tokens)| {
                    let mut tokens = Self::create_tokens_for_optional_implicit_mul(
                        Vec::with_capacity(lexpr_tokens.capacity() + 1), 
                        rexpr_tokens
                    );
                    tokens.insert(0, ExpressionToken::Expression((-1).into()));
                    tokens
                }
            );

            let (input,tokens) = alt(( option1,option2)) (input)?;
                        
            Ok((input,tokens))
        }
    }

    fn parse_nested_expression<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,mut tokens) = delimited(
                pair(char('('),multispace0),
                ExpressionToken::parse(context), 
                pair(multispace0,char(')')),
            )(input)?;

            tokens.insert(0,ExpressionToken::OpenParenthesis);
            tokens.push(ExpressionToken::CloseParenthesis);

            Ok((input,tokens))
        }
    }
}

impl ArithmeticOperation {
    fn map_add_and_subtract_into_tokens<'a>() -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> {
        map(parse_add_sub,|op| Vec::from([ExpressionToken::Operator(op)]) )
    }
}

impl Term {
    fn map_into_tokens<'a>() -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> {
        map(parse_term,|term| Vec::from([ExpressionToken::Expression(term.into())]) )
    }
}

impl ExpressionToken {
    /// Converts an infix expression represented by a vector of `ExpressionToken` into Reverse Polish Notation (RPN).
    ///
    /// This function takes a vector of tokens and uses the Shunting Yard algorithm to convert the
    /// infix expression into RPN. The resulting RPN expression is returned as a vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `vec` - A vector of `ExpressionToken` representing the infix expression to be converted to RPN.
    ///
    /// # Returns
    ///
    /// A vector of `ExpressionToken` representing the RPN expression.
    pub(super) fn to_rpn(vec : Vec<ExpressionToken>) -> Vec<ExpressionToken> {
        let mut output: Vec<ExpressionToken> = Vec::new();
        let mut operator_stack: Vec<ExpressionToken> = Vec::new();

        for token in vec {
            match token {
                ExpressionToken::Expression(_) | ExpressionToken::Custom(_) => output.push(token),
                ExpressionToken::Operator(op1) => {
                    while let Some(&ExpressionToken::Operator(ref op2)) = operator_stack.last() {
                        match op1.precedence() <= op2.precedence() {
                            true => output.push(operator_stack.pop().unwrap()),
                            false => break,
                        }
                    }
                    operator_stack.push(op1.into());
                }
                ExpressionToken::OpenParenthesis => operator_stack.push(token),
                ExpressionToken::CloseParenthesis => while let Some(top) = operator_stack.pop() {
                    if let ExpressionToken::OpenParenthesis = top {
                        break;
                    }
         
                    output.push(top);
                }
            }
        }
    
        while let Some(op) = operator_stack.pop() {
            output.push(op);
        }
    
        output
    }

    const INVALID_RPN_TOKENS_MESSAGE: &'static str = "This is unexpected. If you reach this point, either check your input for errors or create an issue to report the problem.";
    
    /// Converts an expression in Reverse Polish Notation (RPN) represented by a vector of `ExpressionToken`
    /// into an expression tree represented by the `Expression` enum.
    ///
    /// This function takes a vector of tokens in RPN and constructs an expression tree based on
    /// the provided tokens. The resulting expression tree is returned as an `Option<Expression>`.
    ///
    /// # Arguments
    ///
    /// * `rpn_tokens` - A vector of `ExpressionToken` representing the expression in RPN format.
    pub(super) fn into_expression_tree(rpn_tokens  : Vec<ExpressionToken>) -> Expression {
        let mut stack: Vec<Expression> = Vec::new();

        for token in rpn_tokens.into_iter() {
            match token {
                ExpressionToken::Expression(expr) => stack.push(expr),
                ExpressionToken::Custom(value) => stack.push(Expression::Custom(value)),
                ExpressionToken::Operator(operator) => {
                    let right = stack.pop().expect(Self::INVALID_RPN_TOKENS_MESSAGE);
                    let left = stack.pop().expect(Self::INVALID_RPN_TOKENS_MESSAGE);
                    stack.push(Expression::new_binary(operator, left, right));  
                },
                ExpressionToken::OpenParenthesis | ExpressionToken::CloseParenthesis => unreachable!(),
            }
        }

        match stack.len() {
            1 => stack.pop().unwrap(),
            _ => panic!("{} for stack = {:?}",Self::INVALID_RPN_TOKENS_MESSAGE,stack)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use num_notation::Number;
    use test_case::test_case;

    #[test_case("2 + 3 * 4",[
        ExpressionToken::Expression(2.0.into()),
        ExpressionToken::Operator(ArithmeticOperation::Plus),
        ExpressionToken::Expression(3.0.into()),
        ExpressionToken::Operator(ArithmeticOperation::Mal),
        ExpressionToken::Expression(4.0.into()),
    ])]
    #[test_case("2 ++ 3",[
        ExpressionToken::Expression(2.0.into()),
        ExpressionToken::Operator(ArithmeticOperation::Plus.into()),
        ExpressionToken::Expression(3.into()),
    ])]
    #[test_case("5(2x - 3y) + z",[
        ExpressionToken::Expression(5.into()),
        ExpressionToken::Operator('*'.try_into().unwrap()), 
        ExpressionToken::OpenParenthesis,
        ExpressionToken::Expression((Number::Decimal(2f64),'x').into()), 
        ExpressionToken::Operator('-'.try_into().unwrap()), 
        ExpressionToken::Expression((Number::Decimal(3f64),'y').into()), 
        ExpressionToken::CloseParenthesis, 
        ExpressionToken::Operator('+'.try_into().unwrap()), 
        ExpressionToken::Expression('z'.into()), 
    ])]
    fn tokens_are_valid<const S : usize>(input : &str,expected_tokens : [ExpressionToken;S]) {
        let context = Default::default();
        let parsing_result = ExpressionToken::parse(&context)(input);
        assert_eq!(parsing_result,Ok(("",expected_tokens.into_iter().collect_vec())))
    }
    
    #[test]
    fn test_into_tokens_empty_input() {
        // Test with an empty input string
        let input = "";     
        let context = Default::default();   
        let result = ExpressionToken::parse(&context)(input);
        assert!(result.is_err());
    }
}