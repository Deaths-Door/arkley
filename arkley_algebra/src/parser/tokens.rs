use std::vec;

use nom::{
    IResult, Parser, 
    sequence::{tuple, delimited, pair},
    multi::fold_many0, 
    character::complete::{multispace0,char},
    combinator::{map, opt}
};

use crate::{
    Expression, 
    ArithmeticOperation, 
    parse_term, parse_operator, 
    Term,
};

use super::parse_final_add_sub;

#[cfg_attr(test, derive(PartialEq,Debug))]
pub(super) enum Token {
    Term(Term),
    Operator(ArithmeticOperation),
    OpenParenthesis,
    CloseParenthesis
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

impl Token {
    fn parse_expression(input: &str) -> IResult<&str, Vec<Token>> {
        let (input,first_term) = parse_term(input)?;

        let implicit_mul = delimited(
            tuple((multispace0,char('('),multispace0)), 
            parse_term, 
            tuple((multispace0,char(')'))), 
        );

        // TODO : Handle case like (x - 2)(x + 2) while parsing
        let parser = tuple((
            multispace0,
            parse_operator,
            multispace0,
            Token::parse_nested_expression.or(
                map(
                    pair(parse_term, opt(implicit_mul)),
                    |(term,optional_implicit_mul_term)| {
                        let mut vec = vec![term.into()];

                        if let Some(value) = optional_implicit_mul_term {
                            vec.push(ArithmeticOperation::Mal.into());
                            vec.push(value.into());
                        }

                        vec
                    }
                )
            )
        ));

        fold_many0(parser,move || vec![first_term.clone().into()],|mut vec : Vec<Token>,(_,operation,_,tokens)|{
            vec.push(operation.into());

            vec.extend(tokens.into_iter());

            vec
        })(input)
    }

    fn parse_nested_expression(input: &str) -> IResult<&str, Vec<Token>> {    
        let parse = tuple((
            opt(parse_final_add_sub),
            char('('),
            multispace0,
            Token::parse_expression,
            multispace0,
            char(')'),
        ));    

        map(parse,|(operation,_,_,inner_expr_tokens,_,_)|{
            let mut vec = vec![];
            
            if let Some(value) = operation {
                vec.push(value.into())
            };

            vec.push(Token::OpenParenthesis);

            vec.extend(inner_expr_tokens.into_iter());

            vec.push(Token::CloseParenthesis);

            vec
        })(input)
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
    pub(super) fn into_tokens(input: &str) -> IResult<&str,Vec<Token>>  {  
        Token::parse_expression(input)
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
                Token::Term(_) => output.push(token),
                Token::Operator(op1) => {
                    while let Some(&Token::Operator(ref op2)) = operator_stack.last() {
                        if op1.precedence() <= op2.precedence() {
                            output.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(Token::Operator(op1));
                }
                Token::OpenParenthesis => operator_stack.push(token),
                Token::CloseParenthesis => {
                    output.push(Token::CloseParenthesis);
                    
                    while let Some(top) = operator_stack.pop() {
                        if let Token::OpenParenthesis = top {
                            break;
                        }
             
                        output.push(top);
                    }

                    output.push(Token::OpenParenthesis);
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
    ///
    /// # Returns
    ///
    /// An `Option<Expression>` containing the expression tree if the conversion is successful.
    /// If the RPN expression is invalid or incomplete, `None` is returned.
    pub(super) fn into_expression_tree(rpn_tokens  : Vec<Token>) -> Option<Expression> {
        let mut stack: Vec<Expression> = Vec::new();

        // TODO : MAYBE give more infomation , why give expr is invalid instead of just None
        // TODO : Figure out technique without cloning the contents
        for token in rpn_tokens.into_iter() {
            match token {
                Token::Term(term) => stack.push(term.into()),
                Token::Operator(operator) => {
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    stack.push(Expression::new_binary(operator, left, right));  
                },
                _ => todo!()
                /* TODO : Opposite of what I did 
                [Term(1), Operator(+), OpenParenthesis, Term(2), Operator(*), Term(3), CloseParenthesis]
                [Term(1), Term(2), Term(3), CloseParenthesis, Operator(*), OpenParenthesis, Operator(+)]CloseParenthesis is true so None
                thread 'parser::expression::tests::parse_complex_expression' panicked at 'assertion failed: `(left == right)`
                left: `None`,
                right: `Some(1 + (2(3)))`', arkley_algebra\src\parser\expression.rs:53:9
                note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
                 */
               /* Token::OpenParenthesis => { 
                    /*let corresponding_bracket_index = || -> Option<usize> {
                        let mut brackets = 1;

                        for (new_index, token) in rpn_tokens.iter().enumerate().skip(index + 1) {
                            match token {
                                Token::OpenParenthesis => brackets += 1,
                                Token::CloseParenthesis => {
                                    brackets -= 1;
                                    if brackets == 0 {
                                        return Some(index + 1 + new_index);
                                    }
                                }
                                _ => {}
                            }
                        }
                        None
                    };

                    match corresponding_bracket_index() {
                        None => {
                            println!("corresponding_bracket_index is None");
                            return None
                        }
                        Some(end_index) => match Token::into_expression_tree(&rpn_tokens[index..end_index]) {
                            None => {
                                println!("into_expression_tree is None");

                                return None;
                            }//return None,
                            Some(inner) => stack.push(Expression::new_nested(inner))
                        }
                    }*/
                },
                Token::CloseParenthesis => {
                    println!("CloseParenthesis is true so None");
                    return None
                },*/
            }
        }

        match stack.len() {
            1 => Some(stack.pop().unwrap()),
            _ => {
                println!(" stack.len() != 1 so None");
                None
            }
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
        
        assert_eq!(Token::into_tokens(input), Ok(("", expected_tokens)));
    }

    #[test]
    fn test_into_tokens_empty_input() {
        // Test with an empty input string
        let input = "";
        let expected_tokens: Vec<Token> = Vec::from([Term::new(1.0.into()).into()]);
        
        assert_eq!(Token::into_tokens(input), Ok(("", expected_tokens)));
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
        assert_eq!(Token::into_tokens(input), Ok(("", expected_tokens)));
    }
}
