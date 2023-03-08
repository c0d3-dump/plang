use super::ast::Op;
use crate::parser::ast::Expression;
use crate::token::Token;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, space0},
    combinator::peek,
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded, tuple},
    IResult,
};

// ****************
// public functions
// ****************

pub fn parse_tag<'a>(t: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| preceded(space0, tag(t))(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, Expression> {
    let (input, (_, x)) = tuple((peek(alpha1), alphanumeric1))(input)?;
    Ok((input, Expression::Identifier(String::from(x))))
}

pub fn parse_value(input: &str) -> IResult<&str, Expression> {
    parse_math_expr(input)
}

// ****************
// helper functions
// ****************

fn parse_string(input: &str) -> IResult<&str, Expression> {
    let (input, x) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    Ok((input, Expression::String(String::from(x))))
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    let (input, x) = double(input)?;
    Ok((input, Expression::Number(x)))
}

fn parse_boolean(input: &str) -> IResult<&str, Expression> {
    let (input, x) = alt((tag("true"), tag("false")))(input)?;
    Ok((input, Expression::Boolean(x.parse().unwrap())))
}

fn parse_raw_value(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_number,
        parse_string,
        parse_boolean,
        parse_identifier,
        parse_math_expr,
    ))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expression> {
    delimited(
        parse_tag(Token::LEFT_PAREN),
        parse_math_expr,
        parse_tag(Token::RIGHT_PAREN),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expression> {
    alt((parse_parens, preceded(space0, parse_raw_value)))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((
        alt((parse_tag(Token::DIVIDE), parse_tag(Token::MULTIPLY))),
        parse_term,
    )))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expression> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((
        alt((parse_tag(Token::ADDITION), parse_tag(Token::SUBTRACTION))),
        parse_term,
    )))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expression, rem: Vec<(&str, Expression)>) -> Expression {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (&str, Expression), expr1: Expression) -> Expression {
    let (op, expr2) = tup;
    match op {
        Token::ADDITION => Expression::Infix(expr1.boxed(), Op::Add, expr2.boxed()),
        Token::SUBTRACTION => Expression::Infix(expr1.boxed(), Op::Subtract, expr2.boxed()),
        Token::MULTIPLY => Expression::Infix(expr1.boxed(), Op::Multiply, expr2.boxed()),
        Token::DIVIDE => Expression::Infix(expr1.boxed(), Op::Divide, expr2.boxed()),
        _ => panic!("Unknown Operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            parse_math_expr("1 +2"),
            Ok((
                "",
                Expression::Infix(
                    Expression::Number(1.0).boxed(),
                    Op::Add,
                    Expression::Number(2.0).boxed(),
                )
            ))
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_math_expr("( 1+ 2) *3"),
            Ok((
                "",
                Expression::Infix(
                    Expression::Infix(
                        Expression::Number(1.0).boxed(),
                        Op::Add,
                        Expression::Number(2.0).boxed(),
                    )
                    .boxed(),
                    Op::Multiply,
                    Expression::Number(3.0).boxed()
                )
            ))
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            parse_math_expr("( a+ b) *5"),
            Ok((
                "",
                Expression::Infix(
                    Expression::Infix(
                        Expression::Identifier(String::from("a")).boxed(),
                        Op::Add,
                        Expression::Identifier(String::from("b")).boxed(),
                    )
                    .boxed(),
                    Op::Multiply,
                    Expression::Number(5.0).boxed()
                )
            ))
        )
    }
}
