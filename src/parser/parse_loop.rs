use crate::parser::ast::{Expression, Statement};
use crate::parser::parse_block::parse_block;
use crate::parser::tools::{parse_identifier, parse_iterator, parse_tag, parse_value};
use crate::token::Token;

use nom::branch::alt;
use nom::sequence::tuple;
use nom::IResult;

fn parse_iterator_value(input: &str) -> IResult<&str, (Expression, Expression)> {
    let (input, (x, _, y)) = tuple((
        parse_identifier,
        parse_tag(Token::COLON),
        alt((parse_identifier, parse_iterator)),
    ))(input)?;
    Ok((input, (x, y)))
}

pub fn parse_loop(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::LOOP)(input)?;

    match parse_iterator_value(input) {
        Ok((input, (x, y))) => {
            let (input, z) = parse_block(input)?;

            Ok((
                input,
                Statement::Loop {
                    iterable: x.some(),
                    value: y.some(),
                    then: z,
                },
            ))
        }
        Err(..) => {
            let (input, z) = parse_block(input)?;

            Ok((
                input,
                Statement::Loop {
                    iterable: None,
                    value: None,
                    then: z,
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            parse_loop(" loop { }"),
            Ok((
                "",
                Statement::Loop {
                    iterable: None,
                    value: None,
                    then: vec![]
                }
            ))
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_loop(" loop x : p { }"),
            Ok((
                "",
                Statement::Loop {
                    iterable: Expression::Identifier(String::from("x")).some(),
                    value: Expression::Identifier(String::from("p")).some(),
                    then: vec![]
                }
            ))
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            parse_loop(" loop y : [1, 2 , 3] { }"),
            Ok((
                "",
                Statement::Loop {
                    iterable: Expression::Identifier(String::from("y")).some(),
                    value: Expression::List(vec![
                        Expression::Number(1.0),
                        Expression::Number(2.0),
                        Expression::Number(3.0),
                    ])
                    .some(),
                    then: vec![]
                }
            ))
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            parse_loop(" loop y : { 1 : \"H\", 2: true} { }"),
            Ok((
                "",
                Statement::Loop {
                    iterable: Expression::Identifier(String::from("y")).some(),
                    value: Expression::Dict(vec![
                        (
                            Expression::Number(1.0),
                            Expression::String(String::from("H"))
                        ),
                        (Expression::Number(2.0), Expression::Boolean(true)),
                    ])
                    .some(),
                    then: vec![]
                }
            ))
        )
    }
}
