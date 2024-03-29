use ariadne::Fmt;
use ariadne::Source;
use thiserror::Error;

use crate::{ColorConfig, Error};
use popper_ast::Span;

#[derive(Error, Debug)]
#[error("name not found")]
/// this error is throw when the SSO don't find a variable
pub struct NameNotFound {
    name: (Span, String),
    pub name_more_closed: Option<String>,
}

impl NameNotFound {
    pub fn new(name: (Span, String), name_more_closed: Option<String>) -> Self {
        Self {
            name,
            name_more_closed,
        }
    }
}

impl Error for NameNotFound {
    fn report(&self, color: ColorConfig, source: &str, file: &str) {
        let variable = color.get("variable").expect("variable color not found");

        let mut report = ariadne::Report::build(
            ariadne::ReportKind::Error,
            file,
            self.name.0.find_line(source),
        );

        report = report
            .with_code(22)
            .with_message(format!("Variable `{}` not found", self.name.1))
            .with_label(
                ariadne::Label::new((file, self.name.0.into()))
                    .with_message(format!("`{}` not found", self.name.1.clone().fg(*variable))),
            );

        if let Some(name_more_closed) = &self.name_more_closed {
            report = report.with_note(format!(
                "Did you mean `{}` ?",
                name_more_closed.clone().fg(*variable)
            ));
        }

        report.finish().print((file, Source::from(source))).unwrap();
    }
}
