
use lalrpop_util::lalrpop_mod;
use error::generate_color;
use popper_parser::error::ParserError;
use popper_parser::error::ParserErrorType;
use popper_common::error;
use ast::Span;
use popper_parser::Source;
use popper_common::error::Error;


lalrpop_mod!(pub popper);

fn main() {
    let res = popper::FileParser::new().parse(r#"3 + 3;"#);
    if let Ok(e) = res{
        println!("{:#?}", e);

    } else if let Err(e) = res {
        let err: ParserErrorType = e.into();

        let err = ParserError {
            error: err,
            span: Span::new(0, 4),
        };

        err.report(generate_color(), &Source::from(r#"3 + 3"#), "<main>");

    }
}