use crate::parser::ast::{Expression, Statement};
use crate::parser::tools::parse_value;
use crate::parser::tools::{parse_identifier, parse_tag};
use crate::token::Token;

use nom::sequence::preceded;
use nom::{
    character::complete::{space0, space1},
    IResult,
};

pub fn parse_let(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::LET)(input)?;

    let (input, x) = preceded(space1, parse_identifier)(input)?;

    let (input, ..) = parse_tag(Token::ASSIGN)(input)?;

    let (input, y) = preceded(space0, parse_value)(input)?;

    Ok((
        input,
        Statement::Let {
            name: match x {
                Expression::Identifier(g) => g,
                _ => panic!(),
            },
            initial: y,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Expression, Op};

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
            parse_let("let x = 10.0 + 3 * 2"),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x"),
                    initial: Expression::Infix(
                        Expression::Number(10.0).boxed(),
                        Op::Add,
                        Expression::Infix(
                            Expression::Number(3.0).boxed(),
                            Op::Multiply,
                            Expression::Number(2.0).boxed(),
                        )
                        .boxed()
                    )
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
            parse_let("let x1 = \"10\""),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x1"),
                    initial: Expression::String(String::from("10"))
                }
            ))
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            parse_let("let x = (y + z) * k"),
            Ok((
                "",
                Statement::Let {
                    name: String::from("x"),
                    initial: Expression::Infix(
                        Expression::Infix(
                            Expression::Identifier(String::from("y")).boxed(),
                            Op::Add,
                            Expression::Identifier(String::from("z")).boxed(),
                        )
                        .boxed(),
                        Op::Multiply,
                        Expression::Identifier(String::from("k")).boxed(),
                    )
                }
            ))
        );
    }

    #[test]
    fn test6() {
        match parse_let("let 1x = 10") {
            Ok(..) => {
                panic!("need to fail")
            }
            Err(_e) => {}
        }
    }
}
