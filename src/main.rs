// use std::env;
// use std::fs;

// use clang::interpret;
// use clang::parse;

use args::PcliArgs;
use clap::Parser;

mod args;
mod command;

fn main() {
    // let args = env::args().nth(1);

    // let file = if let Some(f) = args {
    //     f
    // } else {
    //     panic!("Provide proper args!");
    // };

    // let maybe_content = fs::read_to_string(file);
    // let content = if maybe_content.is_ok() {
    //     maybe_content.unwrap()
    // } else {
    //     panic!("File not found!");
    // };

    // match parse(&content) {
    //     Ok((_, output)) => {
    //         interpret(output);
    //     }
    //     Err(e) => panic!("{:#?}", e),
    // }

    let args = PcliArgs::parse();

    println!("{:?}", args);
}
