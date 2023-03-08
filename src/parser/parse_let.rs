use crate::parser::ast::Statement;
use crate::parser::tools::parse_value;
use crate::token::Token;

use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1},
    sequence::tuple,
    IResult,
};

pub fn parse_let(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = preceded(space0, tag(Token::LET))(input)?;

    let (input, x) = preceded(space1, alphanumeric1)(input)?;

    let (input, ..) = tuple((space0, tag(Token::ASSIGN), space0))(input)?;

    let (input, y) = parse_value(input)?;

    Ok((
        input,
        Statement::Let {
            name: String::from(x),
            initial: y,
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
            parse_let("let x = \" Hello\""),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x"),
                    initial: Expression::String(String::from(" Hello"))
                }
            ))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_let("let x = 10"),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x"),
                    initial: Expression::Number(10.0)
                }
            ))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            parse_let("let x = true"),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x"),
                    initial: Expression::Boolean(true)
                }
            ))
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            parse_let("let x = \"10\""),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x"),
                    initial: Expression::String(String::from("10"))
                }
            ))
        );
    }

    #[test]
    fn test5() {
        match parse_let("let x = Hello") {
            Ok(..) => {
                panic!("need to fail")
            }
            Err(_e) => {}
        }
    }
}
