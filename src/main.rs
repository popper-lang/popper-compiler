mod lexer;
mod tree;
mod parser; 
mod executer;
use std::io::Read;

use std::fs::File;

fn main() {
    println!("{:?}", execute_file("/Users/antoine/Documents/tlang/src/test.txt"));

}

fn execute_file(path: &str) -> Result<executer::Value, std::io::Error> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut t = lexer::Token::from_str(&contents);
    if let Ok(p) = &mut t {
        let mut vm = executer::Vm::new();
        let mut parser = parser::Parser::new();
        let tree = parser.parse_token(p);
        parser.block.reverse();
        println!("{:?}", parser.block);
        println!("{:?}", parser.is_block);
        println!("{:?}", parser.block.len());
        let value = vm.eval_expr(match tree {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        });

        return match value {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        };
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "error when parsing"));
    }
}
