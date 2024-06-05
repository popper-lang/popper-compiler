
use ariadne::Source;
use thiserror::Error;

use crate::{ColorConfig, Error};
use popper_ast::Span;


#[derive(Error, Debug)]
#[error("not allowed")]
pub struct NotAllowed {
    span: Span,
    outside: String,
    datatype: String,
    data: String,
}

impl NotAllowed {
    pub fn new(span: Span, outside: &str, data: &str, datatype: &str) -> Self {
        Self {
            span,
            outside: outside.to_string(),
            data: data.to_string(),
            datatype: datatype.to_string(),
        }
    }
}

impl Error for NotAllowed {
    fn report(&self, _color: ColorConfig, source: &str, file: &str) {
        let msg = if self.outside.is_empty() {
            format!("The {} `{}` is not allowed here", self.datatype, self.data)
        } else {
            format!(
                "The {} `{}` is not allowed outside `{}`",
                self.datatype, self.data, self.outside
            )
        };
        let mut report = ariadne::Report::build(
            ariadne::ReportKind::Error,
            file,
            self.span.find_line(source),
        );

        report = report
            .with_code(24)
            .with_message(msg.clone())
            .with_label(
                ariadne::Label::new((file, self.span.into()))
                    .with_message(msg),
            );

        report.finish().print((file, Source::from(source))).unwrap();
    }
}
