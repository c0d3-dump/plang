use crate::parser::ast::Statement;
use crate::parser::parse_expr::parse_expr;
use crate::parser::parse_fn::parse_fn;
use crate::parser::parse_if::parse_if;
use crate::parser::parse_let::parse_let;
use crate::parser::parse_loop::parse_loop;
use crate::parser::tools::parse_tag;
use crate::token::Token;

use nom::branch::alt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse_items(input: &str) -> IResult<&str, Vec<Statement>> {
    many0(alt((parse_let, parse_if, parse_loop, parse_fn, parse_expr)))(input)
}

pub fn parse_block(input: &str) -> IResult<&str, Vec<Statement>> {
    delimited(
        parse_tag(Token::LEFT_BRACE),
        parse_items,
        parse_tag(Token::RIGHT_BRACE),
    )(input)
}
