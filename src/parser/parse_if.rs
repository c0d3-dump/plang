use crate::parser::ast::Statement;
use crate::parser::parse_block::parse_block;
use crate::parser::tools::parse_tag;
use crate::parser::tools::parse_value;
use crate::token::Token;

use nom::IResult;

fn parse_else(input: &str) -> IResult<&str, Option<Vec<Statement>>> {
    match parse_tag(Token::ELSE)(input) {
        Ok((input, ..)) => {
            let (input, y) = parse_block(input)?;

            Ok((input, Some(y)))
        }
        Err(_e) => Ok((input, None)),
    }
}

pub fn parse_if(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::IF)(input)?;

    let (input, x) = parse_value(input)?;

    let (input, y) = parse_block(input)?;

    let (input, z) = parse_else(input)?;

    Ok((
        input,
        Statement::If {
            condition: x,
            then: y,
            otherwise: z,
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
            parse_if("if true { let t = 0 } else { }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::Boolean(true),
                    then: vec![Statement::Let {
                        name: Expression::Identifier(String::from("t")),
                        initial: Expression::Number(0.0)
                    }],
                    otherwise: Some(vec![])
                }
            ))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_if("if x == 1 { }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::Infix(
                        Expression::Identifier(String::from("x")).boxed(),
                        Op::Equals,
                        Expression::Number(1.0).boxed(),
                    ),
                    then: vec![],
                    otherwise: None
                }
            ))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            parse_if("if x == 1 and y != 2 { }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::Infix(
                        Expression::Infix(
                            Expression::Identifier(String::from("x")).boxed(),
                            Op::Equals,
                            Expression::Number(1.0).boxed(),
                        )
                        .boxed(),
                        Op::And,
                        Expression::Infix(
                            Expression::Identifier(String::from("y")).boxed(),
                            Op::NotEquals,
                            Expression::Number(2.0).boxed(),
                        )
                        .boxed(),
                    ),
                    then: vec![],
                    otherwise: None
                }
            ))
        );
    }
}
