
use popper::lexer;
use popper::parser;
use popper::interpreter::Interpreter;

fn main() {
    let mut l = lexer::Lexer::new("if false { 1 }".to_string());
    let t = l.scan_token();
    let mut p = parser::Parser::new(t.clone());
    let mut inter = Interpreter::new();
    println!("{:?}", t);
    let e = p.parse();
    println!("{:#?}", e);
    e.statements.into_iter().for_each(|e| {
        println!("{:#?}", e.accept(&mut inter));
    });

}