use super::ast::Op;
use crate::parser::ast::Expression;
use crate::token::Token;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::peek,
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

// ****************
// public functions
// ****************

pub fn parse_tag<'a>(t: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| preceded(multispace0, tag(t))(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, Expression> {
    let (input, (_, x)) = preceded(multispace0, tuple((peek(alpha1), alphanumeric1)))(input)?;
    Ok((input, Expression::Identifier(String::from(x))))
}

pub fn parse_value(input: &str) -> IResult<&str, Expression> {
    alt((parse_string, parse_boolean, parse_math_expr))(input)
}

pub fn parse_iterator(input: &str) -> IResult<&str, Expression> {
    alt((parse_list, parse_dict))(input)
}

// ****************
// helper functions
// ****************

fn parse_string(input: &str) -> IResult<&str, Expression> {
    let (input, x) = preceded(
        multispace0,
        delimited(tag("\""), take_until("\""), tag("\"")),
    )(input)?;
    Ok((input, Expression::String(String::from(x))))
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    let (input, x) = preceded(multispace0, double)(input)?;
    Ok((input, Expression::Number(x)))
}

fn parse_boolean(input: &str) -> IResult<&str, Expression> {
    let (input, x) = preceded(multispace0, alt((tag("true"), tag("false"))))(input)?;
    Ok((input, Expression::Boolean(x.parse().unwrap())))
}

fn parse_list(input: &str) -> IResult<&str, Expression> {
    let (input, x) = delimited(
        parse_tag(Token::LEFT_BRACKET),
        separated_list0(
            parse_tag(Token::COMMA),
            alt((
                parse_string,
                parse_number,
                parse_boolean,
                parse_identifier,
                parse_list,
            )),
        ),
        parse_tag(Token::RIGHT_BRACKET),
    )(input)?;
    Ok((input, Expression::List(x)))
}

fn parse_dict(input: &str) -> IResult<&str, Expression> {
    let (input, x) = delimited(
        parse_tag(Token::LEFT_BRACE),
        separated_list0(
            parse_tag(Token::COMMA),
            separated_pair(
                alt((parse_number, parse_boolean, parse_identifier)),
                parse_tag(Token::COLON),
                alt((parse_value, parse_iterator)),
            ),
        ),
        parse_tag(Token::RIGHT_BRACE),
    )(input)?;
    Ok((input, Expression::Dict(x)))
}

fn parse_raw_value(input: &str) -> IResult<&str, Expression> {
    alt((parse_number, parse_identifier))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expression> {
    delimited(
        parse_tag(Token::LEFT_PAREN),
        parse_math_expr,
        parse_tag(Token::RIGHT_PAREN),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expression> {
    alt((parse_parens, parse_raw_value))(input)
}

fn parse_assignment(input: &str) -> IResult<&str, Expression> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((
        alt((
            parse_tag(Token::EQUAL),
            parse_tag(Token::NOT_EQUAL),
            parse_tag(Token::GREATER_THAN),
            parse_tag(Token::LESS_THAN),
            parse_tag(Token::GREATER_THAN_EQUAL),
            parse_tag(Token::LESS_THAN_EQUAL),
        )),
        parse_assignment,
    )))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_cond(input: &str) -> IResult<&str, Expression> {
    let (input, num1) = parse_assignment(input)?;
    let (input, exprs) = many0(tuple((
        alt((parse_tag(Token::AND), parse_tag(Token::OR))),
        parse_assignment,
    )))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, num1) = parse_cond(input)?;
    let (input, exprs) = many0(tuple((
        alt((parse_tag(Token::DIVIDE), parse_tag(Token::MULTIPLY))),
        parse_cond,
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
        Token::EQUAL => Expression::Infix(expr1.boxed(), Op::Equals, expr2.boxed()),
        Token::NOT_EQUAL => Expression::Infix(expr1.boxed(), Op::NotEquals, expr2.boxed()),
        Token::GREATER_THAN => Expression::Infix(expr1.boxed(), Op::GreaterThan, expr2.boxed()),
        Token::LESS_THAN => Expression::Infix(expr1.boxed(), Op::LessThan, expr2.boxed()),
        Token::GREATER_THAN_EQUAL => {
            Expression::Infix(expr1.boxed(), Op::GreaterThanOrEquals, expr2.boxed())
        }
        Token::LESS_THAN_EQUAL => {
            Expression::Infix(expr1.boxed(), Op::LessThanOrEquals, expr2.boxed())
        }
        Token::AND => Expression::Infix(expr1.boxed(), Op::And, expr2.boxed()),
        Token::OR => Expression::Infix(expr1.boxed(), Op::Or, expr2.boxed()),
        _ => panic!("Unknown Operation"),
    }
}
