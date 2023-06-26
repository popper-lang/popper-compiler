
use lalrpop_util::lalrpop_mod;
use error::generate_color;
use popper_parser::error::ParserError;
use popper_parser::error::ParserErrorType;
use popper_common::error;
use popper_ast::Span;
use popper_parser::Source;
use popper_common::error::Error;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use popper_ast::*;

lalrpop_mod!(pub popper);

fn main() {
    let body = r#"
        while true {
            1 + 2;
        }
        "#;
    let res = popper::FileParser::new().parse(body);
    if let Ok(e) = res{
        println!("{:#}", serde_json::to_string(&e).unwrap());

    } else if let Err(e) = res {
        panic!("{:#?}", e)
    }
}