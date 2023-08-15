
use lalrpop_util::lalrpop_mod;

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
        func hello(name: string): int {
            5 + nam;
            4 + name;
        }
        "#;
    let res = popper::FileParser::new().parse(body);
    if let Ok(e) = res{
        println!("{:#}", serde_json::to_string(&e).unwrap());

    } else if let Err(e) = res {
        panic!("{:#?}", e)
    }
}