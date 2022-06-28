
use pest::Parser;
use popper::parser::ExprParser;
use popper::parser::build_ast;


fn main() {

    let mut result = ExprParser::parse(popper::parser::Rule::statement,"[4, 6, 2] to int");
    match result {
        Ok(ref mut e) => println!("{:#?}", build_ast(e.into_iter().next().unwrap())),
        Err(e) => println!("{}", e)
    }
}