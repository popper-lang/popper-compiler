mod lexer;
mod tree;
mod parser;
mod executer;

fn main() {
    let l = parser::parse_expr(&mut lexer::tokenize("if < a b {+ a b} else {- a b}"));
    println!("{:?}", l);

}
