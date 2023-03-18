use crate::parser::ast::Statement;
use crate::parser::tools::{parse_iterator, parse_tag, parse_value};
use crate::token::Token;

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::sequence::preceded;
use nom::IResult;

pub fn parse_return(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::RETURN)(input)?;

    match preceded(multispace1, alt((parse_value, parse_iterator)))(input) {
        Ok((input, x)) => Ok((input, Statement::Return { value: Some(x) })),
        Err(_) => Ok((input, Statement::Return { value: None })),
    }
}

pub fn parse_break(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::BREAK)(input)?;
    Ok((input, Statement::Break))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Expression;

    #[test]
    fn test1() {
        assert_eq!(
            parse_return("return x"),
            Ok((
                "",
                Statement::Return {
                    value: Some(Expression::Identifier(String::from("x")))
                }
            ))
        )
    }

    #[test]
    fn test2() {
        assert_eq!(parse_break("break"), Ok(("", Statement::Break)))
    }
}
