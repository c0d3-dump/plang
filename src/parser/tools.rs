use crate::parser::ast::Expression;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    sequence::delimited,
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

pub fn parse_value(input: &str) -> IResult<&str, Expression> {
    let (input, x) = alt((parse_number, parse_string, parse_boolean))(input)?;
    Ok((input, x))
}
