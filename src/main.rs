// use std::env;
use std::fs;

// use clang::interpret;
// use clang::parse;

use args::PcliArgs;
use clap::Parser;
use command::list_command;

use crate::command::RunCommand;

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

    let maybe_list_content = fs::read_to_string("examples/applist.pla");

    let list_content = match maybe_list_content {
        Ok(t) => t,
        Err(_) => panic!("something went wrong"),
    };

    let content = list_content.split_terminator(',').collect::<Vec<&str>>();

    let args = PcliArgs::parse();

    match args.cmd {
        args::Cmd::Run(RunCommand { name }) => {
            if content.contains(&name.as_ref()) {
                println!("working");
            } else {
                println!("{name} app does not exists!");
            }
        }
        args::Cmd::List => list_command(content),
        args::Cmd::Install => todo!(),
        args::Cmd::Search => todo!(),
        args::Cmd::New => todo!(),
    }
}
