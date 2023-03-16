use crate::parser::ast::Expression::{self, Boolean, List, Number, String};

pub fn run_print(input: &Expression) {
    match &input {
        Number(e) => print!("{}", e),
        String(e) => print!("{}", e),
        Boolean(e) => print!("{}", e),
        List(e) => {
            print!("[ ");
            for t in e.iter() {
                run_print(t);
                print!(", ");
            }
            print!("]");
        }
        _ => panic!("Enter proper arguments"),
    }
}
