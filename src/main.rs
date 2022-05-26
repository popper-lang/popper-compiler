mod executer;
mod tree;
mod errors;
use std::fs;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub tlang); // synthesized by LALRPOP

fn main() {
    let contents = fs::read_to_string("/Users/antoine/Documents/tlang/src/test.txt")
        .expect("Something went wrong reading the file");
    let exprs = tlang::ExprsParser::new().parse(&contents);
    match exprs {
        Ok(exprs) => {
            let mut vm = executer::Vm::new();
            let value = vm.eval_expr(exprs);
            match value {
                Ok(value) => println!("{:?}", value),
                Err(err) => {
                    println!("erreur: {:?}", err);
                },
            };
        }
        Err(e) => {
            println!("erreur: {:?}", e);
        }
    }
}
