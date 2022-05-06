mod lexer;
mod tree;
mod parser; 
mod executer;


use std::str::FromStr;

fn main() {
    let mut t = lexer::Token::from_str("for i in 8 { i }");
    if let Ok(p) = &mut t {
        let mut vm = executer::Vm::new();
        let tree = parser::parse_token(p);
        println!("{:?}", vm.eval_expr(match tree {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                return;
            }
        }));
    } else {
        println!("{:?}", t);
    }

}
