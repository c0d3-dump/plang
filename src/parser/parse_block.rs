use crate::parser::ast::Statement;
use crate::parser::tools::parse_tag;
use crate::token::Token;

use nom::branch::alt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;

use super::parse_if::parse_if;
use super::parse_let::parse_let;
use super::parse_loop::parse_loop;

pub fn parse_items(input: &str) -> IResult<&str, Vec<Statement>> {
    many0(alt((parse_let, parse_if, parse_loop)))(input)
}

pub fn parse_block(input: &str) -> IResult<&str, Vec<Statement>> {
    delimited(
        parse_tag(Token::LEFT_BRACE),
        parse_items,
        parse_tag(Token::RIGHT_BRACE),
    )(input)
}
