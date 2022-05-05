mod lexer;
mod tree;
mod parser; 
mod executer;

use std::str::FromStr;

fn main() {
    if let Ok(t) = &mut lexer::Token::from_str("let a = 5") {
        let tree = parser::parse_expr(t);
        if let Ok(p) = tree {
            let mut vm = executer::Vm::new();
            println!("{:?}", vm.eval_expr(p));
        } else if let Err(d) = tree {
            println!("error: {:?}", d);
        }
        
    } else if let Err(e) = lexer::Token::from_str("if < 5 8 {+ 5 3} else {- 5 3}") {
        println!("{}", e);
    }

}
