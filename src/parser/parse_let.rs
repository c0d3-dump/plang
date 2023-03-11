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
    fn test1() {
        assert_eq!(
            parse_let("let x = \" Hello\""),
            Ok((
                "",
                Statement::Let {
                    name: Expression::Identifier(String::from("x")),
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
                    name: Expression::Identifier(String::from("x")),
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
                    name: Expression::Identifier(String::from("x")),
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
                    name: Expression::Identifier(String::from("x1")),
                    initial: Expression::String(String::from("10"))
                }
            ))
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            parse_let("let x = (y + 1.0) * k"),
            Ok((
                "",
                Statement::Let {
                    name: Expression::Identifier(String::from("x")),
                    initial: Expression::Infix(
                        Expression::Infix(
                            Expression::Identifier(String::from("y")).boxed(),
                            Op::Add,
                            Expression::Number(1.0).boxed(),
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

    #[test]
    fn test7() {
        assert_eq!(
            parse_let("let x = [1, \"Hello\", true, a]"),
            Ok((
                "",
                Statement::Let {
                    name: Expression::Identifier(String::from("x")),
                    initial: Expression::List(vec![
                        Expression::Number(1.0),
                        Expression::String(String::from("Hello")),
                        Expression::Boolean(true),
                        Expression::Identifier(String::from("a")),
                    ])
                }
            ))
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            parse_let("let x = [1, [2, 3, [5]]]"),
            Ok((
                "",
                Statement::Let {
                    name: Expression::Identifier(String::from("x")),
                    initial: Expression::List(vec![
                        Expression::Number(1.0),
                        Expression::List(vec![
                            Expression::Number(2.0),
                            Expression::Number(3.0),
                            Expression::List(vec![Expression::Number(5.0),]),
                        ]),
                    ])
                }
            ))
        );
    }

    #[test]
    fn test9() {
        assert_eq!(
            parse_let("let x = { x: 1, y: false, 1: [1, 2], 2: { 3: false } }"),
            Ok((
                "",
                Statement::Let {
                    name: Expression::Identifier(String::from("x")),
                    initial: Expression::Dict(vec![
                        (
                            Expression::Identifier(String::from("x")),
                            Expression::Number(1.0)
                        ),
                        (
                            Expression::Identifier(String::from("y")),
                            Expression::Boolean(false)
                        ),
                        (
                            Expression::Number(1.0),
                            Expression::List(vec![
                                Expression::Number(1.0),
                                Expression::Number(2.0),
                            ])
                        ),
                        (
                            Expression::Number(2.0),
                            Expression::Dict(vec![(
                                Expression::Number(3.0),
                                Expression::Boolean(false)
                            )])
                        )
                    ])
                }
            ))
        );
    }

    #[test]
    fn test10() {
        assert_eq!(
            parse_let("let x = { l : mod(1) }"),
            Ok((
                "",
                Statement::Let {
                    name: Expression::Identifier(String::from("x")),
                    initial: Expression::Dict(vec![(
                        Expression::Identifier(String::from("l")),
                        Expression::Call(
                            Expression::Identifier(String::from("mod")).boxed(),
                            vec![Expression::Number(1.0)]
                        )
                    )])
                }
            ))
        );
    }
}
