use crate::parser::ast::Statement;
use crate::parser::parse_block::parse_block;
use crate::parser::tools::parse_tag;
use crate::parser::tools::parse_value;
use crate::token::Token;

use nom::sequence::tuple;
use nom::IResult;

use super::ast::Expression;
use super::tools::parse_identifier;

fn parse_iterator_value(input: &str) -> IResult<&str, (Expression, Expression)> {
    let (input, (x, _, y)) =
        tuple((parse_identifier, parse_tag(Token::COLON), parse_value))(input)?;
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
            let (input, y) = parse_block(input)?;

            Ok((
                input,
                Statement::Loop {
                    iterable: None,
                    value: None,
                    then: y,
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
}
