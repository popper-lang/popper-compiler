mod executer;
mod tree;
use std::fs;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub tlang); // synthesized by LALRPOP

fn main() {
    let contents = fs::read_to_string("/Users/antoine/Documents/tlang/src/test.txt")
        .expect("Something went wrong reading the file");
    let exprs = tlang::ExprsParser::new().parse(&contents);
    match exprs {
        Ok(exprs) => {
            let mut vm = executer::Vm::new();
            println!("{:?}", vm.eval_expr(exprs));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
