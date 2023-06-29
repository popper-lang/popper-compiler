
use lalrpop_util::lalrpop_mod;





lalrpop_mod!(pub popper); // synthesized by LALRPOP

/*
pub fn parse(input: &str, file: &str) -> Vec<Statement> {
    popper::FileParser::new().parse(input).unwrap_or_else(|e| {
        let err_type: ParserErrorType = e.into();
        let err = ParserError {
            error: err_type,
            span: ast::Span::new(0, 5),
        };
        err.report(
            generate_color(),
            &Source::from(input),
            file
        );
    })
}*/