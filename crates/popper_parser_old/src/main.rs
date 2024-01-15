
use lalrpop_util::lalrpop_mod;

use popper_common::error;
use popper_ast::Span;
use popper_parser::Source;
use popper_common::error::Error;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use popper_ast::*;
use popper_parser::error::ParserErrorType;
use popper_parser::error::ParserError;
use popper_common::error::generate_color;


lalrpop_mod!(pub popper);

fn main() {
    let body = r#"
        func add(a: int, b: int): unit {
            return a + b;
        }

        add(2, 4, 1);
        "#;
    let res = popper::FileParser::new().parse(body);
    if let Ok(e) = res {
        println!("{:#}", serde_json::to_string(&e).unwrap());

    } else if let Err(e) = res {
        let err_type: ParserErrorType = e.into();
        let err = ParserError {
            error: err_type,
            span: Span::new(0, body.len()),
        };
        err.report(
            generate_color(),
            body,
            "<popper_parser_old/src/main.rs 19:25>"
        );
    }
}