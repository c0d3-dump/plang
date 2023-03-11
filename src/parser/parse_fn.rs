use crate::parser::ast::Statement;
use crate::parser::parse_block::parse_block;
use crate::parser::tools::{parse_identifier, parse_tag};
use crate::token::Token;

use nom::character::complete::multispace1;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::IResult;

pub fn parse_fn(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::FN)(input)?;

    let (input, x) = preceded(multispace1, parse_identifier)(input)?;

    let (input, y) = delimited(
        parse_tag(Token::LEFT_PAREN),
        separated_list0(parse_tag(Token::COMMA), parse_identifier),
        parse_tag(Token::RIGHT_PAREN),
    )(input)?;

    let (input, z) = parse_block(input)?;

    Ok((
        input,
        Statement::Fn {
            name: x,
            params: y,
            body: z,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Expression;

    #[test]
    fn test1() {
        assert_eq!(
            parse_fn("fn main() {  }"),
            Ok((
                "",
                Statement::Fn {
                    name: Expression::Identifier(String::from("main")),
                    params: vec![],
                    body: vec![]
                }
            ))
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_fn("fn main(x, y) {  }"),
            Ok((
                "",
                Statement::Fn {
                    name: Expression::Identifier(String::from("main")),
                    params: vec![
                        Expression::Identifier(String::from("x")),
                        Expression::Identifier(String::from("y")),
                    ],
                    body: vec![]
                }
            ))
        )
    }

    #[test]
    fn test3() {
        match parse_fn("fnmain() {  }") {
            Ok(_) => panic!(),
            Err(_) => {}
        }
    }
}
