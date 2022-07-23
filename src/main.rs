
use popper::lexer;
use popper::parser;

fn main() {
    let mut l = lexer::Lexer::new("6 + 8 + 9".to_string());
    let mut t: lexer::Token = l.read_token();
    let mut vt: Vec<lexer::Token> = vec![t.clone()];
    while t.clone() != lexer::Token::EOF {
        t = l.read_token();
        vt.push(t.clone())
    }
    println!("{:?}", vt);
    let mut p = parser::Parser::new(vt);
    let p = p.parse();
    println!("{:#?}", p);
    
    
}