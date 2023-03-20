use std::collections::HashMap;

use crate::parser::ast::{Expression, Op, Statement};
use crate::stdlib::Std;

fn register_globals(interpreter: &mut Interpreter) {
    interpreter.register_global("print", Std::Print);
    interpreter.register_global("cmd", Std::Cmd);
}

pub fn interpret(input: Vec<Statement>) {
    let mut interpreter = Interpreter::new(HashMap::new(), HashMap::new(), HashMap::new());

    register_globals(&mut interpreter);

    interpreter.run(input);
}

type Block = Vec<Statement>;

#[derive(Debug, PartialEq, Clone)]
pub enum Program {
    Fn {
        params: Vec<Expression>,
        body: Block,
    },
}

#[derive(Debug)]
struct Interpreter {
    globals: HashMap<String, Std>,
    functions: HashMap<String, Program>,
    variables: HashMap<String, Expression>,
}

impl Interpreter {
    fn new(
        globals: HashMap<String, Std>,
        functions: HashMap<String, Program>,
        variables: HashMap<String, Expression>,
    ) -> Self {
        Self {
            globals,
            functions,
            variables,
        }
    }

    fn register_global(&mut self, name: &str, std: Std) {
        self.globals.insert(String::from(name), std);
    }

    fn run(&mut self, ast: Vec<Statement>) -> Option<Expression> {
        let mut ast = ast.into_iter();
        let mut out: Option<Expression> = None;

        while let Some(statement) = ast.next() {
            out = self.run_statement(statement);
            if out != None {
                break;
            }
        }
        out
    }

    fn run_statement(&mut self, input: Statement) -> Option<Expression> {
        match input {
            Statement::Let { name, initial } => {
                let temp = match self.evaluate(initial) {
                    Some(t) => t,
                    None => panic!("cannot use let without value"),
                };

                self.variables.insert(
                    match name {
                        Expression::Identifier(t) => t,
                        _ => panic!("Enter proper identifier"),
                    },
                    temp,
                );
                None
            }

            Statement::Fn { name, params, body } => {
                self.functions.insert(
                    match name {
                        Expression::Identifier(t) => t,
                        _ => panic!("Enter proper function name"),
                    },
                    Program::Fn { params, body },
                );
                None
            }

            Statement::If {
                condition,
                then,
                otherwise,
            } => match self.evaluate(condition) {
                Some(cond) => match cond {
                    Expression::Boolean(t) => {
                        if t {
                            self.run(then)
                        } else {
                            match otherwise {
                                Some(o) => self.run(o),
                                None => None,
                            }
                        }
                    }
                    _ => panic!("must be true or false conditional value"),
                },
                None => panic!("condition does not return value"),
            },

            Statement::Loop {
                iterable,
                value,
                then,
            } => match iterable {
                Some(Expression::Identifier(t)) => {
                    let mut i = Interpreter::new(
                        self.globals.clone(),
                        self.functions.clone(),
                        self.variables.clone(),
                    );

                    match value {
                        Some(ts) => match self.evaluate(ts)? {
                            Expression::List(val) => {
                                let mut temp: Option<Expression> = None;

                                for n in val {
                                    i.variables.insert(t.to_string(), n);
                                    let l = i.run(then.clone());

                                    match l {
                                        Some(y) => {
                                            temp = Some(y);
                                            break;
                                        }
                                        _ => {}
                                    }
                                }

                                let keys: Vec<String> =
                                    i.variables.keys().map(|f| f.to_string()).collect();

                                for k in keys {
                                    if !self.variables.contains_key(&k) {
                                        i.variables.remove(&k);
                                    }
                                }

                                self.variables.extend(i.variables);

                                match &temp {
                                    Some(Expression::Break) => None,
                                    _ => temp,
                                }
                            }
                            _ => panic!("insert proper iterator"),
                        },
                        None => panic!("value cannot be None"),
                    }
                }
                None => {
                    let mut i = Interpreter::new(
                        self.globals.clone(),
                        self.functions.clone(),
                        self.variables.clone(),
                    );

                    let temp = loop {
                        let l = i.run(then.clone());

                        match l {
                            Some(y) => break y,
                            _ => {}
                        }
                    };

                    let keys: Vec<String> = i.variables.keys().map(|f| f.to_string()).collect();

                    for k in keys {
                        if !self.variables.contains_key(&k) {
                            i.variables.remove(&k);
                        }
                    }

                    self.variables.extend(i.variables);

                    match &temp {
                        Expression::Break => None,
                        _ => Some(temp),
                    }
                }

                _ => panic!(),
            },

            Statement::Return { value } => match value {
                Some(t) => self.evaluate(t),
                None => None,
            },

            Statement::Break => Some(Expression::Break),

            Statement::Expr { expression } => {
                self.evaluate(expression);
                None
            }
        }
    }

    fn call(&mut self, input: Program, params: Vec<Expression>) -> Option<Expression> {
        let (p, body) = match input {
            Program::Fn { params, body } => (params, body),
        };

        let a = params.len();
        let b = p.len();
        if a != b {
            panic!("provide necessary arguments");
        }

        for i in 0..a {
            self.variables.insert(
                match p.get(i).unwrap() {
                    Expression::Identifier(l) => l.to_string(),
                    _ => panic!(),
                },
                params.get(i)?.clone(),
            );
        }

        self.run(body)
    }

    fn evaluate(&mut self, input: Expression) -> Option<Expression> {
        match input {
            Expression::Number(t) => Some(Expression::Number(t)),
            Expression::String(t) => Some(Expression::String(t)),
            Expression::Boolean(t) => Some(Expression::Boolean(t)),
            Expression::Identifier(t) => {
                if self.variables.contains_key(&t) {
                    self.evaluate(self.variables.get(&t).unwrap().clone())
                } else {
                    panic!("Wrong identifier {:?}", t)
                }
            }
            Expression::Infix(left, op, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                Some(match (left, op, right) {
                    (Expression::Number(l), Op::Add, Expression::Number(r)) => {
                        Expression::Number(l + r)
                    }
                    (Expression::Number(l), Op::Multiply, Expression::Number(r)) => {
                        Expression::Number(l * r)
                    }
                    (Expression::Number(l), Op::Divide, Expression::Number(r)) => {
                        Expression::Number(l / r)
                    }
                    (Expression::Number(l), Op::Subtract, Expression::Number(r)) => {
                        Expression::Number(l - r)
                    }
                    (Expression::Number(l), Op::Equals, Expression::Number(r)) => {
                        Expression::Boolean(l == r)
                    }
                    (Expression::Number(l), Op::NotEquals, Expression::Number(r)) => {
                        Expression::Boolean(l != r)
                    }
                    (Expression::Number(l), Op::GreaterThan, Expression::Number(r)) => {
                        Expression::Boolean(l > r)
                    }
                    (Expression::Number(l), Op::LessThan, Expression::Number(r)) => {
                        Expression::Boolean(l < r)
                    }
                    (Expression::Number(l), Op::GreaterThanOrEquals, Expression::Number(r)) => {
                        Expression::Boolean(l >= r)
                    }
                    (Expression::Number(l), Op::LessThanOrEquals, Expression::Number(r)) => {
                        Expression::Boolean(l <= r)
                    }
                    (Expression::Boolean(l), Op::And, Expression::Boolean(r)) => {
                        Expression::Boolean(l && r)
                    }
                    (Expression::Boolean(l), Op::Or, Expression::Boolean(r)) => {
                        Expression::Boolean(l || r)
                    }
                    (Expression::String(l), Op::Equals, Expression::String(r)) => {
                        Expression::Boolean(l.eq(&r))
                    }
                    _ => todo!(),
                })
            }
            Expression::List(t) => {
                let mut values: Vec<Expression> = Vec::new();

                for item in t.into_iter() {
                    values.push(self.evaluate(item)?);
                }

                Some(Expression::List(values))
            }
            Expression::Assign(t, value) => {
                let value = self.evaluate(*value)?;
                let i = match *t {
                    Expression::Identifier(i) => i,
                    _ => panic!("expected identifier"),
                };

                if self.variables.contains_key(&i) {
                    self.variables.insert(i, value);
                }
                None
            }
            Expression::Call(name, params) => match *name {
                Expression::Identifier(t) => {
                    let params = params
                        .into_iter()
                        .map(|t| self.evaluate(t).unwrap())
                        .collect();

                    if self.globals.contains_key(&t) {
                        let x = self.globals.get(&t).unwrap();

                        x.run(params);
                        None
                    } else if self.functions.contains_key(&t) {
                        let x = self.functions.get(&t).unwrap();
                        let mut i = Interpreter::new(
                            self.globals.clone(),
                            self.functions.clone(),
                            HashMap::new(),
                        );

                        i.call(x.clone(), params)
                    } else {
                        panic!("Enter proper function name")
                    }
                }
                _ => panic!("Enter proper function name"),
            },
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![
            Statement::Expr {
                expression: Expression::Call(
                    Expression::Identifier(String::from("print")).boxed(),
                    vec![
                        Expression::String(String::from("Hello \n")),
                        Expression::List(vec![
                            Expression::Number(1.0),
                            Expression::Number(2.0),
                            Expression::Number(3.0),
                        ]),
                        Expression::String(String::from("\n")),
                    ],
                ),
            },
            Statement::Expr {
                expression: Expression::Call(
                    Expression::Identifier(String::from("print")).boxed(),
                    vec![Expression::String(String::from("world \n"))],
                ),
            },
        ];

        interpret(input);
    }
}
