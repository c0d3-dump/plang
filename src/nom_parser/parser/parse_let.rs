use crate::nom_parser::{
    ast::{Expression, Statement},
    token as Token,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, digit1, space0, space1},
    sequence::{delimited, tuple},
    IResult,
};

fn parse_string(input: &str) -> IResult<&str, Expression> {
    let (input, x) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    Ok((input, Expression::String(String::from(x))))
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    let (input, x) = digit1(input)?;
    Ok((input, Expression::Number(x.parse().unwrap())))
}

fn parse_boolean(input: &str) -> IResult<&str, Expression> {
    let (input, x) = alt((tag("true"), tag("false")))(input)?;
    Ok((input, Expression::Boolean(x.parse().unwrap())))
}

fn parse_value(input: &str) -> IResult<&str, Expression> {
    let (input, x) = alt((parse_number, parse_string, parse_boolean))(input)?;
    Ok((input, x))
}

pub fn parse_let(input: &str) -> IResult<&str, Statement> {
    let (input, (..)) = tuple((space0, tag(Token::LET)))(input)?;

    let (input, (_, x)) = tuple((space1, alphanumeric1))(input)?;

    let (input, (..)) = tuple((space0, tag(Token::ASSIGN), space0))(input)?;

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
        match parse_let("let x = \"Hello") {
            Ok(..) => {
                panic!("need to fail")
            }
            Err(_e) => {}
        }
    }
}
