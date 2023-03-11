use crate::parser::ast::{Expression, Statement};
use crate::parser::tools::{parse_call, parse_identifier, parse_tag};
use crate::parser::tools::{parse_iterator, parse_value};
use crate::token::Token;

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

fn parse_assignment(input: &str) -> IResult<&str, Expression> {
    let (input, (x, y)) = separated_pair(
        parse_identifier,
        parse_tag(Token::ASSIGN),
        alt((parse_value, parse_iterator)),
    )(input)?;

    Ok((input, Expression::Assign(x.boxed(), y.boxed())))
}

fn parse_break(input: &str) -> IResult<&str, Expression> {
    let (input, ..) = parse_tag(Token::BREAK)(input)?;
    Ok((input, Expression::Break))
}

fn parse_return(input: &str) -> IResult<&str, Expression> {
    let (input, ..) = parse_tag(Token::RETURN)(input)?;

    match preceded(multispace1, alt((parse_value, parse_iterator)))(input) {
        Ok((input, x)) => Ok((input, Expression::Return(Some(x.boxed())))),
        Err(_) => Ok((input, Expression::Return(None))),
    }
}

pub fn parse_expr(input: &str) -> IResult<&str, Statement> {
    let (input, x) = alt((parse_assignment, parse_call, parse_break, parse_return))(input)?;
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

    #[test]
    fn test3() {
        assert_eq!(
            parse_expr("break"),
            Ok((
                "",
                Statement::Expr {
                    expression: Expression::Break
                }
            ))
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            parse_expr("return"),
            Ok((
                "",
                Statement::Expr {
                    expression: Expression::Return(None)
                }
            ))
        )
    }

    #[test]
    fn test5() {
        assert_eq!(
            parse_expr("return x"),
            Ok((
                "",
                Statement::Expr {
                    expression: Expression::Return(Some(
                        Expression::Identifier(String::from("x")).boxed()
                    ))
                }
            ))
        )
    }
}
