mod api;
mod ast;
mod errors;
mod expr;
mod std_t;
mod value;
mod vm;
use crate::vm::Evaluateur;
use lalrpop_util::lalrpop_mod;
use std::fs;

lalrpop_mod!(pub popper); // synthesized by LALRPOP

fn main() {
    let contents = fs::read_to_string("/Users/antoine/Documents/popper/src/test.txt")
        .expect("Something went wrong reading the file");
    let exprs = popper::ExprsParser::new().parse(&contents);
    match exprs {
        Ok(exprs) => {
            /*let mut vm = vm::Vm::new();

            let value = exprs.eval(&mut vm);
            match value {
                Ok(value) => println!("{:?}", value),
                Err(err) => {
                    println!("erreur: {:?}", err);
                }
            };*/
            println!("{:#?}", exprs);
        }
        Err(e) => {
            println!("erreur: {:?}", e);
        }
    }
}
