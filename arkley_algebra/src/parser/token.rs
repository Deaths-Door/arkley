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
    // TODO : Check if this rly improves output
    Term(Term),
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

    fn parse_with_optional_implicit_mul<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,(lexpr_tokens,rexpr_tokens)) = 
                separated_pair(
                    alt((
                        Term::map_into_tokens(),
                        Self::parse_nested_expression(context),
                        context.parse_tags(),
                        context.parse_values()
                    )),
                    multispace0,
                    opt(Self::parse_nested_expression(context)).map(|v| v.unwrap_or_default())
                )(input)?;
           

            let mut tokens = vec![];
            
            // Do not put brackets around a single token
            match lexpr_tokens.len() == 1 {
                true => tokens.extend(lexpr_tokens.into_iter()),
                false => {
                    tokens.push(ExpressionToken::OpenParenthesis);
    
                    tokens.extend(lexpr_tokens.into_iter());
        
                    tokens.push(ExpressionToken::CloseParenthesis);
                }
            };
    
            if !rexpr_tokens.is_empty() {
                tokens.push(ArithmeticOperation::Mal.into());
    
                // No requirement to add bracktets as [Self::parse_nested_expression] already adds it
                tokens.extend(rexpr_tokens.into_iter());    
            }

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

/*
impl ExpressionToken {
    // TODO : ADD CUSTOM PARSERS TO IT
   /* */ // Genenral shit $expr ...
    pub(super) fn parse<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,mut tokens) = Self::parse_with_optional_implicit_mul(context)(input)?;

            let inner_parser = separated_pair(
                pair(multispace0,parse_operator), 
                multispace0, 
                Self::parse_with_optional_implicit_mul(context)
            );

            let (input,mut follow_up_tokens) = fold_many0(inner_parser,Vec::new, |mut tokens,(operation,inner_tokens) | {
                tokens.push(operation.1.into());
                tokens.extend(inner_tokens.into_iter());
                tokens
            })(input)?;

            tokens.append(&mut follow_up_tokens);

            Ok((input,follow_up_tokens))
        }
    }

    // Implicit Mul $term | $op ($expr)
    fn parse_with_optional_implicit_mul<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,(lexpr,rexpr)) = separated_pair(
                alt((
                    Term::map_into_tokens(),
                    Self::parse_nested_expression(context),
                    context.parse_values(),
                    context.parse_tags(),
                )),
                multispace0,
                opt(ExpressionToken::parse(context))
            )(input)?;

            let mut vec = vec![];
            
            // length one means only one token which means only one thing hence no brackets are required
            match lexpr.len() == 1 {
                true => vec.extend(lexpr.into_iter()),
                false => {
                    vec.push(ExpressionToken::OpenParenthesis);
    
                    vec.extend(lexpr.into_iter());
        
                    vec.push(ExpressionToken::CloseParenthesis);
                }
            };
    
            if let Some(value) = rexpr {
                vec.push(ArithmeticOperation::Mal.into());
    
                vec.push(ExpressionToken::OpenParenthesis);
    
                vec.extend(value.into_iter());
    
                vec.push(ExpressionToken::CloseParenthesis);
    
            }

            Ok((input,vec))
        }
    }
     /*    move |input| {
            // Basically a term can be followed by an optional nested expression
            // But '+' or '-' needs to be followed by an nested expression
            let (input,(lexpr_tokens,rexpr_tokens)) = alt((
                separated_pair(
                    ArithmeticOperation::map_add_and_subtract_into_tokens(), 
                    multispace0, 
                    Self::parse_nested_expression(context)
                ),
                separated_pair(
                    alt((
                        Term::map_into_tokens(), 
                        Self::parse_nested_expression(context),
                        context.parse_values(),
                        context.parse_tags()
                    )), 
                    multispace0, 
                    opt(Self::parse_nested_expression(context))
                ).map(|(l,r)| (l,r.unwrap_or_default()))
            ))(input)?;

            let mut tokens = vec![];

            // Do not put brackets around a single token
            match lexpr_tokens.len() == 1 {
                true => tokens.extend(lexpr_tokens.into_iter()),
                false => {
                    tokens.push(ExpressionToken::OpenParenthesis);
                    tokens.extend(lexpr_tokens.into_iter());
                    tokens.push(ExpressionToken::CloseParenthesis);
                }
            }
            
            if !rexpr_tokens.is_empty() {
                tokens.push(ArithmeticOperation::Mal.into());

                // No requirement to add bracktets as [Self::parse_nested_expression] already adds it
                tokens.extend(rexpr_tokens.into_iter());
            }

            Ok((input,tokens))
        }
    }
*/
    // Double Brackets ($expr)($expr)
    fn parse_nested_expression<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,mut tokens) = delimited(
                pair(char('('),multispace0),
                ExpressionToken::parse(context), 
                pair(multispace0,char(')')),
            )(input)?;
            
            tokens.insert(0, ExpressionToken::OpenParenthesis);
            tokens.push(ExpressionToken::OpenParenthesis);
            
            Ok((input,tokens))
        }
    }
    /*
     fn parse_with_optional_implicit_mul<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,(lexpr,rexpr)) = separated_pair(
                alt((
                    Term::map_into_tokens(),
                    Self::parse_nested_expression(context),
                    context.parse_values(),
                    context.parse_tags(),
                )),
                multispace0,
                opt(ExpressionToken::parse(context))
            )(input)?;

            let mut vec = vec![];
            
            // length one means only one token which means only one thing hence no brackets are required
            match lexpr.len() == 1 {
                true => vec.extend(lexpr.into_iter()),
                false => {
                    vec.push(ExpressionToken::OpenParenthesis);
    
                    vec.extend(lexpr.into_iter());
        
                    vec.push(ExpressionToken::CloseParenthesis);
                }
            };
    
            if let Some(value) = rexpr {
                vec.push(ArithmeticOperation::Mal.into());
    
                vec.push(ExpressionToken::OpenParenthesis);
    
                vec.extend(value.into_iter());
    
                vec.push(ExpressionToken::CloseParenthesis);
    
            }

            Ok((input,vec))
        }
    }
*//* 
    fn parse_nested_expression<'a : 'b,'b>(context : &'b Context<'b>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> + 'b {
        move |input| {
            let (input,(sign,expr)) = delimited(
                pair(char('('),multispace0),
                pair(
                    opt(parse_add_sub),
                    ExpressionToken::parse(context)
                ), 
                pair(multispace0,char(')')),
            )(input)?;

            let mut vec = vec![];

            if let Some(value) = sign {
                vec.push(value.into())
            }

            vec.push(ExpressionToken::OpenParenthesis);

            vec.extend(expr.into_iter());

            vec.push(ExpressionToken::CloseParenthesis);

            Ok((input,vec))
        }
    }*/
}
*/
impl ArithmeticOperation {
    fn map_add_and_subtract_into_tokens<'a>() -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> {
        map(parse_add_sub,|op| Vec::from([ExpressionToken::Operator(op)]) )
    }
}

impl Term {
    fn map_into_tokens<'a>() -> impl FnMut(&'a str) -> IResult<&'a str,Vec<ExpressionToken>> {
        map(parse_term,|term| Vec::from([ExpressionToken::Term(term)]) )
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
                ExpressionToken::Term(_) | ExpressionToken::Expression(_) | ExpressionToken::Custom(_) => output.push(token),
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
                ExpressionToken::Term(term) => stack.push(term.into()),
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
        ExpressionToken::Term(2.0.into()),
        ExpressionToken::Operator(ArithmeticOperation::Plus),
        ExpressionToken::Term(3.0.into()),
        ExpressionToken::Operator(ArithmeticOperation::Mal),
        ExpressionToken::Term(4.0.into()),
    ])]
    #[test_case("2 ++ 3",[
        ExpressionToken::Term(2.0.into()),
        ExpressionToken::Operator(ArithmeticOperation::Plus.into()),
        ExpressionToken::Term(3.into()),
    ])]
    #[test_case("5(2x - 3y) + z",[
        ExpressionToken::Term(5.into()),
        ExpressionToken::Operator('*'.try_into().unwrap()), 
        ExpressionToken::OpenParenthesis,
        ExpressionToken::Term((Number::Decimal(2f64),'x').into()), 
        ExpressionToken::Operator('-'.try_into().unwrap()), 
        ExpressionToken::Term((Number::Decimal(3f64),'y').into()), 
        ExpressionToken::CloseParenthesis, 
        ExpressionToken::Operator('+'.try_into().unwrap()), 
        ExpressionToken::Term('z'.into()), 
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