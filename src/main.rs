
mod expr;
mod vm;
mod errors;
mod ast;
mod value;
mod std_t;
use lalrpop_util::lalrpop_mod;
use std::fs;
use crate::vm::Evaluateur;

lalrpop_mod!(pub popper); // synthesized by LALRPOP

fn main() {

    let contents = fs::read_to_string("/Users/antoine/Documents/popper/src/test.txt")
        .expect("Something went wrong reading the file");
    let exprs = popper::ExprsParser::new().parse(&contents);
    match exprs {
        Ok(exprs) => {
            let mut vm = vm::Vm::new();

            let value = exprs.eval(&mut vm);
            match value {
                Ok(value) => println!("{:?}", value),
                Err(err) => {
                    println!("erreur: {:?}", err);
                }
            };
        }
        Err(e) => {
            println!("erreur: {:?}", e);
        }
    }
}
