use crate::parser::ast::Statement;
use crate::parser::tools::parse_value;
use crate::token::Token;

use nom::bytes::complete::take_until;
use nom::sequence::{delimited, preceded};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, space0, space1},
    sequence::tuple,
    IResult,
};

use super::ast::Expression;
use super::tools::parse_tag;

pub fn parse_if(input: &str) -> IResult<&str, Statement> {
    let (input, ..) = parse_tag(Token::IF)(input)?;

    let (input, x) = preceded(space0, parse_value)(input)?;

    let (input, y) = delimited(preceded(space0, tag("{")), take_until("}"), tag("}"))(input)?;

    Ok((
        input,
        Statement::If {
            condition: x,
            then: vec![],
            otherwise: None,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Expression;

    #[test]
    fn test1() {
        assert_eq!(
            parse_if("if true { print(\"Hello\") }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::Boolean(true),
                    then: vec![],
                    otherwise: None
                }
            ))
        );

        // TOOD: this should be in then vec
        // Statement::Expr {
        //   expression: Expression::Call(
        //       Expression::Identifier(String::from("print")).boxed(),
        //       vec![Expression::String(String::from("Hello"))],
        //   )
        // }
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_if("if x == 1 { print(\"Hello\") }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::Boolean(true),
                    then: vec![],
                    otherwise: None
                }
            ))
        );
    }
}
