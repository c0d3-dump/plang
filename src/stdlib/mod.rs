mod function;

pub use function::run_print;

use crate::parser::ast::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum Std {
    Print,
}

impl Std {
    pub fn run(&self, input: Vec<Expression>) {
        match self {
            Self::Print => {
                input.iter().for_each(|i| run_print(i));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Expression;

    #[test]
    fn test1() {
        Std::Print.run(vec![
            Expression::String(String::from("Hello ")),
            Expression::List(vec![
                Expression::Number(1.0),
                Expression::Number(2.0),
                Expression::Number(3.0),
            ]),
            Expression::String(String::from("\n")),
        ]);
    }
}
