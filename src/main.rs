
use popper::lexer;
use popper::parser;

fn main() {
    let mut l = lexer::Lexer::new("const a = 8".to_string());
    let mut t: lexer::Token = l.read_token();
    let mut vt: Vec<lexer::Token> = vec![t.clone()];
    while t.clone() != lexer::Token::EOF {
        t = l.read_token();
        vt.push(t.clone())
    }
    println!("{:?}", vt);
    let mut p = parser::Parser::new(vt);
    let mut p = p.parse();
    println!("{:#?}", p);
    
    
}