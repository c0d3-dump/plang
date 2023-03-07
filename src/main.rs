use std::env;
use std::fs;

// use logos::Logos;
// use token::Token;

mod nom_parser;
// mod token;
fn main() {
    let args = env::args().nth(1);

    let file = if let Some(f) = args {
        f
    } else {
        panic!("Provide proper args!");
    };

    let maybe_content = fs::read_to_string(file);
    let content = if maybe_content.is_ok() {
        maybe_content.unwrap()
    } else {
        panic!("File not found!");
    };

    // let lex = Token::lexer(&content);
    // lex.clone().for_each(|f| println!("{:?}", f));
    // let out = parse(lex.collect());
    // println!("{:#?}", out);
}
