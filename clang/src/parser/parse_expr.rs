use crate::parser::ast::{Expression, Statement};
use crate::parser::tools::{parse_call, parse_identifier, parse_tag};
use crate::parser::tools::{parse_iterator, parse_value};
use crate::token::Token;

use nom::branch::alt;
use nom::sequence::separated_pair;
use nom::IResult;

fn parse_assignment(input: &str) -> IResult<&str, Expression> {
    let (input, (x, y)) = separated_pair(
        parse_identifier,
        parse_tag(Token::ASSIGN),
        alt((parse_value, parse_iterator)),
    )(input)?;

    Ok((input, Expression::Assign(x.boxed(), y.boxed())))
}

pub fn parse_expr(input: &str) -> IResult<&str, Statement> {
    let (input, x) = alt((parse_assignment, parse_call))(input)?;
    Ok((input, Statement::Expr { expression: x }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Expression, Op};

    #[test]
    fn test1() {
        assert_eq!(
            parse_expr("x = x + 1"),
            Ok((
                "",
                Statement::Expr {
                    expression: Expression::Assign(
                        Expression::Identifier(String::from("x")).boxed(),
                        Expression::Infix(
                            Expression::Identifier(String::from("x")).boxed(),
                            Op::Add,
                            Expression::Number(1.0).boxed()
                        )
                        .boxed()
                    )
                }
            ))
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse_expr("main()"),
            Ok((
                "",
                Statement::Expr {
                    expression: Expression::Call(
                        Expression::Identifier(String::from("main")).boxed(),
                        vec![]
                    )
                }
            ))
        )
    }
}
