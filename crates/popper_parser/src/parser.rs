use lalrpop_util::{lalrpop_mod, ParseError};
use lalrpop_util::lexer::Token;

use crate::error::{ParserError, ParserErrorType};
use popper_common::error::generate_color;
use popper_common::error::Error;
use popper_ast::Statement;




lalrpop_mod!(pub popper); // synthesized by LALRPOP


pub fn parse<'a>(input: &'a str, file: &'a str) -> Option<Vec<Statement>> {
    let res = popper::FileParser::new().parse(input);

    match res {
        Ok(ast) => Some(ast),
        Err(e) => {
            let err_type: ParserErrorType = e.into();
            let err = ParserError {
                error: err_type,
                span: popper_ast::Span::new(0, input.len()),
            };
            err.report(
                generate_color(),
                input,
                file
            );
            None
        }
    }
}