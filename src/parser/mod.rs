use nom::IResult;

use ast::Statement;
use parse_block::parse_items;

pub mod ast;
mod parse_block;
mod parse_expr;
mod parse_fn;
mod parse_if;
mod parse_let;
mod parse_loop;
mod tools;

pub fn parse(input: &str) -> IResult<&str, Vec<Statement>> {
    parse_items(input)
}
