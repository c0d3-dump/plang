use crate::parser::ast::Expression::{self, Boolean, Dict, List, Number, String};

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
        Dict(e) => {
            print!("{{ ");
            for (t1, t2) in e.iter() {
                run_print(t1);
                print!(" : ");
                run_print(t2);
                print!(", ");
            }
            print!("}}");
        }
        _ => panic!("Enter proper arguments"),
    }
}
