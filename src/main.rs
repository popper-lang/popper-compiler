mod lexer;
mod tree;
mod parser;

fn main() {
    let l = parser::parse_expr(&mut lexer::tokenize("if < a b {+ a b} else {- a b}"));
    println!("{:?}", l);

}
