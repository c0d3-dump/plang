use crate::parser::ast::Statement;
use crate::parser::tools::{parse_identifier, parse_iterator, parse_tag, parse_value};
use crate::token::Token;

use nom::branch::alt;
use nom::IResult;

pub fn parse_let(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::LET)(input)?;

    let (input, x) = parse_identifier(input)?;

    let (input, ..) = parse_tag(Token::ASSIGN)(input)?;

    let (input, y) = alt((parse_value, parse_iterator))(input)?;

    Ok((
        input,
        Statement::Let {
            name: x,
            initial: y,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Expression, Op};

    #[test]
    fn test1() {}
}
