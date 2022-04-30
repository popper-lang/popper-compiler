mod lexer;
mod tree;
mod parser;

fn main() {
    let l = parser::parse_expr(&mut lexer::tokenize("if 1 then 3 else 5 + 8"));
    if let Ok(t) = l {
        println!("{:?}", t);
    } else if let Err(e) = l {
        println!("error: {}", e);
    }
}
