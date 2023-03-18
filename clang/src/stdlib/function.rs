use std::process::Command;
use std::string::String;

use crate::parser::ast::Expression;

pub fn run_print(input: &Expression) {
    match &input {
        Expression::Number(e) => print!("{}", e),
        Expression::String(e) => print!("{}", e),
        Expression::Boolean(e) => print!("{}", e),
        Expression::List(e) => {
            let len = e.len();
            print!("[ ");
            for (i, t) in e.iter().enumerate() {
                run_print(t);
                if i + 1 < len {
                    print!(", ");
                }
            }
            print!(" ]");
        }
        _ => panic!("Enter proper arguments"),
    }
}

pub fn get_print(input: Expression) -> String {
    match input {
        Expression::Number(e) => e.to_string(),
        Expression::String(e) => e,
        Expression::Boolean(e) => e.to_string(),
        _ => panic!("Enter proper arguments"),
    }
}

pub fn run_cmd(input: Vec<Expression>) {
    let mut t = input.into_iter().map(|i| get_print(i));

    let cmd = Command::new(t.next().unwrap()).args(t).spawn();
    match cmd {
        Ok(mut c) => {
            c.wait().expect("loading...");
        }
        Err(_e) => println!("command failed!"),
    }
}

// TODO
// pub fn run_http(input: &Expression) {}
