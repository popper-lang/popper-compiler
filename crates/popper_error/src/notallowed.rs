use ariadne::Fmt;
use ariadne::Source;
use thiserror::Error;

use crate::{ColorConfig, Error};
use popper_ast::Span;

#[derive(Error, Debug)]
#[error("not allowed")]
pub struct NotAllowed {
    span: Span,
    outside: String,
    keyword: String,
}

impl NotAllowed {
    pub fn new(span: Span, outside: &str, keyword: &str) -> Self {
        Self {
            span,
            outside: outside.to_string(),
            keyword: keyword.to_string(),
        }
    }
}

impl Error for NotAllowed {
    fn report(&self, color: ColorConfig, source: &str, file: &str) {
        let keyword = color.get("keyword").expect("keyword color not found");
        let mut report = ariadne::Report::build(
            ariadne::ReportKind::Error,
            file,
            self.span.find_line(source),
        );

        report = report
            .with_code(24)
            .with_message(format!(
                "The keyword `{}` is not allowed outside `{}`",
                self.keyword, self.outside
            ))
            .with_label(
                ariadne::Label::new((file, self.span.into()))
                    .with_message(format!(
                        "The keyword `{}` is not allowed outside `{}`",
                        self.keyword.clone().fg(*keyword),
                        self.outside.clone().fg(*keyword)
                    )),
            );

        report.finish().print((file, Source::from(source))).unwrap();
    }
}
