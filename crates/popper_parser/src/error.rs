use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use popper_common::error::{ColorConfig, Error, source_to_string};
use ariadne::{Report, Source, ReportKind, Label};
use ariadne::Fmt;


#[derive(Clone)]
pub struct ParserErrorType<'a>(pub ParseError<usize, Token<'a>, &'static str>);

#[derive(Clone)]
pub struct ParserError<'a> {
    pub error: ParserErrorType<'a>,
    pub span: ast::Span,
}


impl<'a> From<ParseError<usize, Token<'a>, &'static str>> for ParserErrorType<'a> {
    fn from(val: ParseError<usize, Token<'a>, &'static str>) -> Self {
        ParserErrorType(val)
    }
}

impl<'a> std::fmt::Display for ParserErrorType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            ParseError::InvalidToken { location } => {
                write!(f, "Invalid token at {}", location)
            }
            ParseError::UnrecognizedEof { location, expected } => {
                write!(f, "Unexpected end of file at {}, expected {:?}", location, expected)
            }
            ParseError::UnrecognizedToken { token: (start, token, end), expected } => {
                write!(f, "Unexpected token {:?} at {}:{}, expected {:?}", token, start, end, expected)
            }
            ParseError::ExtraToken { token: (start, token, end) } => {
                write!(f, "Extra token {:?} at {}:{}", token, start, end)
            }
            ParseError::User { error } => {
                write!(f, "User error: {}", error)
            }
        }
    }
}

impl<'a> Error for ParserError<'a> {
    fn report(&self, color: ColorConfig, source: &Source, file: &str) {
        let keyword_color = color.get("keyword").expect("keyword color not found");

        let mut report = Report::build(ReportKind::Error,
                                                file,
                                                self.span.find_line(
                                                    source_to_string(source).as_str()
                                                )
        );

        match self.clone().error.clone().0 {
            ParseError::InvalidToken { location } => {
                report = report.with_code(1)
                    .with_message("Invalid token".to_string())
                    .with_label(
                        Label::new((file, self.span.into()))
                            .with_message(
                                format!("invalid token `{}`",
                                        self.span.extract_from_str(
                                            source_to_string(source).as_str()
                                        ).fg(
                                            *keyword_color
                                        )
                                )
                            )
                    );
            }
            ParseError::UnrecognizedEof { location, expected } => {
                report = report.with_code(1)
                    .with_message("Unexpected end of file".to_string())
                    .with_label(
                        Label::new((file, self.span.into()))
                            .with_message(
                                format!("expected `{}`",
                                        expected.join(", ")
                                )
                            )
                    );
            }
            ParseError::UnrecognizedToken { token: (start, token, end), expected } => {
                report = report.with_code(1)
                    .with_message("Unexpected token".to_string())
                    .with_label(
                        Label::new((file, start..end))
                            .with_message(
                                format!("expected `{}`",
                                        expected.join(", ")
                                )
                            )
                    );
            }
            ParseError::ExtraToken { token: (start, token, end) } => {
                report = report.with_code(1)
                    .with_message("Extra token".to_string())
                    .with_label(
                        Label::new((file, start..end))
                            .with_message(
                                format!("unexpected token `{}`",
                                        self.span.extract_from_str(
                                            source_to_string(source).as_str()
                                        ).fg(
                                            *keyword_color
                                        )
                                )
                            )
                    );
            }
            ParseError::User { error } => {
                report = report.with_code(1)
                    .with_message("User error".to_string())
                    .with_label(
                        Label::new((file, self.span.into()))
                            .with_message(
                                format!("user error `{}`",
                                        self.span.extract_from_str(
                                            source_to_string(source).as_str()
                                        ).fg(
                                            *keyword_color
                                        )
                                )
                            )
                    );
            }
        }

        report
            .finish()
            .print((file, Source::from(source_to_string(source))))
            .unwrap();





    }
}