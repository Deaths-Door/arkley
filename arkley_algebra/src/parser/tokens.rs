use nom::{
    IResult, sequence::{delimited, pair, separated_pair},
    multi::fold_many0, 
    character::complete::{multispace0,char},
    combinator::{map, opt}, 
    branch::alt
};

use crate::{
    Expression, 
    ArithmeticOperation, 
    parse_term, parse_operator, 
    Term, Context, parse_function, Function,
};

use super::parse_add_sub;

#[cfg_attr(test, derive(PartialEq,Debug))]
pub(super) enum Token {
    Term(Term),
    Operator(ArithmeticOperation),
    OpenParenthesis,
    CloseParenthesis,

    /// Used for context parsing like "five_x_plus_y* x" => "(5x + y) * x" => "5x^2"
    Expression(Expression) 
}

impl From<Term> for Token {
    fn from(value: Term) -> Self {
        Token::Term(value)
    }
}

impl From<ArithmeticOperation> for Token {
    fn from(value: ArithmeticOperation) -> Self {
        Token::Operator(value)
    }
}

impl From<Expression> for Token {
    fn from(value: Expression) -> Self {
        Token::Expression(value)
    }
}

impl Token {
    fn parse_expression<'a>(context : &'a Context<'_>) -> impl FnMut(&'a str) -> IResult<&str, Vec<Token>> {
        move |input: &str| {    
            let (input,mut vec1) = Self::parse_with_opt_implicit_mul(context)(input)?;
    
            let parser = separated_pair(
                parse_operator, 
                multispace0, 
                Self::parse_with_opt_implicit_mul(context)
            );
            
            let (input,vec2) = fold_many0(parser,Vec::new,|mut vec : Vec<Token>,(operation,tokens)|{
                vec.push(operation.into());
    
                vec.extend(tokens.into_iter());
    
                vec
            })(input)?;
    
            vec1.extend(vec2.into_iter());
    
            Ok((input,vec1))
        }
    }

    fn parse_nested_expression<'a>(context : &'a Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Token>> {
        move |input : &'a str| {
            let (input,(sign,expr)) = delimited(
                pair(char('('),multispace0),
                pair(
                    opt(parse_add_sub),
                    Token::parse_expression(context)
                ), 
                pair(multispace0,char(')')),
            )(input)?;

            let mut vec = vec![];
                
            if let Some(value) = sign {
                vec.push(value.into())
            }

            vec.push(Token::OpenParenthesis);

            vec.extend(expr.into_iter());

            vec.push(Token::CloseParenthesis);


            Ok((input,vec))
        }
    }
    
    fn parse_with_opt_implicit_mul<'a>(context : &'a Context<'_>) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<Token>> {
        move |input| {    
            alt((
                Self::opt_implicit_mul_parser(context, Function::map_into_tokens(context)),
                Self::opt_implicit_mul_parser(context ,Self::parse_nested_expression(context)),
                Self::opt_implicit_mul_parser(context,Term::map_into_tokens()),
                Self::opt_implicit_mul_parser(context,context.parse_tags())
            ))(input)
        }
    }

    fn opt_implicit_mul_parser<'a>(context : &'a Context<'_>,first_parser : impl FnMut(&'a str) -> IResult<&str,Vec<Token>>) -> impl FnMut(&'a str) -> IResult<&str,Vec<Token>> {
        let mut parser = separated_pair(
            first_parser,
            multispace0,
            opt(Token::parse_expression(context))
        );
      
        move |input| {
            let (input,(lexpr,rexpr)) = (parser)(input)?;

            let mut vec = vec![];
            
            // length one means only one token which means only one thing hence no brackets are required
            match lexpr.len() == 1 {
                true => vec.extend(lexpr.into_iter()),
                false => {
                    vec.push(Token::OpenParenthesis);
    
                    vec.extend(lexpr.into_iter());
        
                    vec.push(Token::CloseParenthesis);
                }
            };
    
            if let Some(value) = rexpr {
                vec.push(ArithmeticOperation::Mal.into());
    
                vec.push(Token::OpenParenthesis);
    
                vec.extend(value.into_iter());
    
                vec.push(Token::CloseParenthesis);
    
            }

            Ok((input,vec))
        }
    }
}

impl Term {
    fn map_into_tokens<'a>() -> impl FnMut(&'a str) -> IResult<&'a str,Vec<Token>> {
        map(parse_term,|term| Vec::from([Token::from(term)]) )
    }
}

impl Function {
    fn map_into_tokens<'a>(context : &'a Context<'_> ) -> impl FnMut(&'a str) -> IResult<&'a str,Vec<Token>> {
        map(parse_function(context),|func| Vec::from([Token::from(Expression::from(func))]) )
    }
}

impl ArithmeticOperation {
    const fn precedence(&self) -> i32 {
        match self {
            ArithmeticOperation::Plus | ArithmeticOperation::Minus => 1,
            ArithmeticOperation::Mal | ArithmeticOperation::Durch => 2,
        }
    }
}

impl Token {
    /// Parses an input string into a vector of tokens.
    ///
    /// This function tokenizes an input string and converts it into a vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `input`: A reference to the input string to be parsed.
    ///
    /// # Returns
    ///
    /// - If the parsing is successful, it returns a `Result` with the remaining input and a
    ///   vector of `Token` representing the parsed tokens.
    #[inline(always)]
    pub(super) fn into_tokens<'a>(input: &'a str,context : &'a Context<'_>) -> IResult<&'a str,Vec<Token>>  {  
        Token::parse_expression(context)(input)
    }

    /// Converts an infix expression represented by a vector of `Token` into Reverse Polish Notation (RPN).
    ///
    /// This function takes a vector of tokens and uses the Shunting Yard algorithm to convert the
    /// infix expression into RPN. The resulting RPN expression is returned as a vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `vec` - A vector of `Token` representing the infix expression to be converted to RPN.
    ///
    /// # Returns
    ///
    /// A vector of `Token` representing the RPN expression.
    pub(super) fn to_rpn(vec : Vec<Token>) -> Vec<Token> {
        let mut output: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        for token in vec {
            match token {
                Token::Term(_) | Token::Expression(_) => output.push(token),
                Token::Operator(op1) => {
                    while let Some(&Token::Operator(ref op2)) = operator_stack.last() {
                        match op1.precedence() <= op2.precedence() {
                            true => output.push(operator_stack.pop().unwrap()),
                            false => break,
                        }
                    }
                    operator_stack.push(op1.into());
                }
                Token::OpenParenthesis => operator_stack.push(token),
                Token::CloseParenthesis => while let Some(top) = operator_stack.pop() {
                    if let Token::OpenParenthesis = top {
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

    /// Converts an expression in Reverse Polish Notation (RPN) represented by a vector of `Token`
    /// into an expression tree represented by the `Expression` enum.
    ///
    /// This function takes a vector of tokens in RPN and constructs an expression tree based on
    /// the provided tokens. The resulting expression tree is returned as an `Option<Expression>`.
    ///
    /// # Arguments
    ///
    /// * `rpn_tokens` - A vector of `Token` representing the expression in RPN format.
    pub(super) fn into_expression_tree(rpn_tokens  : Vec<Token>) -> Expression {
        let mut stack: Vec<Expression> = Vec::new();

        for token in rpn_tokens.into_iter() {
            match token {
                Token::Term(term) => stack.push(term.into()),
                Token::Expression(expr) => stack.push(expr),
                Token::Operator(operator) => {
                    let right = stack.pop().expect("Expected valid input : Check parser");
                    let left = stack.pop().expect("Expected valid input : Check parser");
                    stack.push(Expression::new_binary(operator, left, right));  
                },
                Token::OpenParenthesis | Token::CloseParenthesis => unreachable!(),
            }
        }

        match stack.len() {
            1 => stack.pop().unwrap(),
            _ => panic!("Expected valid input : Check parser")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_tokens_valid_input() {
        // Test with a valid input string
        let input = "2 + 3 * 4";
        let expected_tokens = vec![
            Token::Term(2.0.into()),
            Token::Operator(ArithmeticOperation::Plus),
            Token::Term(3.0.into()),
            Token::Operator(ArithmeticOperation::Mal),
            Token::Term(4.0.into()),
        ];
        
        assert_eq!(Token::into_tokens(input,&Default::default()), Ok(("", expected_tokens)));
    }

    #[test]
    fn test_into_tokens_empty_input() {
        // Test with an empty input string
        let input = "";     
        let context = Default::default();   
        let result = Token::into_tokens(input,&context);
        assert!(result.is_err());
    }

    #[test]
    fn test_into_tokens_special_input() {
        // Test with an invalid input string
        let input = "2 ++ 3";
        
        let expected_tokens: Vec<Token> = Vec::from([
            Term::new(2.0.into()).into(),
            ArithmeticOperation::Plus.into(),
            Term::new(3.0.into()).into(),
        ]);

        // Expect an error since "++" is not a valid operator
        assert_eq!(Token::into_tokens(input,&Default::default()), Ok(("", expected_tokens)));
    }
}