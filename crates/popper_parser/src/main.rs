
use lalrpop_util::lalrpop_mod;
use ast::Statement;
use ast::Expression;
use error::generate_color;
use popper_parser::error::ParserError;
use popper_parser::error::ParserErrorType;
use popper_common::error;
use ast::Span;
use popper_parser::Source;
use popper_common::error::Error;


lalrpop_mod!(pub popper);

fn main() {
    let res = popper::FileParser::new().parse("while { 1 + 1 }");
    if let Ok(e) = res{
        println!("{:#?}", e);

    } else if let Err(e) = res {
        let err: ParserErrorType = e.into();

        let err = ParserError {
            error: err,
            span: Span::new(0, 10),
        };

        err.report(generate_color(), &Source::from("while { 1 + 1 }"), "<main>");

    }
}